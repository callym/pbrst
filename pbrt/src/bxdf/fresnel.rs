use std::cmp::{ max, min };
use std::fmt::Debug;
use std::mem;
use num;
use prelude::*;

pub trait Fresnel: Debug {
    fn evaluate(&self, cos_theta_i: Float) -> Spectrum;
}

#[derive(Debug)]
pub struct FresnelConductor {
    eta_i: Spectrum,
    eta_t: Spectrum,
    k: Spectrum,
}

impl FresnelConductor {
    pub fn new(eta_i: Spectrum, eta_t: Spectrum, k: Spectrum) -> Self {
        Self { eta_i, eta_t, k }
    }
}

impl Fresnel for FresnelConductor {
    fn evaluate(&self, cos_theta_i: Float) -> Spectrum {
        conductor(cos_theta_i.abs(), self.eta_i, self.eta_t, self.k)
    }
}

#[derive(Debug)]
pub struct FresnelDielectric {
    eta_i: Float,
    eta_t: Float,
}

impl FresnelDielectric {
    pub fn new(eta_i: Float, eta_t: Float) -> Self {
        Self { eta_i, eta_t }
    }
}

impl Fresnel for FresnelDielectric {
    fn evaluate(&self, cos_theta_i: Float) -> Spectrum {
        Spectrum::new(dielectric(cos_theta_i, self.eta_i, self.eta_t))
    }
}

#[derive(Debug)]
pub struct FresnelNoOp;

impl Fresnel for FresnelNoOp {
    fn evaluate(&self, _: Float) -> Spectrum {
        Spectrum::new(1.0)
    }
}

fn dielectric(cos_theta_i: Float, mut eta_i: Float, mut eta_t: Float) -> Float {
    let mut cos_theta_i = num::clamp(cos_theta_i, float(-1.0), float(1.0));

    let entering = cos_theta_i > 0.0;
    if !entering {
        mem::swap(&mut eta_i, &mut eta_t);
        cos_theta_i = cos_theta_i.abs();
    }

    // compute cos_theta_t using Snell's Law
    let sin_theta_i = max(float(0.0), float(1.0) - cos_theta_i.powi(2));
    let sin_theta_t = eta_i / eta_t * sin_theta_i;

    // handle total internal reflection
    if sin_theta_t >= 1.0 {
        return float(1.0)
    }

    let cos_theta_t = max(float(0.0), float(1.0) - sin_theta_t.powi(2));

    let r_parr = ((eta_t * cos_theta_i) - (eta_i * cos_theta_t)) /
                 ((eta_t * cos_theta_i) + (eta_i * cos_theta_t));
    let r_perp = ((eta_i * cos_theta_i) - (eta_t * cos_theta_t)) /
                 ((eta_i * cos_theta_i) + (eta_t * cos_theta_t));

    (r_parr.powi(2) + r_perp.powi(2)) / float(2.0)
}

fn conductor(cos_theta_i: Float, eta_i: Spectrum, eta_t: Spectrum, k: Spectrum) -> Spectrum {
    let cos_theta_i = num::clamp(cos_theta_i, float(-1.0), float(1.0));
    let eta = eta_t / eta_i;
    let eta_k = k / eta_i;

    let cos_theta_i_2 = cos_theta_i.powi(2);
    let sin_theta_i_2 = float(1.0) - cos_theta_i_2;
    let eta_2 = eta.powi(2);
    let eta_k_2 = eta_k.powi(2);

    let t0 = eta_2 - eta_k_2 - sin_theta_i_2;
    let a2_p_b2 = (t0.powi(2) + eta_2 * eta_k_2 * float(4)).sqrt();
    let t1 = a2_p_b2 + cos_theta_i_2;
    let a = ((a2_p_b2 + t0) * float(0.5)).sqrt();
    let t2 = a * float(2.0) * cos_theta_i;
    let rs = (t1 - t2) / (t1 + t2);

    let t3 = a2_p_b2 * cos_theta_i_2 + sin_theta_i_2.powi(2);
    let t4 = t2 * sin_theta_i_2;
    let rp = rs * (t3 - t4) / (t3 + t4);

    (rp + rs) * float(0.5)
}
