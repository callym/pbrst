#![feature(
    associated_type_defaults,
    const_fn,
    macro_at_most_once_rep,
    nll,
    slice_patterns,
    try_from,
    underscore_imports,
    use_extern_macros,
)]

extern crate atomic;
#[macro_use] extern crate bitflags;
extern crate cgmath as cg;
#[macro_use] extern crate derive_more;
#[macro_use] extern crate derivative;
#[macro_use] extern crate hexf;
extern crate image;
#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;
extern crate noisy_float;
extern crate num;
extern crate num_cpus;
extern crate physical_constants;
extern crate rand;
extern crate rayon;
#[macro_use] extern crate shrinkwraprs;
extern crate xoshiro;

extern crate pbrt_proc;

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
    use cg;
    pub use pbrt_proc::*;
    pub use num::Float as _;
    pub use math::*;
    use super::spectrum;

    pub use spectrum::Spectrum as _;
    pub type Spectrum = spectrum::RgbSpectrum;

    pub type Bounds2f = Bounds2<Float>;
    pub type Bounds2i = Bounds2<i32>;
    pub type Bounds3f = Bounds3<Float>;
    pub type Bounds3i = Bounds3<i32>;

    pub type Vector2f = cg::Vector2<Float>;
    pub type Vector2i = cg::Vector2<i32>;
    pub type Vector3f = cg::Vector3<Float>;
    pub type Vector3i = cg::Vector3<i32>;

    pub type Point2f = cg::Point2<Float>;
    pub type Point2i = cg::Point2<i32>;
    pub type Point3f = cg::Point3<Float>;
    pub type Point3i = cg::Point3<i32>;
}
