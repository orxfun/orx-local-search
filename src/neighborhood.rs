use crate::{problem::Problem, symbolic::Symbolic};
use core::fmt::Debug;

pub trait Neighborhood: Symbolic {
    type Problem: Problem;

    type Move: Ord + Eq + Debug;

    fn neighborhood(solution: &Soln<Self>) -> impl Iterator<Item = Self::Move>;

    fn apply_move(mv: &Self::Move, solution: &mut Soln<Self>);
}

type Soln<N> = <<N as Neighborhood>::Problem as Problem>::Solution;
