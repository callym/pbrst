use std::cmp::max;
use crate::prelude::*;
use super::Filter;

#[derive(Clone, Debug)]
pub struct BoxFilter {
    radius: Vector2f,
    radius_inv: Vector2f,
}

impl BoxFilter {
    pub fn new(radius: Vector2f) -> Self {
        Self {
            radius,
            radius_inv: Vector2f::new(
                float(1.0) / radius.x,
                float(1.0) / radius.y,
            ),
        }
    }
}

impl Filter for BoxFilter {
    fn radius(&self) -> Vector2f {
        self.radius
    }

    fn radius_inv(&self) -> Vector2f {
        self.radius_inv
    }

    fn evaluate(&self, _: Point2f) -> Float {
        float(1.0)
    }
}

#[derive(Clone, Debug)]
pub struct TriangleFilter {
    radius: Vector2f,
    radius_inv: Vector2f,
}

impl TriangleFilter {
    pub fn new(radius: Vector2f) -> Self {
        Self {
            radius,
            radius_inv: Vector2f::new(
                float(1.0) / radius.x,
                float(1.0) / radius.y,
            ),
        }
    }
}

impl Filter for TriangleFilter {
    fn radius(&self) -> Vector2f {
        self.radius
    }

    fn radius_inv(&self) -> Vector2f {
        self.radius_inv
    }

    fn evaluate(&self, p: Point2f) -> Float {
        max(float(0.0), self.radius.x - p.x.abs()) *
        max(float(0.0), self.radius.y - p.y.abs())
    }
}
