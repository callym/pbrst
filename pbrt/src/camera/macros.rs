use std::sync::{ Arc, RwLock };
use cg::prelude::*;
use cg::Matrix4;
use prelude::*;
use math::*;
use math::transform::Transform;
use sampler::CameraSample;

macro_rules! projective_camera {
    ($screen_window:ident, $film:ident, $camera_to_screen:ident, $(,)?) => {
        {
            let screen = $screen_window;

            let translate = Matrix4::from_translation(Vector3f::new(
                -screen.min.x,
                -screen.max.y,
                float(0.0)
            ));

            let scale_ndc = Matrix4::from_nonuniform_scale(
            float(1.0) / (screen.max.x - screen.min.x),
            float(1.0) / (screen.min.y - screen.max.y),
            float(1.0)
            );

            let scale_res = Matrix4::from_nonuniform_scale(
                float($film.full_resolution.x as f32),
                float($film.full_resolution.y as f32),
                float(1.0)
            );

            let screen_to_raster = Transform::new(scale_res * scale_ndc * translate);

            let raster_to_camera = Transform::new(
                $camera_to_screen.inverse *
                screen_to_raster.inverse
            );

            (screen_to_raster, raster_to_camera)
        }
    };
}
