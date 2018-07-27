use interaction::SurfaceInteraction;
use super::Texture;

pub struct ConstantTexture<T: Clone> {
    value: T,
}

impl<T: Clone> ConstantTexture<T> {
    pub fn new(value: impl Into<T>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl<T: Clone> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _: &SurfaceInteraction) -> T {
        self.value.clone()
    }
}
