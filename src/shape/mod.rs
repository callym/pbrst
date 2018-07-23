use std::fmt::Debug;

pub trait Shape: Debug {
    fn reverse_orientation(&self) -> bool;
    fn transform_swaps_handedness(&self) -> bool;
}
