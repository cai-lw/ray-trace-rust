pub mod surface;
pub mod texture;
pub mod material;
pub mod bsdf;
pub mod light;
pub mod scene;
pub mod sampling;
pub mod camera;
pub mod film;

use cgmath::*;
use palette::{Rgb, named};

pub type Vec3 = Vector3<f32>;
pub type Vec2 = Vector2<f32>;
pub type Pt3 = Point3<f32>;
pub type Pt2 = Point2<f32>;
pub type Color = Rgb<f32>;

const BLACK: Color = Rgb { red: 0.0, green: 0.0, blue: 0.0 };

#[derive(Clone, Debug)]
pub struct Ray {
    pub src: Pt3,
    pub dir: Vec3
}
