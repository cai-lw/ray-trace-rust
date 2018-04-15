use super::Ray;
use ::rt::*;
use super::sampling::*;

pub trait Surface {
    fn intersect_with_ray(&self, &Ray) -> Option<(f32, SurfaceSide)>;

    fn normal(&self, Pt3) -> Vec3;

    fn sample_from_point(&self, Pt3) -> (Vec3, f32);

    fn oriented_normal(&self, point: Pt3, side: SurfaceSide) -> Vec3 {
        let normal = self.normal(point);
        match side {
            SurfaceSide::Front => normal,
            SurfaceSide::Back => -normal
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SurfaceSide {
    Front,
    Back
}

impl SurfaceSide {
    pub fn from_float(x: f32) -> SurfaceSide {
        if x > 0.0 {
            SurfaceSide::Front
        }
        else {
            SurfaceSide::Back
        }
    }
}

pub struct Rect {
    corner: Pt3,
    side1: Vec3,
    side2: Vec3,
    _normal: Vec3,
    _area: f32
}

impl Rect {
    pub fn new(corner: Pt3, side1: Vec3, side2: Vec3) -> Rect {
        assert!(side1.dot(side2).abs() < 1e-6 * side1.magnitude() * side2.magnitude());
        let cross = side1.cross(side2);
        Rect {
            corner,
            side1,
            side2,
            _normal: cross.normalize(),
            _area: cross.magnitude()
        }
    }

    fn includes_point(&self, point_on_plane: Pt3) -> bool {
        let point_vec = point_on_plane - self.corner;
        let dot1 = point_vec.dot(self.side1);
        let dot2 = point_vec.dot(self.side2);
        0.0 < dot1 && dot1 < self.side1.magnitude2() && 0.0 < dot2 && dot2 < self.side2.magnitude2()
    }
}

impl Surface for Rect {
    fn intersect_with_ray(&self, ray: &Ray) -> Option<(f32, SurfaceSide)> {
        let src_vec = ray.src - self.corner;
        let src_vec_dot_normal = src_vec.dot(self._normal);
        let dir_distance = -src_vec_dot_normal / ray.dir.dot(self._normal);
        if dir_distance < 0.0 {
            return None;
        }
        let intersection = ray.src + ray.dir * dir_distance;
        if self.includes_point(intersection) {
            Some((dir_distance, SurfaceSide::from_float(src_vec_dot_normal)))
        }
        else { None }
    }

    fn sample_from_point(&self, point: Pt3) -> (Vec3, f32) {
        let (l1, l2) = random::<(f32, f32)>();
        let endpoint = self.corner + self.side1 * l1 + self.side2 * l2;
        let v = (endpoint - point).normalize();
        let p = self._area.recip() * point.distance2(endpoint) / v.dot(self._normal).abs();
        (v, p)
    }

    fn normal(&self, point: Pt3) -> Vec3 {
        self._normal
    }
}

pub struct Sphere {
    center: Pt3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Pt3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Surface for Sphere {
    fn intersect_with_ray(&self, ray: &Ray) -> Option<(f32, SurfaceSide)> {
        let src_vec = ray.src - self.center;
        let b = src_vec.dot(ray.dir);
        let c = src_vec.magnitude2() - self.radius * self.radius;
        let delta = b * b - 4.0 * c;
        if delta < 0.0 { None }
        else {
            if c < 0.0 {
                Some(((-b + delta.sqrt()) * 0.5, SurfaceSide::Back))
            }
            else {
                let x_small = (-b - delta.sqrt()) * 0.5;
                if x_small > 0.0 {
                    Some((x_small, SurfaceSide::Front))
                }
                else { None }
            }
        }
    }

    fn sample_from_point(&self, point: Pt3) -> (Vec3, f32) {
        use cgmath::Quaternion;
        use std::f32::consts::PI;

        let pointing_vec = self.center - point;
        let sin_theta_max = self.radius / pointing_vec.magnitude();
        let cos_theta_min = (1.0 - sin_theta_max * sin_theta_max).sqrt();
        let cos_theta = 1.0 - random::<f32>() / (1.0 - cos_theta_min);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let (sin_phi, cos_phi) = random_sin_cos();
        let v_up = vec3(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
        let rotation = Quaternion::between_vectors(Vec3::unit_z(), pointing_vec.normalize());
        (rotation.rotate_vector(v_up), (2.0 * PI * (1.0 - cos_theta_min)).recip())
    }

    fn normal(&self, point: Pt3) -> Vec3 {
        (point - self.center).normalize()
    }
}