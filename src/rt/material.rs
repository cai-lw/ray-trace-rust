use ::rt::*;
use super::bsdf::{blinn_phong, black_body, BSDF};
use super::texture::{Texture, Uniform};

pub trait Material {
    fn model(&self, Pt3) -> Box<BSDF>;
}

pub struct BlackBody;

impl Material for BlackBody {
    fn model(&self, point: Pt3) -> Box<BSDF> {
        black_body()
    }
}

pub struct BlinnPhongParams<Td, Ts, Tn> {
    pub diff: Td,
    pub spec: Ts,
    pub n: Tn
}

impl<Td, Ts, Tn> Material for BlinnPhongParams<Td, Ts, Tn> where
    Td: Texture<Output=Color>,
    Ts: Texture<Output=Color>,
    Tn: Texture<Output=i32>    
{
    fn model(&self, point: Pt3) -> Box<BSDF> {
        blinn_phong(
            self.diff.at(point),
            self.spec.at(point),
            self.n.at(point)
        )
    }
}

pub fn uniform_material(emit: Color, diff: Color, spec: Color, n: i32)
    -> BlinnPhongParams<Uniform<Color>, Uniform<Color>, Uniform<i32>>
{
    BlinnPhongParams {
        diff: Uniform::new(diff),
        spec: Uniform::new(spec),
        n: Uniform::new(n)
    }
}