use crate::{objective::Objective, symbolic::Symbolic};
use core::fmt::Debug;

pub trait Problem: Symbolic {
    type ObjectiveUnit: Debug + PartialEq + Eq + PartialOrd + Clone + Copy;

    type Objective: Objective<Unit = Self::ObjectiveUnit>;

    type Solution;
}
