use std::cmp::min;
use std::sync::Arc;
use cg;
use cg::prelude::*;

use prelude::*;
use bxdf::{ Bxdf, BxdfType };
use light::Light;
use math::*;
use sampler::Sampler;
use scene::Scene;
use interaction::{ Interactions, BaseInteraction, SurfaceInteraction };
use super::ParIntegratorData;

pub fn specular_reflect(integrator: &impl ParIntegratorData, ray: &RayDifferential, isect: &SurfaceInteraction, scene: &Scene, sampler: &mut Box<Sampler + Send>, arena: &(), depth: i32) -> Spectrum {
    // compute specular reflection direction wi and bsdf value
    let wo = isect.wo;

    let ty = BxdfType::Reflection | BxdfType::Specular;

    let bsdf = match &isect.bsdf {
        Some(bsdf) => bsdf,
        None => return Spectrum::new(0.0),
    };

    let f = match bsdf.sample_f(wo, sampler.get_2d()) {
        Some(f) => f,
        None => return Spectrum::new(0.0),
    };

    // return contribution of specular reflection
    let ns = isect.shading.n;

    if f.pdf > 0.0 && !f.li.is_black() && cg::dot(f.wi.into(), *ns).abs() != 0.0 {
        // compute ray differential `rd` for specular reflection
        // todo - this is wrong
        let rd = isect.spawn_ray(&f.wi);
        let rd = RayDifferential::from_ray(rd);

        f.li * integrator.li(rd, scene, sampler, arena, depth + 1) * (cg::dot(f.wi.into(), *ns) / f.pdf)
    } else {
        Spectrum::new(0.0)
    }
}

pub fn specular_transmit(integrator: &impl ParIntegratorData, ray: &RayDifferential, isect: &SurfaceInteraction, scene: &Scene, sampler: &mut Box<Sampler + Send>, arena: &(), depth: i32) -> Spectrum {
    // compute specular reflection direction wi and bsdf value
    let wo = isect.wo;

    let ty = BxdfType::Transmission | BxdfType::Specular;

    let bsdf = match &isect.bsdf {
        Some(bsdf) => bsdf,
        None => return Spectrum::new(0.0),
    };

    let f = match bsdf.sample_f(wo, sampler.get_2d()) {
        Some(f) => f,
        None => return Spectrum::new(0.0),
    };

    // return contribution of specular reflection
    let ns = isect.shading.n;

    if f.pdf > 0.0 && !f.li.is_black() && cg::dot(f.wi.into(), *ns).abs() != 0.0 {
        // compute ray differential `rd` for specular reflection
        // todo - this is wrong
        let rd = isect.spawn_ray(&f.wi);
        let rd = RayDifferential::from_ray(rd);

        f.li * integrator.li(rd, scene, sampler, arena, depth + 1) * (cg::dot(f.wi.into(), *ns) / f.pdf)
    } else {
        Spectrum::new(0.0)
    }
}
