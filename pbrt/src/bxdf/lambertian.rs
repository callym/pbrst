use prelude::*;
use super::*;
use super::utils::*;
use sampling::utils::*;

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

    fn sample_f(&self, wo: Vector3f, u: Point2f, sampled_type: BxdfType) -> Option<Sample> {
        let mut wi = cosine_sample_hemisphere(u);
        if wo.z < 0.0 {
            wi.z *= float(-1.0);
        }

        // todo - this is wrong
        let pdf = cos_theta(wi).abs() * Float::frac_1_pi();
        let f = self.f(wo, wi);

        Some(Sample {
            wi,
            pdf,
            li: f,
            ty: Some(self.ty()),
        })
    }

    fn rho(&self, _: Option<Vector3f>, _: i32, _: &[Point2f]) -> Spectrum {
        self.r
    }
}
