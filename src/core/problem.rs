use super::objective::Objective;

pub trait Problem: 'static {
    type Objective: Objective;

    type Solution;
}
