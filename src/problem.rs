use crate::objective::Objective;

pub trait Problem: 'static + Default + Clone + Copy {
    type Objective: Objective;

    type Solution;
}
