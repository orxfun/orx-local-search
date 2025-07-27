use crate::objective_value::ObjectiveValue;

pub trait Problem: 'static + Default + Clone + Copy {
    type ObjectiveValue: ObjectiveValue;

    type Solution;
}
