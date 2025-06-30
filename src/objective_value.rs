use std::fmt::Debug;

pub trait ObjectiveValue {
    type Unit: Debug + PartialEq + Eq;

    fn identity() -> Self::Unit;

    fn reduce(a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
