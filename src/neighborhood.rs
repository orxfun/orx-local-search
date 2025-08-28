use crate::{problem::Problem, symbolic::Symbolic};

pub trait Neighborhood: Symbolic {
    type Problem: Problem;

    type Move;

    fn neighborhood(
        solution: &<Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = Self::Move>;

    fn apply_move(mv: &Self::Move, solution: &mut <Self::Problem as Problem>::Solution);
}
