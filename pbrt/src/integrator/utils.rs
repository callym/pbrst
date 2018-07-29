use std::cmp::min;
use std::sync::Arc;
use cg::prelude::*;

use prelude::*;
use bxdf::BxdfType;
use light::Light;
use math::*;
use sampler::Sampler;
use sampling::utils::*;
use scene::Scene;
use interaction::{ Interactions, SurfaceInteraction };
use super::ParIntegratorData;

pub fn uniform_sample_all_lights<'a>(isect: &(impl Into<Interactions<'a>> + Clone), scene: &Scene, sampler: &mut Sampler, arena: &(), n_samples: &[u32], handle_media: bool) -> Spectrum {
    let mut l = Spectrum::new(0.0);

    for (light, s_i) in izip!(&*scene.lights, n_samples) {
        let light_arr = sampler.get_2d_vec(*s_i);
        let scattering_arr = sampler.get_2d_vec(*s_i);

        if light_arr.is_none() || scattering_arr.is_none() {
            // fall back to a single sample for the light
            let u_light = sampler.get_2d();
            let u_scattering = sampler.get_2d();
            l += estimate_direct(isect, u_scattering, light, u_light, scene, sampler, arena, handle_media, false);
        } else {
            // estimate direct lighting using sample arrays
            let light_arr = light_arr.unwrap();
            let scattering_arr = scattering_arr.unwrap();

            let mut ld = Spectrum::new(0.0);

            for (u_light, u_scattering) in izip!(light_arr, scattering_arr) {
                ld +=estimate_direct(isect, u_scattering, light, u_light, scene, sampler, arena, handle_media, false);
            }

            l += ld / float(*s_i);
        }
    }

    l
}

