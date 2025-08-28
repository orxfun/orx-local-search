use crate::symbolic::Symbolic;

pub trait Objective: Symbolic {
    type Unit: PartialEq + Eq + PartialOrd + Clone + Copy;

    fn identity() -> Self::Unit;

    fn reduce(a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
