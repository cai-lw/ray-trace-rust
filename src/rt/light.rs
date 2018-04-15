use ::rt::*;
use super::texture::Texture;

pub trait Light {
    fn emission(&self, Pt3, Vec3, Vec3) -> Color;
}

pub struct Dark;

impl Light for Dark {
    fn emission(&self, point: Pt3, dir: Vec3, norm: Vec3) -> Color {
        BLACK
    }
}

pub struct Lambertian<T> {
    color: T
}

impl<T: Texture<Output=Color>> Light for Lambertian<T> {
    fn emission(&self, point: Pt3, dir: Vec3, norm: Vec3) -> Color {
        self.color.at(point)
    }
}