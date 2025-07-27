use crate::objective::Objective;
use core::fmt::Debug;

pub trait Problem: 'static + Default + Clone + Copy {
    type ObjectiveUnit: Debug + PartialEq + Eq + PartialOrd + Clone + Copy;

    type Objective: Objective<Unit = Self::ObjectiveUnit>;

    type Solution;
}
