use prelude::*;
use scene::Scene;
use interaction::{ Interaction, Sample };
use sampler::Sampler;
use math::*;
use math::Transform;

mod point;
pub use self::point::PointLight;

bitflags! {
    pub struct LightType: u8 {
        /// The light uses a Delta Function for its position.
        /// This means that it cannot be intersected by chance.
        const DeltaPosition     = 1 << 0;
        const DeltaDirection    = 1 << 1;
        const Area              = 1 << 2;
        const Infinite          = 1 << 3;
    }
}

pub trait Light {
    fn ty(&self) -> LightType;

    fn is_delta_light(&self) -> bool {
        self.ty().intersects(LightType::DeltaPosition | LightType::DeltaDirection)
    }

    fn num_samples(&self) -> u32 {
        1
    }

    fn medium_interface(&self) -> Option<()>;

    fn light_to_world(&self) -> &Transform;

    fn world_to_light(&self) -> &Transform;

    fn preprocess(&mut self, _: &Scene) {

    }

    fn le(&self, ray: &Ray) -> Spectrum;

    /// Returns the radiance arriving at the `isect` point and time,
    /// assuming there are no occluding objects between them.
    /// The `VisibilityTester` is not returned if the radiance is black,
    /// as in this case, visibility is irrelevant.
    fn sample_li(&self, isect: &Interaction, sample: Point2f) -> (Sample, Option<VisibilityTester>);

    fn power(&self) -> Spectrum;
}

pub struct VisibilityTester {
    pub p0: Interaction,
    pub p1: Interaction,
}

impl VisibilityTester {
    pub fn new(p0: Interaction, p1: Interaction) -> Self {
        Self { p0, p1 }
    }

    #[inline(always)]
    fn ray(&self) -> Ray {
        self.p0.spawn_ray_to(&self.p1.p)
    }

    #[inline(always)]
    pub fn unoccluded(&self, scene: &Scene) -> bool {
        scene.intersect_p(&self.ray())
    }

    pub fn tr(&self, scene: &Scene, _sampler: &Sampler) -> Spectrum {
        let mut ray = self.ray();
        let tr = Spectrum::new(1.0);

        loop {
            let isect = scene.intersect(&mut ray);
            // if the ray intersects something
            // and that something has a material
            // then the ray is occluded
            if let Some(isect) = &isect {
                if isect.primitive.map_or(false, |p| p.get_material().is_some()) {
                    return Spectrum::new(0.0);
                }
            }

            // todo - transmittance for current ray segment in medium
            if let Some(_medium) = ray.medium {
                // tr *= ray.medium.tr(ray, sampler)
            }

            // if no intersection is found then the ray has got to
            // p1!
            // else - we've hit an invisible surface, so start
            // tracing again from that surface -> p1
            match &isect {
                Some(isect) => ray = isect.spawn_ray_to(&self.p1.p),
                None => break,
            }
        }

        tr
    }
}
