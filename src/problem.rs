use crate::{r#move::Move, objective_value::ObjectiveValue};

pub trait Problem {
    type Solution;

    type ObjectiveValue: ObjectiveValue;

    type Move: Move;

    fn neighborhood(solution: &Self::Solution) -> impl Iterator<Item = Self::Move>;
}
