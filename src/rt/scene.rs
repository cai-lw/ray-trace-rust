use ::rt::*;
use super::material::{Material, BlackBody};
use super::surface::{Surface, SurfaceSide};
use super::light::{Light, Dark};

pub trait SceneObject {
    fn surface(&self) -> &Surface;

    fn material(&self) -> &Material;

    fn light(&self) -> &Light;
}

struct NormalObject<S, M> {
    surface: S,
    material: M
}

impl<S, M> SceneObject for NormalObject<S, M> where
    S: Surface,
    M: Material
{
    fn surface(&self) -> &Surface {
        &self.surface
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn light(&self) -> &Light {
        &Dark
    }
}

struct LightObject<S, L> {
    surface: S,
    light: L
}

impl<S, L> SceneObject for LightObject<S, L> where
    S: Surface,
    L: Light
{
    fn surface(&self) -> &Surface {
        &self.surface
    }

    fn material(&self) -> &Material {
        &BlackBody
    }

    fn light(&self) -> &Light {
        &self.light
    }
}

pub trait Scene {
    fn intersect_with_ray(&self, &Ray) -> Option<(&SceneObject, f32, SurfaceSide)>;

    fn ray_trace(&self, ray: &Ray) -> Color {
        use palette::named::WHITE;
        let mut spectrum = Color::from_pixel(&WHITE);
        let mut current_ray: Ray = ray.clone();
        for _ in 0..10 {
            if let Some((object, distance, side)) = self.intersect_with_ray(&current_ray) {
                let point = current_ray.src + current_ray.dir * distance;
                let refl = -current_ray.dir;
                let norm = object.surface().oriented_normal(point, side);
                let (infl, f, p) = object.material().model(point).sample_f_p(refl, norm);
                spectrum = spectrum * (f / p);
                current_ray = Ray { src: point, dir: infl };
            }
        }
        spectrum
    }
}

pub struct SimpleScene {
    objects: Vec<Box<SceneObject>>,
    lights: Vec<Box<SceneObject>>
}

impl SimpleScene {
    pub fn new() -> SimpleScene {
        SimpleScene { objects: vec![], lights: vec![] }
    }

    pub fn push_object<S: Surface + 'static, M: Material + 'static>(&mut self, surface: S, material: M) {
        self.objects.push(Box::new(NormalObject { surface, material }))
    }

    pub fn push_light<S: Surface + 'static, L: Light + 'static>(&mut self, surface: S, light: L) {
        self.lights.push(Box::new(LightObject { surface, light }))
    }
}

impl Scene for SimpleScene {
    fn intersect_with_ray(&self, ray: &Ray) -> Option<(&SceneObject, f32, SurfaceSide)> {
        use noisy_float::prelude::*;
        self.objects.iter()
            .chain(self.lights.iter())
            .filter_map(|object| 
                object.surface().intersect_with_ray(ray)
                    .map(|(distance, side)| (object.as_ref(), distance, side))
            ).min_by_key(|&(_, distance, _)| r32(distance))
    }
}