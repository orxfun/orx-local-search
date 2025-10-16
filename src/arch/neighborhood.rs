use crate::problem::Problem;
use core::fmt::Debug;

pub trait Neighborhood<P>: 'static
where
    P: Problem,
{
    type Move: Ord + Eq + Debug;

    fn neighborhood(solution: &P::Solution) -> impl Iterator<Item = Self::Move>;

    fn apply_move(mv: &Self::Move, solution: &mut P::Solution);
}
