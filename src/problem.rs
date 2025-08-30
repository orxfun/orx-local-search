use crate::objective::Objective;

pub trait Problem {
    type Objective: Objective;

    type Solution;
}
