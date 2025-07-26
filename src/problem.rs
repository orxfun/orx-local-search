use crate::obj::Objective;

pub trait Problem: 'static + Default + Clone + Copy {
    type Objective: Objective;

    type Solution;
}
