use crate::{
    crit::Criterion, empty::EmptyCriterion, eval_move::EvalMove, move_gen::MoveGenerator,
    neighborhood::Neighborhood, problem::Problem,
};
use core::marker::PhantomData;

pub struct EmptyMoveGenerator<'i, N>(PhantomData<&'i N>)
where
    N: Neighborhood;

impl<'i, N> Default for EmptyMoveGenerator<'i, N>
where
    N: Neighborhood,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'i, N> MoveGenerator<'i> for EmptyMoveGenerator<'i, N>
where
    N: Neighborhood,
{
    type Neighborhood = N;

    type X = EmptyCriterion<N::Problem>;

    fn moves<'a>(
        &'a mut self,
        _: <Self::X as Criterion>::Input<'i>,
        _: &'a <<Self::X as Criterion>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        // TODO: or should it actually return the neighborhood around solution
        core::iter::empty()
    }
}
