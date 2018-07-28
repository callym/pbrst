use std::sync::Arc;
use prelude::*;
use bxdf::{ Bsdf, TransportMode };
use super::Primitive;
use material::Material;
use shape::Shape;
use interaction::SurfaceInteraction;

#[derive(Debug)]
pub struct GeometricPrimitive {
    pub shape: Arc<Shape + Send + Sync>,
    pub material: Option<Box<Material + Send + Sync>>,
    pub area_light: Option<Arc<()>>,
    pub medium_interface: Option<()>,
}

impl Primitive for GeometricPrimitive {
    fn intersect<'a>(&'a self, ray: &mut Ray) -> Option<SurfaceInteraction<'a>> {
        if let Some((hit, mut isect)) = self.shape.intersect(ray, true) {
            ray.max = Some(hit);
            isect.primitive = Some(self);

            // Initialize mediumInterface

            Some(isect)
        } else {
            None
        }
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.shape.intersect_p(ray, true)
    }

    fn world_bound(&self) -> Bounds3<Float> {
        self.shape.world_bound()
    }

    fn get_area_light(&self) -> Option<Arc<()>> {
        self.area_light.as_ref().map(|a| a.clone())
    }

    fn get_material(&self) -> Option<&Box<Material + Send + Sync>> {
        self.material.as_ref()
    }

    fn compute_scattering_functions<'a>(&'a self, isect: SurfaceInteraction<'a>, arena: &(), mode: TransportMode, allow_multiple_lobes: bool) -> SurfaceInteraction<'a> {
        if let Some(material) = &self.material {
            material.compute_scattering_functions(isect, arena, mode, allow_multiple_lobes)
        } else {
            panic!("GeometricPrimitive doesn't have material")
        }
    }
}
