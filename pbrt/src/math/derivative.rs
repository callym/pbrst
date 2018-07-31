use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct DerivativeTerm {
    pub kc: Float,
    pub kx: Float,
    pub ky: Float,
    pub kz: Float,
}

impl DerivativeTerm {
    pub fn new(kc: Float, kx: Float, ky: Float, kz: Float) -> Self {
        Self { kc, kx, ky, kz }
    }

    pub fn eval(&self, p: Point3f) -> Float {
        let Self { kc, kx, ky, kz } = *self;

        kc + (kx * p.x) + (ky * p.y) + (kz * p.z)
    }
}
