use prelude::*;

mod box_triangle;
pub use self::box_triangle::{ BoxFilter, TriangleFilter };

pub trait Filter {
    fn radius(&self) -> Vector2f;

    fn radius_inv(&self) -> Vector2f;

    fn evaluate(&self, p: Point2f) -> Float;
}
