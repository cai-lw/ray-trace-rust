use ::rt::*;
use cgmath::prelude::*;
use cgmath::{Point3, Quaternion};

pub trait Camera {
    fn to_ray(&self, Pt2) -> Ray;
}

pub struct PerspectiveCamera {
    place: Pt3,
    height: f32,
    width: f32,
    rotation: Quaternion<f32>
}

impl PerspectiveCamera {
    fn new(place: Pt3, fovy: f32, aspect: f32, dir: Vec3, up: Vec3) -> PerspectiveCamera {
        PerspectiveCamera {
            place,
            height: fovy.tan() * 0.5,
            width: fovy.tan() * 0.5 * aspect,
            rotation: Quaternion::look_at(dir, up).invert()
        }
    }
}

impl Camera for PerspectiveCamera {
    fn to_ray(&self, point: Pt2) -> Ray {
        Ray {
            src: self.place,
            dir: self.rotation.rotate_vector(vec3((point.x - 0.5) * self.width, (point.y - 0.5) * self.height, 1.0))
        }
    }
}