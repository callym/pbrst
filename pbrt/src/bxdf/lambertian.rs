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
        self.r * Float::frac_1_pi()
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f, sampled_type: BxdfType) -> Option<Sample> {
        unimplemented!()
    }

    fn rho(&self, _: Option<Vector3f>, _: i32, _: &[Point2f]) -> Spectrum {
        self.r
    }
}
