use crate::{r#move::Move, objective_value::ObjectiveValue};

pub trait Problem {
    type Solution;

    type ObjectiveValue: ObjectiveValue;

    type Move: Move;

    fn neighborhood(solution: &Self::Solution) -> impl Iterator<Item = Self::Move>;
}

pub type ObjectiveUnitOf<P> = <<P as Problem>::ObjectiveValue as ObjectiveValue>::Unit;

pub type MoveOf<P> = <P as Problem>::Move;
