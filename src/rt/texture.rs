use ::rt::*;
use std::ops::Deref;

pub trait Texture {
    type Output;

    fn at(&self, Pt3) -> Self::Output;

    fn map<U, F>(self, f: F) -> Map<Self, F> where
        Self: Sized,
        F: FnMut(Self::Output) -> U {
        Map {
            texture: self,
            function: f
        }
    }

    fn zip<T>(self, other: T) -> Zip<Self, T> where
        Self: Sized,
        T: Texture {
        Zip {
            texture1: self,
            texture2: other
        }
    }

    fn with_point(self) -> Zip<Identity, Self> where Self: Sized {
        Identity.zip(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Uniform<T> {
    value: T
}

impl<T> Uniform<T> {
    pub fn new(value: T) -> Uniform<T> {
        Uniform { value }
    }
}

impl<T: Clone> Texture for Uniform<T> {
    type Output = T;

    fn at(&self, point: Pt3) -> T {
        self.value.clone()
    }
}

pub struct Identity;

impl Texture for Identity {
    type Output = Pt3;

    fn at(&self, point: Pt3) -> Pt3 {
        point
    }
}

pub struct Map<T, F> {
    texture: T,
    function: F
}

impl<T, U, F> Texture for Map<T, F> where
    T: Texture,
    F: Fn(T::Output) -> U
{
    type Output = U;

    fn at(&self, point: Pt3) -> U {
        (self.function)(self.texture.at(point))
    }
}

pub struct Zip<T1, T2> {
    texture1: T1,
    texture2: T2
}

impl<T1, T2> Texture for Zip<T1, T2> where
    T1: Texture,
    T2: Texture
{
    type Output = (T1::Output, T2::Output);

    fn at(&self, point: Pt3) -> (T1::Output, T2::Output) {
        (self.texture1.at(point), self.texture2.at(point))
    }
}