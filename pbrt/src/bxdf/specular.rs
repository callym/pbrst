use prelude::*;
use super::*;
use super::utils::*;

#[derive(Debug)]
pub struct SpecularReflection {
    pub r: Spectrum,
    pub fresnel: Box<Fresnel>,
}

impl Bxdf for SpecularReflection {
    fn ty(&self) -> BxdfType {
        BxdfType::Reflection | BxdfType::Specular
    }

    fn f(&self, _: Vector3f, _: Vector3f) -> Spectrum {
        Spectrum::new(0.0)
    }

    fn sample_f(&self, wo: Vector3f, _: Point2f, _: BxdfType) -> Option<Sample> {
        let wi = Vector3f::new(-wo.x, -wo.y, wo.z);

        Some(Sample {
            pdf: float(1.0),
            wi,
            li: self.fresnel.evaluate(cos_theta(wi)) * self.r / cos_theta_abs(wi),
            ty: Some(self.ty()),
        })
    }

    fn rho(&self, wo: Option<Vector3f>, n_samples: i32, samples: &[Point2f]) -> Spectrum {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct SpecularTransmission {
    t: Spectrum,
    eta_a: Float,
    eta_b: Float,
    transport_mode: TransportMode,
    fresnel: FresnelDielectric,
}

impl SpecularTransmission {
    pub fn new(t: Spectrum, eta_a: Float, eta_b: Float, transport_mode: TransportMode) -> Self {
        Self {
            t,
            eta_a,
            eta_b,
            transport_mode,
            fresnel: FresnelDielectric::new(eta_a, eta_b),
        }
    }
}

impl Bxdf for SpecularTransmission {
    fn ty(&self) -> BxdfType {
        BxdfType::Transmission | BxdfType::Specular
    }

    fn f(&self, _: Vector3f, _: Vector3f) -> Spectrum {
        Spectrum::new(0.0)
    }

    fn sample_f(&self, wo: Vector3f, samples: Point2f, sampled_type: BxdfType) -> Option<Sample> {
        // which eta is incident and which is transmitted
        let (eta_i, eta_t) = if cos_theta(wo) > 0.0 {
            (self.eta_a, self.eta_b)
        } else {
            (self.eta_b, self.eta_a)
        };

        let wi = if let Some(wi) = refract(wo, Normal::new(0.0, 0.0, 1.0).face_forward(wo), eta_i / eta_t) {
            wi
        } else {
            return None;
        };

        let li = self.t * (Spectrum::new(1.0) - self.fresnel.evaluate(cos_theta(wi)));

        // account for non-symmetry w transmission to different medium

        Some(Sample {
            wi,
            pdf: float(1.0),
            li,
            ty: Some(self.ty()),
        })
    }

    fn rho(&self, wo: Option<Vector3f>, n_samples: i32, samples: &[Point2f]) -> Spectrum {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct SpecularFresnel {
    r: Spectrum,
    t: Spectrum,
    eta_a: Float,
    eta_b: Float,
    mode: TransportMode,
    fresnel: FresnelDielectric,
}

impl SpecularFresnel {
    pub fn new(r: Spectrum, t: Spectrum, eta_a: Float, eta_b: Float, mode: TransportMode) -> Self {
        Self {
            r,
            t,
            eta_a,
            eta_b,
            mode,
            fresnel: FresnelDielectric::new(eta_a, eta_b),
        }
    }
}

impl Bxdf for SpecularFresnel {
    fn ty(&self) -> BxdfType {
        BxdfType::Reflection | BxdfType::Transmission | BxdfType::Specular
    }

    fn f(&self, _: Vector3f, _: Vector3f) -> Spectrum {
        Spectrum::new(0.0)
    }

    fn sample_f(&self, wo: Vector3f, samples: Point2f, sampled_type: BxdfType) -> Option<Sample> {
        unimplemented!()
    }

    fn rho(&self, wo: Option<Vector3f>, n_samples: i32, samples: &[Point2f]) -> Spectrum {
        unimplemented!()
    }
}
