use ::rt::*;
use rand::{Rand, Rng, XorShiftRng};
use std::cell::RefCell;
use std::ops::DerefMut;
thread_local!(static RNG: RefCell<XorShiftRng> = RefCell::new(XorShiftRng::new_unseeded()));

pub fn random<T: Rand>() -> T {
    RNG.with(|rng| rng.borrow_mut().gen())
}

pub fn with_rng<F, T>(f: F) -> T where
    F: FnOnce(&mut XorShiftRng) -> T {
    RNG.with(|rng| f(rng.borrow_mut().deref_mut()))
}

pub fn random_sin_cos() -> (f32, f32) {
    with_rng(|rng| {
        loop {
            let x = rng.gen::<f32>() - 0.5;
            let y = rng.gen::<f32>() - 0.5;
            let norm2 = x * x + y * y;
            if norm2 <= 0.25 && norm2 > 1e-10 {
                let inv_norm = norm2.sqrt().recip();
                return (y * inv_norm, x * inv_norm);
            }
        }
    })
}

pub fn random_cos_hemisphere(norm: Vec3) -> Vec3 {
    with_rng(|rng| {
        loop {
            let v = rng.gen::<Vec3>() - vec3(0.5, 0.5, 0.5);
            let norm2 = v.magnitude2();
            if norm2 <= 0.25 && norm2 > 1e-10 {
                let inv_norm = norm2.sqrt().recip();
                return (v * inv_norm + norm).normalize();
            }
        }
    })
}