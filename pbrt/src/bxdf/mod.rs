use std::fmt::Debug;
use cg::{ Point2, Vector3 };
use prelude::*;
use math::*;
use interaction::Sample;
use sampling::utils::*;

pub mod bsdf;
pub use self::bsdf::*;

pub mod fresnel;
pub use self::fresnel::*;

pub mod lambertian;
pub use self::lambertian::*;

pub mod specular;
pub use self::specular::*;

pub mod utils;
use self::utils::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransportMode {
    Camera,
    Light,
}

bitflags! {
    pub struct BxdfType: u8 {
        const Reflection    = 1 << 0;
        const Transmission  = 1 << 1;
        const Diffuse       = 1 << 2;
        const Glossy        = 1 << 3;
        const Specular      = 1 << 4;
    }
}

pub trait Bxdf: Debug {
    fn ty(&self) -> BxdfType;

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum;

    fn sample_f(&self, wo: Vector3f, u: Point2f, _: BxdfType) -> Option<Sample> {
        let mut wi = cosine_sample_hemisphere(u);
        if wo.z < 0.0 {
            wi.z *= float(-1.0);
        }

        let pdf = self.pdf(wo, wi);
        let f = self.f(wo, wi);

        Some(Sample {
            wi,
            pdf,
            li: f,
            ty: Some(self.ty()),
        })
    }

    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> Float {
        if same_hemisphere(wo, wi) {
            cos_theta_abs(wi) * Float::frac_1_pi()
        } else {
            float(0.0)
        }
    }

    fn rho(&self, wo: Option<Vector3f>, n_samples: i32, samples: &[Point2f]) -> Spectrum;
}

#[derive(Debug)]
pub struct ScaledBxdf<B: Bxdf> {
    bxdf: B,
    scale: Spectrum,
}

impl<B: Bxdf> Bxdf for ScaledBxdf<B> {
    fn ty(&self) -> BxdfType {
        self.bxdf.ty()
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        self.scale * self.bxdf.f(wo, wi)
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f, sampled_type: BxdfType) -> Option<Sample> {
        if let Some(mut sample) = self.bxdf.sample_f(wo, sample, sampled_type) {
            sample.li *= self.scale;
            Some(sample)
        } else {
            None
        }
    }

    fn rho(&self, wo: Option<Vector3f>, n_samples: i32, samples: &[Point2f]) -> Spectrum {
        self.scale * self.bxdf.rho(wo, n_samples, samples)
    }
}
