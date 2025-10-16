use crate::objective::Objective;

pub trait Problem: 'static {
    type Objective: Objective;

    type Solution;
}
