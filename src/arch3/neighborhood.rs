use crate::{r#move::Move, problem::Problem, symbolic::Symbolic};

pub trait Neighborhood: Symbolic {
    type Problem: Problem;

    type Move: Move<Problem = Self::Problem>;

    fn neighborhood(
        solution: &<Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = Self::Move>;
}
