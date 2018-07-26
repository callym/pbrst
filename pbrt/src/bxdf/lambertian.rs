#[cfg(not(feature = "double"))]
use std::f32::consts;

#[cfg(feature = "double")]
use std::f64::consts;

use prelude::*;
use super::*;
use super::utils::*;

#[derive(Debug)]
pub struct LambertianReflection {
    r: Spectrum,
}

impl LambertianReflection {
    pub fn new(r: Spectrum) -> Self {
        Self {
            r,
        }
    }
}

impl Bxdf for LambertianReflection {
    fn ty(&self) -> BxdfType {
        BxdfType::Reflection | BxdfType::Diffuse
    }

    fn f(&self, _: Vector3f, _: Vector3f) -> Spectrum {
        self.r * float(consts::FRAC_1_PI)
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f, sampled_type: BxdfType) -> Option<Sample> {
        unimplemented!()
    }

    fn rho(&self, _: Option<Vector3f>, _: i32, _: &[Point2f]) -> Spectrum {
        self.r
    }
}
