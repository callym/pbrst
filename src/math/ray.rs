use cg;
use prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct RayData {
    pub origin: Point3f,
    pub direction: Vector3f,
}

impl RayData {
    pub fn p(&self, t: Float) -> Point3f {
        self.origin + self.direction * t
    }
}

#[derive(Copy, Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Ray {
    #[shrinkwrap(main_field)]
    pub ray: RayData,
    pub max: Option<Float>,
    pub time: Float,
    pub medium: Option<()>,
}

impl Ray {
    pub fn new(origin: Point3f, direction: Vector3f) -> Self {
        Self {
            ray: RayData {
                origin,
                direction,
            },
            max: None,
            time: float(0.0),
            medium: None,
        }
    }
}

impl From<cg::Vector3<Float>> for Ray {
    fn from(vec: cg::Vector3<Float>) -> Self {
        unimplemented!()
    }
}

impl Into<cg::Vector3<Float>> for Ray {
    fn into(self) -> cg::Vector3<Float> {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct RayDifferential {
    #[shrinkwrap(main_field)]
    pub ray: Ray,
    pub x: Option<RayData>,
    pub y: Option<RayData>,
}

impl RayDifferential {
    pub fn new(origin: Point3f, direction: Vector3f) -> Self {
        Self {
            ray: Ray::new(origin, direction),
            x: None,
            y: None,
        }
    }

    pub fn has_differentials(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }

    pub fn scale_differentials(&mut self, scale: impl Into<Float>) {
        let scale: Float = scale.into();

        let o = self.origin;
        let d = self.direction;

        self.x.map(|mut x| {
            x.origin = o + (x.origin - o) * scale;
            x.direction = d + (x.direction - d) * scale;
        });

        self.y.map(|mut y| {
            y.origin = o + (y.origin - o) * scale;
            y.direction = d + (y.direction - d) * scale;
        });
    }
}
