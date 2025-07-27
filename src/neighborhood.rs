use crate::{CandidateMove, Problem, r#move::Move};

pub trait Neighborhood: 'static + Default + Clone + Copy {
    type Problem: Problem;

    type Move: Move<Solution = <Self::Problem as Problem>::Solution>;

    fn neighborhood(solution: &<Self::Move as Move>::Solution) -> impl Iterator<Item = Self::Move>;
}

pub type CandidateMoveOf<N> =
    CandidateMove<<N as Neighborhood>::Move, <<N as Neighborhood>::Problem as Problem>::Objective>;
