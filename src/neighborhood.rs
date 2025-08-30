use crate::problem::Problem;
use core::fmt::Debug;

pub trait Neighborhood {
    type Problem: Problem;

    type Move: PartialOrd + Eq + Debug;

    fn neighborhood(
        solution: &<Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = Self::Move>;

    fn apply_move(mv: &Self::Move, solution: &mut <Self::Problem as Problem>::Solution);
}
