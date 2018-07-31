use std::fmt::Debug;

use crate::interaction::SurfaceInteraction;
use super::Texture;

#[derive(Debug)]
pub struct ConstantTexture<T: Clone + Debug> {
    value: T,
}

impl<T: Clone + Debug> ConstantTexture<T> {
    pub fn new(value: impl Into<T>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl<T: Clone + Debug> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _: &SurfaceInteraction<'_>) -> T {
        self.value.clone()
    }
}
