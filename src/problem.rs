use crate::{CandidateMove, r#move::Move, objective_value::ObjectiveValue};

pub trait Problem: 'static + Default + Clone + Copy {
    type ObjectiveValue: ObjectiveValue;

    type Solution;

    type Move: Move<Solution = Self::Solution>;

    fn neighborhood(solution: &<Self::Move as Move>::Solution) -> impl Iterator<Item = Self::Move>;
}

pub type CandidateMoveOf<P> = CandidateMove<<P as Problem>::Move, <P as Problem>::ObjectiveValue>;
