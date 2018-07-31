#![feature(
    associated_type_defaults,
    const_fn,
    macro_at_most_once_rep,
    nll,
    rust_2018_preview,
    slice_patterns,
    specialization,
    try_from,
    underscore_imports,
    use_extern_macros,
)]
#![warn(rust_2018_idioms)]

#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

pub mod bxdf;
pub mod camera;
pub mod film;
pub mod filter;
pub mod interaction;
pub mod integrator;
pub mod light;
pub mod material;
pub mod math;
pub mod primitive;
pub mod sampler;
pub mod sampling;
pub mod scene;
pub mod shape;
pub mod spectrum;
pub mod texture;

pub mod prelude {
    use cgmath;
    pub use pbrt_proc::*;
    pub use num::Float as NumFloatTrait;
    pub use crate::math::*;
    use super::spectrum;

    pub use crate::spectrum::Spectrum as PbrtSpectrumTrait;
    pub type Spectrum = spectrum::RgbSpectrum;

    pub type Bounds2f = Bounds2<Float>;
    pub type Bounds2i = Bounds2<i32>;
    pub type Bounds3f = Bounds3<Float>;
    pub type Bounds3i = Bounds3<i32>;

    pub type Vector2f = cgmath::Vector2<Float>;
    pub type Vector2i = cgmath::Vector2<i32>;
    pub type Vector3f = cgmath::Vector3<Float>;
    pub type Vector3i = cgmath::Vector3<i32>;

    pub type Point2f = cgmath::Point2<Float>;
    pub type Point2i = cgmath::Point2<i32>;
    pub type Point3f = cgmath::Point3<Float>;
    pub type Point3i = cgmath::Point3<i32>;
}
