use crate::{objective::Objective, symbolic::Symbolic};

pub trait Problem: Symbolic {
    type Objective: Objective;

    type Solution;
}
