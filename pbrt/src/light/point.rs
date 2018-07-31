use std::sync::Arc;
use cgmath::prelude::*;
use crate::prelude::*;
use super::{ Light, LightType, VisibilityTester };
use crate::interaction::{ Interactions, BaseInteraction, Sample };
use crate::math::Transform;

#[derive(Debug)]
pub struct PointLight {
    light_to_world: Arc<Transform>,
    world_to_light: Arc<Transform>,
    position: Point3f,
    spectrum: Spectrum,
}

impl PointLight {
    pub fn new(spectrum: Spectrum, light_to_world: Arc<Transform>) -> Self {
        Self {
            spectrum,
            world_to_light: Arc::new(light_to_world.inverse()),
            position: light_to_world.transform_point(Point3f::zero()),
            light_to_world,
        }
    }
}

impl Light for PointLight {
    fn ty(&self) -> LightType {
        LightType::DeltaPosition
    }

    fn medium_interface(&self) -> Option<()> {
        None
    }

    fn light_to_world(&self) -> &Transform {
        &self.light_to_world
    }

    fn world_to_light(&self) -> &Transform {
        &self.world_to_light
    }

    fn le(&self, _: &Ray) -> Spectrum {
        Spectrum::new(0.0)
    }

    /// Returns the radiance arriving at the `isect` point and time,
    /// assuming there are no occluding objects between them.
    /// The `VisibilityTester` is not returned if the radiance is black,
    /// as in this case, visibility is irrelevant.
    fn sample_li(&self, isect: &Interactions<'a>, _sample: Point2f) -> (Sample, Option<VisibilityTester>) {
        let isect = isect.get_base();

        let wi = self.position - isect.p;
        let vis = VisibilityTester::new(isect.clone(), BaseInteraction {
            p: self.position,
            time: isect.time,
            p_err: Vector3f::zero(),
            wo: Vector3f::zero(),
            n: None,
            medium: self.medium_interface(),
        });

        let li = self.spectrum / self.position.distance2(isect.p);

        (
            Sample {
                li,
                pdf: float(1.0),
                wi,
                ty: None,
            },
            Some(vis),
        )
    }

    fn power(&self) -> Spectrum {
        self.spectrum * float(4.0) * Float::pi()
    }
}
