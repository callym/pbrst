use std::fmt::Debug;
use cg::{ Point2, Vector3 };
use prelude::*;
use math::*;
use interaction::Sample;

bitflags! {
    pub struct BxdfType: u8 {
        const Reflection    = 1 << 0;
        const Transmission  = 1 << 1;
        const Diffuse       = 1 << 2;
        const Glossy        = 1 << 3;
        const Specular      = 1 << 4;
    }
}

pub trait Bsdf: Debug {
    fn f(&self, wo: Ray, wi: Ray) -> Spectrum;

    fn sample_f(&self, wo: Ray, u: Point2f, BxdfType) -> Sample;
}
