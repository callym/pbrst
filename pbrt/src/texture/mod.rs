use prelude::*;
use interaction::SurfaceInteraction;

mod constant;
pub use self::constant::ConstantTexture;

mod uv_mapping;
pub use self::uv_mapping::UvMapping2d;

pub trait Texture<T> {
    fn evaluate(&self, si: &SurfaceInteraction) -> T;
}

pub struct Mapping2d {
    pub point: Point2f,
    pub dstdx: Vector2f,
    pub dstdy: Vector2f,
}

pub trait TextureMapping2d {
    fn map(&self, si: &SurfaceInteraction) -> Mapping2d;
}