pub fn uniform_sample_one_light<'a>(isect: &(impl Into<Interactions<'a>> + Clone), scene: &Scene, sampler: &mut Sampler, arena: &(), handle_media: bool) -> Spectrum {
    // randomly choose a single light to sample
    let n_lights = scene.lights.len();
    if n_lights == 0 {
        return Spectrum::new(0.0);
    }

    let light_num = min(sampler.get_1d().raw() as usize * n_lights, n_lights - 1);
    let light = &scene.lights[light_num];

    let u_light = sampler.get_2d();
    let u_scattering = sampler.get_2d();

    estimate_direct(isect, u_scattering, light, u_light, scene, sampler, arena, handle_media, false) * float(n_lights)
}

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
pub fn estimate_direct<'a>(isect: &(impl Into<Interactions<'a>> + Clone), u_scattering: Point2f, light: &Arc<Light + Send + Sync>, u_light: Point2f, scene: &Scene, sampler: &mut Sampler, _arena: &(), handle_media: bool, specular: bool) -> Spectrum {
    let isect: Interactions = (*isect).clone().into();
    let flags = if specular {
        BxdfType::all()
    } else {
        let mut bxdf = BxdfType::all();
        bxdf.remove(BxdfType::Specular);
        bxdf
    };

    let mut ld = Spectrum::new(0.0);
    let mut scattering_pdf = float(0.0);

    // sample light source with multiple importance sampling
    let (mut light_sample, vis) = light.sample_li(&isect, u_light);

    if light_sample.pdf > 0.0 && !light_sample.li.is_black() {
        // compute BSDF or phase function value for light sample
        let mut f = Spectrum::new(0.0);

        if let Some(isect) = isect.get_surface() {
            // evaluate BSDF for light sampling strategy
            if let Some(bsdf) = &isect.bsdf {
                f = bsdf.f(
                        isect.wo,
                        light_sample.wi,
                        flags
                    ) * light_sample.wi.dot(*isect.shading.n).abs();
                scattering_pdf = bsdf.pdf(isect.wo, light_sample.wi, flags);
            }
        } else {
            // evaluate phase function for light sampling strategy
            unimplemented!()
        }

        if vis.is_some() && !f.is_black() {
            let vis = vis.unwrap();
            // compute effect of visibility for light source sample
            if handle_media {
                light_sample.li *= vis.tr(scene, sampler);
            } else if !vis.unoccluded(scene) {
                light_sample.li = Spectrum::new(0.0)
            }

            // add light's contribution to reflected radiance
            if !light_sample.li.is_black() {
                if light.is_delta_light() {
                    ld += f * light_sample.li / light_sample.pdf;
                } else {
                    let weight = power_heuristic(1, light_sample.pdf, 1, scattering_pdf);
                    ld += f * light_sample.li * weight * light_sample.pdf;
                }
            }
        }
    }

    // sample BSDF with multiple importance sampling
    if !light.is_delta_light() {
        let mut f = Spectrum::new(0.0);

        // was a delta distribution sampled?
        // specular is always a delta distribution
        // because the 1-1 mapping between
        // wo and wi (I think?)
        let mut sampled_specular = false;

        if let Some(isect) = isect.get_surface() {
            if let Some(bsdf) = &isect.bsdf {
                if let Some(sample) = bsdf.sample_f(isect.wo, u_scattering, flags) {
                    f = sample.li * sample.wi.dot(*isect.shading.n).abs();
                    sampled_specular = sample.ty.map_or(false, |ty| ty.contains(BxdfType::Specular));
                }
            }
        } else {
            unimplemented!()
        }

        if !f.is_black() && scattering_pdf > 0.0 {
            // account for light contributions along wi
            let weight = if !sampled_specular {
                //let light_pdf = light.pdf_li(isect, light_sample.wi);
                let light_pdf = float(0.0);
                if light_pdf == 0.0 {
                    return ld;
                }

                power_heuristic(1, scattering_pdf, 1, light_pdf)
            } else {
                float(1.0)
            };

            // find intersection & compute transmittance
            let mut ray = isect.get_base().spawn_ray(&light_sample.wi);

            let tr = Spectrum::new(1.0);

            let found_surface_interaction = if handle_media {
                //scene.intersect_tr(ray, sampler);
                None
            } else {
                scene.intersect(&mut ray)
            };

            // add light contribution from material sampling
            let mut li = Spectrum::new(0.0);
            if let Some(l_isect) = found_surface_interaction {
                if let Some(primitive) = l_isect.primitive {
                    if let Some(area_light) = primitive.get_area_light() {
                        // if the two Arc<Light>s point to the same light
                        if Arc::ptr_eq(&area_light, light) {
                            li = l_isect.le(&-light_sample.wi);
                        }
                    }
                }
            } else {
                li = light.le(&ray);
            }

            if !li.is_black() {
                ld += f * li * tr * weight / scattering_pdf;
            }
        }
    }

    ld
}

pub fn specular_reflect(integrator: &impl ParIntegratorData, ray: &RayDifferential, isect: &SurfaceInteraction, scene: &Scene, sampler: &mut Sampler, arena: &(), depth: i32) -> Spectrum {
    // compute specular reflection direction wi and bsdf value
    let wo = isect.wo;

    let ty = BxdfType::Reflection | BxdfType::Specular;

    let bsdf = match &isect.bsdf {
        Some(bsdf) => bsdf,
        None => return Spectrum::new(0.0),
    };

    let f = match bsdf.sample_f(wo, sampler.get_2d(), ty) {
        Some(f) => f,
        None => return Spectrum::new(0.0),
    };

    // return contribution of specular reflection
    let ns = isect.shading.n;

    if f.pdf > 0.0 && !f.li.is_black() && f.wi.dot(*ns).abs() != 0.0 {
        // compute ray differential `rd` for specular reflection
        let rd = isect.spawn_ray(&f.wi);
        let mut rd = RayDifferential::from_ray(rd);

        if ray.has_differentials() {
            let rx = ray.x.unwrap();
            let ry = ray.y.unwrap();

            let rx_o = isect.interaction.p + isect.dpdx;
            let ry_o = isect.interaction.p + isect.dpdy;

            // compute differential reflected directions
            let dndx =  *isect.shading.dndu * isect.dudx +
                        *isect.shading.dndv * isect.dvdx;
            let dndy =  *isect.shading.dndu * isect.dudy +
                        *isect.shading.dndv * isect.dvdy;
            let dwodx = -rx.direction - wo;
            let dwody = -ry.direction - wo;
            let ddndx = dwodx.dot(*ns) + wo.dot(dndx);
            let ddndy = dwody.dot(*ns) + wo.dot(dndy);

            let rx_d = f.wi - dwodx + (dndx * wo.dot(*ns) +  *ns * ddndx * float(2.0));
            let ry_d = f.wi - dwody + (dndy * wo.dot(*ns) +  *ns * ddndy * float(2.0));

            let rx = RayData::new(rx_o, rx_d);
            let ry = RayData::new(ry_o, ry_d);

            rd.x = Some(rx);
            rd.y = Some(ry);
        }

        f.li * integrator.li(rd, scene, sampler, arena, depth + 1) * (f.wi.dot(*ns) / f.pdf)
    } else {
        Spectrum::new(0.0)
    }
}

