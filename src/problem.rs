use crate::obj::Objective;

pub trait Problem: 'static + Default + Clone + Copy {
    type ObjectiveValue: Objective;

    type Solution;
}
