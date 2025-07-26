use core::fmt::Debug;

pub trait Objective: Default + Clone + Copy {
    type Unit: Debug + PartialEq + Eq + PartialOrd + Clone + Copy;

    fn identity() -> Self::Unit;

    fn reduce(a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
