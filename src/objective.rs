use core::fmt::Debug;

pub trait Objective {
    type Unit: PartialOrd + Debug;

    fn identity() -> Self::Unit;

    fn compose(a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
