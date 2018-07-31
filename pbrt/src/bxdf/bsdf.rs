use std::cmp::min;
use std::sync::Arc;
use cgmath::prelude::*;
use crate::prelude::*;
use crate::interaction::SurfaceInteraction;
use super::{ Bxdf, BxdfType };
use crate::interaction::Sample;

#[derive(Clone, Debug)]
pub struct Bsdf {
    pub eta: Float,
    n_s: Normal,
    n_g: Normal,
    ss: Vector3f,
    ts: Vector3f,
    bxdfs: Vec<Arc<dyn Bxdf>>,
}

impl Bsdf {
    pub fn new(si: &SurfaceInteraction<'_>, eta: Option<Float>) -> Self {
        let eta = eta.unwrap_or_else(|| float(1.0));

        Self {
            eta,
            n_s: si.shading.n,
            n_g: si.n.unwrap(),
            ss: si.shading.dpdu.normalize(),
            ts: (*si.shading.n).cross(si.shading.dpdu.normalize()),
            bxdfs: vec![],
        }
    }

    pub fn add(&mut self, bxdf: Arc<dyn Bxdf>) {
        self.bxdfs.push(bxdf);
    }

    pub fn num_components(&self, flags: BxdfType) -> usize {
        let mut num = 0;

        for bxdf in &self.bxdfs {
            if bxdf.ty().contains(flags) {
                num += 1;
            }
        }

        num
    }

    pub fn world_to_local(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(v.dot(self.ss), v.dot(self.ts), v.dot(*self.n_s))
    }

    pub fn local_to_world(&self, v: Vector3f) -> Vector3f {
        let ss = self.ss.mul_element_wise(v.x);
        let ts = self.ts.mul_element_wise(v.y);
        let ns = self.n_s.mul_element_wise(v.z);

        ss + ts + ns
    }

    pub fn f(&self, wo_w: Vector3f, wi_w: Vector3f, flags: BxdfType) -> Spectrum {
        let wi = self.world_to_local(wi_w);
        let wo = self.world_to_local(wo_w);

        let reflect = wi_w.dot(*self.n_g) * wo_w.dot(*self.n_g) > 0.0;

        let mut f = Spectrum::new(0.0);

        for bxdf in &self.bxdfs {
            let ty = bxdf.ty();
            if ty.intersects(flags) &&
                ((reflect && ty.contains(BxdfType::Reflection)) ||
                (!reflect && ty.contains(BxdfType::Transmission))) {
                f += bxdf.f(wo, wi);
            }
        }

        f
    }

    pub fn sample_f(&self, wo_world: Vector3f, u: Point2f, ty: BxdfType) -> Option<Sample> {
        let matching = self.num_components(ty);

        if matching == 0 {
            return None;
        }

        let comp = min(
            (u[0] * float(matching)).floor().raw() as usize,
            matching - 1,
        );

        let bxdfs = self.bxdfs.iter()
            .filter(|b| b.ty().contains(ty))
            .enumerate()
            .collect::<Vec<_>>();
        let (chosen_idx, bxdf) = bxdfs[comp];

        // u[0] is no longer uniformly distributed
        // but we can a uniformly distributed one back
        let u_remapped = Point2f::new(
            u[0] * float(matching) - float(comp),
            u[1],
        );

        let wo_local = self.world_to_local(wo_world);
        let mut sample = match bxdf.sample_f(wo_local, u_remapped) {
            Some(sample) => sample,
            None => return None,
        };

        if sample.pdf == 0.0 {
            return None;
        }

        let wi_world = self.local_to_world(sample.wi);

        if !bxdf.ty().contains(BxdfType::Specular) && matching > 1 {
            for (idx, bxdf) in &bxdfs {
                if *idx == chosen_idx {
                    continue;
                }

                sample.pdf += bxdf.pdf(wo_local, sample.wi);
            }
        }

        if matching > 1 {
            sample.pdf /= float(matching);
        }

        if !bxdf.ty().contains(BxdfType::Specular) && matching > 1 {
            let reflect = wi_world.dot(*self.n_g) * wo_world.dot(*self.n_g) > 0.0;

            let mut f = Spectrum::new(0.0);

            for (_, bxdf) in &bxdfs {
                if bxdf.ty().contains(ty) &&
                    ((reflect && bxdf.ty().contains(BxdfType::Reflection)) ||
                     (!reflect && bxdf.ty().contains(BxdfType::Transmission))) {
                    f += bxdf.f(wo_local, sample.wi);
                }
            }

            sample.li = f;
        }

        Some(sample)
    }

    pub fn pdf(&self, wo_w: Vector3f, wi_w: Vector3f, flags: BxdfType) -> Float {
        if self.bxdfs.is_empty() {
            return float(0.0);
        }

        let wi = self.world_to_local(wi_w);
        let wo = self.world_to_local(wo_w);

        if wo.z == 0.0 {
            return float(0.0);
        }

        let mut pdf = float(0.0);
        let mut matching = 0;

        for bxdf in &self.bxdfs {
            let ty = bxdf.ty();
            if ty.intersects(flags) {
                matching += 1;
                pdf += bxdf.pdf(wo, wi);
            }
        }

        if matching > 0 {
            pdf / float(matching)
        } else {
            float(0.0)
        }
    }

    pub fn rho(&self, wo: Option<Vector3f>, n_samples: i32, samples: &[Point2f]) -> Spectrum {
        let mut f = Spectrum::new(0.0);

        for bxdf in &self.bxdfs {
            f += bxdf.rho(wo, n_samples, samples);
        }

        f
    }
}
