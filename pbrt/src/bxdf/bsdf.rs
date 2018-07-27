use std::sync::Arc;
use cg::prelude::*;
use prelude::*;
use interaction::SurfaceInteraction;
use super::{ Bxdf, BxdfType };
use interaction::Sample;

#[derive(Clone, Debug)]
pub struct Bsdf {
    eta: Float,
    n_s: Normal,
    n_g: Normal,
    ss: Vector3f,
    ts: Vector3f,
    bxdfs: Vec<Arc<Bxdf>>,
}

impl Bsdf {
    pub fn new(si: &SurfaceInteraction, eta: Option<Float>) -> Self {
        let eta = eta.unwrap_or(float(1.0));

        Self {
            eta,
            n_s: si.shading.n,
            n_g: si.n,
            ss: si.shading.dpdu.normalize(),
            ts: (*si.shading.n).cross(si.shading.dpdu.normalize()),
            bxdfs: vec![],
        }
    }

    pub fn add(&mut self, bxdf: Arc<Bxdf>) {
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
            if ty.contains(flags) {
                if (reflect && ty.contains(BxdfType::Reflection)) ||
                  (!reflect && ty.contains(BxdfType::Transmission)) {
                    f += bxdf.f(wo, wi);
                }
            }
        }

        f
    }

    pub fn sample_f(&self, wo: Vector3f, sample: Point2f, sampled_type: BxdfType) -> Option<Sample> {
        unimplemented!()
    }

    pub fn rho(&self, wo: Option<Vector3f>, n_samples: i32, samples: &[Point2f]) -> Spectrum {
        let mut f = Spectrum::new(0.0);

        for bxdf in &self.bxdfs {
            f += bxdf.rho(wo, n_samples, samples);
        }

        f
    }

}
