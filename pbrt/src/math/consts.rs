use std::convert::TryFrom;

pub struct TryFromUsizeError(());

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Dim {
    X,
    Y,
    Z,
}

impl Into<usize> for Dim {
    fn into(self) -> usize {
        match self {
            Dim::X => 0,
            Dim::Y => 1,
            Dim::Z => 2,
        }
    }
}

impl TryFrom<usize> for Dim {
    type Error = TryFromUsizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Dim::X),
            1 => Ok(Dim::Y),
            2 => Ok(Dim::Z),
            _ => Err(TryFromUsizeError(())),
        }
    }
}
