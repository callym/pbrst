use cg::Point2;
use prelude::*;
use scene::Scene;
use interaction::{ Sample, SurfaceInteraction };
use math::*;

pub trait Light {
    fn preprocess(&mut self, _: &Scene) {

    }

    fn le(&self, ray: &Ray) -> Spectrum {
        Spectrum::new(0.0)
    }

    fn sample_li(&self, isect: &SurfaceInteraction, sample: Point2f) -> (Sample, VisibilityTester);
}

pub struct VisibilityTester {

}

impl VisibilityTester {
    pub fn unoccluded(&self, scene: &Scene) -> bool {
        unimplemented!()
    }
}
