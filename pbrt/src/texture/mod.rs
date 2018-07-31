use std::fmt::Debug;

use crate::prelude::*;
use crate::interaction::SurfaceInteraction;

mod constant;
pub use self::constant::ConstantTexture;

mod uv_mapping;
pub use self::uv_mapping::UvMapping2d;

pub trait Texture<T>: Debug {
    fn evaluate(&self, si: &SurfaceInteraction<'_>) -> T;
}

pub struct Mapping2d {
    pub point: Point2f,
    pub dstdx: Vector2f,
    pub dstdy: Vector2f,
}

pub trait TextureMapping2d {
    fn map(&self, si: &SurfaceInteraction<'_>) -> Mapping2d;
}