pub fn specular_transmit(integrator: &impl ParIntegratorData, ray: &RayDifferential, isect: &SurfaceInteraction, scene: &Scene, sampler: &mut Sampler, arena: &(), depth: i32) -> Spectrum {
    // compute specular reflection direction wi and bsdf value
    let wo = isect.wo;

    let ty = BxdfType::Transmission | BxdfType::Specular;

    let bsdf = match &isect.bsdf {
        Some(bsdf) => bsdf,
        None => return Spectrum::new(0.0),
    };

    let f = match bsdf.sample_f(wo, sampler.get_2d(), ty) {
        Some(f) => f,
        None => return Spectrum::new(0.0),
    };

    // return contribution of specular reflection
    let ns = isect.shading.n;

    if f.pdf > 0.0 && !f.li.is_black() && f.wi.dot(*ns).abs() != 0.0 {
        // compute ray differential `rd` for specular reflection
        let rd = isect.spawn_ray(&f.wi);
        let mut rd = RayDifferential::from_ray(rd);

        if ray.has_differentials() {
            let rx = ray.x.unwrap();
            let ry = ray.y.unwrap();

            let rx_o = isect.interaction.p + isect.dpdx;
            let ry_o = isect.interaction.p + isect.dpdy;

            let w = -wo;

            let eta = if wo.dot(*ns) < 0.0 {
                float(1.0) / bsdf.eta
            } else {
                bsdf.eta
            };

            let dndx =  *isect.shading.dndu * isect.dudx +
                        *isect.shading.dndv * isect.dvdx;
            let dndy =  *isect.shading.dndu * isect.dudy +
                        *isect.shading.dndv * isect.dvdy;
            let dwodx = -rx.direction - wo;
            let dwody = -ry.direction - wo;
            let ddndx = dwodx.dot(*ns) + wo.dot(dndx);
            let ddndy = dwody.dot(*ns) + wo.dot(dndy);

            let mu = eta * w.dot(*ns) - f.wi.dot(*ns);
            let dmudx = (eta - (eta.powi(2) * w.dot(*ns)) / f.wi.dot(*ns)) * ddndx;
            let dmudy = (eta - (eta.powi(2) * w.dot(*ns)) / f.wi.dot(*ns)) * ddndy;

            let rx_d = f.wi + dwodx * eta - (dndx * mu + *ns * dmudx);
            let ry_d = f.wi + dwody * eta - (dndy * mu + *ns * dmudy);

            let rx = RayData::new(rx_o, rx_d);
            let ry = RayData::new(ry_o, ry_d);

            rd.x = Some(rx);
            rd.y = Some(ry);
        }

        f.li * integrator.li(rd, scene, sampler, arena, depth + 1) * (f.wi.dot(*ns) / f.pdf)
    } else {
        Spectrum::new(0.0)
    }
}
