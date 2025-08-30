use crate::symbolic::Symbolic;
use core::fmt::Debug;

pub trait Objective: Symbolic {
    type Unit: Debug + PartialEq + Eq + PartialOrd + Clone + Copy;

    fn identity() -> Self::Unit;

    fn compose(a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
