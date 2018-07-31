use crate::prelude::*;
use super::{ TextureMapping2d, Mapping2d };
use crate::interaction::SurfaceInteraction;

pub struct UvMapping2d {
    pub su: Float,
    pub sv: Float,
    pub du: Float,
    pub dv: Float,
}

impl TextureMapping2d for UvMapping2d {
    fn map(&self, si: &SurfaceInteraction<'_>) -> Mapping2d {
        let dstdx = Vector2f::new(self.su * si.dudx, self.sv * si.dvdx);
        let dstdy = Vector2f::new(self.su * si.dudy, self.sv * si.dvdy);

        let point = Point2f::new(
            self.su * si.uv[0] + self.du,
            self.sv * si.uv[1] + self.dv,
        );

        Mapping2d { dstdx, dstdy, point }
    }
}
