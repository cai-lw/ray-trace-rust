use ::rt::*;
use super::sampling::{random, random_sin_cos, random_cos_hemisphere};
use std::f32::consts::*;

const FRAC_1_2PI: f32 = 1.0 / (PI * 2.0);
const FRAC_1_8PI: f32 = 1.0 / (PI * 8.0);

pub trait BSDF {
    fn f_p(&self, Vec3, Vec3, Vec3) -> (Color, f32);

    fn sample_f_p(&self, Vec3, Vec3) -> (Vec3, Color, f32);

    fn sample_nonzero_f_p(&self, refl: Vec3, norm: Vec3) -> (Vec3, Color, f32) {
        let mut count = 0;
        loop {
            count += 1;
            let (infl, f, p) = self.sample_f_p(refl, norm);
            if !relative_eq!(f, BLACK) {
                return (infl, f / count as f32, p);
            }
        }
    }

    fn add<F>(self, other: F) -> Add<Self, F> where Self: Sized {
        Add { f1: self, f2: other }
    }

    fn weight(&self) -> f32 {
        1.0
    }
}

struct Diffuse {
    color: Color
}

impl Diffuse {
    fn new(color: Color) -> Diffuse {
        Diffuse { color }
    }
}

impl BSDF for Diffuse {
    fn f_p(&self, infl: Vec3, refl: Vec3, norm: Vec3) -> (Color, f32) {
        let p = infl.dot(norm) * FRAC_1_PI;
        (self.color * p, p)
    }

    fn sample_f_p(&self, refl: Vec3, norm: Vec3) -> (Vec3, Color, f32) {
        let infl = random_cos_hemisphere(norm);
        let (f, p) = self.f_p(infl, refl, norm);
        (infl, f, p)
    }
}

struct BlinnPhongSpecular {
    color: Color,
    n: i32
}

impl BlinnPhongSpecular {
    fn new(color: Color, n: i32) -> BlinnPhongSpecular {
        BlinnPhongSpecular { color, n }
    }
}

impl BSDF for BlinnPhongSpecular {
    fn f_p(&self, infl: Vec3, refl: Vec3, norm: Vec3) -> (Color, f32) {
        let mut halfway = (infl + refl).normalize();
        let mut cos_halfway_norm = halfway.dot(norm);
        if cos_halfway_norm < 0.0 {
            halfway = -halfway;
            cos_halfway_norm = -cos_halfway_norm;
        }
        let cos_halfway_norm_pow_n = cos_halfway_norm.powi(self.n);
        let p_halfway = (self.n + 1) as f32 * FRAC_1_2PI * cos_halfway_norm_pow_n;
        let p = p_halfway / (4.0 * halfway.dot(refl).abs());
        let f = if infl.dot(norm) < 0.0 { BLACK } else {
            self.color * ((self.n + 4) as f32 * FRAC_1_8PI * cos_halfway_norm_pow_n)
        };
        (f, p)
    }

    fn sample_f_p(&self, refl: Vec3, norm: Vec3) -> (Vec3, Color, f32) {
        use cgmath::Quaternion;

        let cos_theta = random::<f32>().powf(((self.n + 1) as f32).recip());
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cos_theta_pow_n = cos_theta.powi(self.n);
        let (sin_phi, cos_phi) = random_sin_cos();
        let halfway_z = vec3(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
        let p_halfway = (self.n + 1) as f32 * FRAC_1_2PI * cos_theta_pow_n;
        let rotation = Quaternion::between_vectors(Vec3::unit_z(), norm);
        let halfway = rotation.rotate_vector(halfway_z);
        let cos_halfway_refl = halfway.dot(refl);
        let infl = halfway * (cos_halfway_refl * 2.0) - refl;
        let p = p_halfway / (4.0 * cos_halfway_refl.abs());
        let f = if infl.dot(norm) < 0.0 { BLACK } else {
            self.color * ((self.n + 4) as f32 * FRAC_1_8PI * cos_theta_pow_n)
        
        };
        (infl, f, p)
    }
}

pub struct Add<F1, F2> {
    f1: F1,
    f2: F2
}

impl<F1, F2> BSDF for Add<F1, F2> where F1: BSDF, F2: BSDF {
    fn f_p(&self, infl: Vec3, refl: Vec3, norm: Vec3) -> (Color, f32) {
        let w1 = self.f1.weight();
        let w2 = self.f2.weight();
        let (f1, p1) = self.f1.f_p(infl, refl, norm);
        let (f2, p2) = self.f2.f_p(infl, refl, norm);
        (f1 + f2, (p1 * w1 + p2 * w2) / (w1 + w2))
    }

    fn sample_f_p(&self, refl: Vec3, norm: Vec3) -> (Vec3, Color, f32) {
        let w1 = self.f1.weight();
        let w2 = self.f2.weight();
        if random::<f32>() < w1 / (w1 + w2) {
            let (infl, f1, p1) = self.f1.sample_f_p(refl, norm);
            let (f2, p2) = self.f2.f_p(infl, refl, norm);
            (infl, f1 + f2, (p1 * w1 + p2 * w2) / (w1 + w2))
        }
        else {
            let (infl, f2, p2) = self.f2.sample_f_p(refl, norm);
            let (f1, p1) = self.f1.f_p(infl, refl, norm);
            (infl, f1 + f2, (p1 * w1 + p2 * w2) / (w1 + w2))
        }
    }

    fn weight(&self) -> f32 {
        let w1 = self.f1.weight();
        let w2 = self.f2.weight();
        w1 + w2
    }
}

pub fn blinn_phong(diff: Color, spec: Color, n: i32) -> Box<BSDF> {
    Box::new(Diffuse::new(diff).add(BlinnPhongSpecular::new(spec, n)))
}

pub fn black_body() -> Box<BSDF> {
    Box::new(Diffuse::new(BLACK))
}