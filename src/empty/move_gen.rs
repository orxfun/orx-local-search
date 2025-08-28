use crate::{
    crit::Criterion, eval_move::EvalMove, move_gen::MoveGenerator, neighborhood::Neighborhood,
    problem::Problem,
};
use core::marker::PhantomData;

pub struct EmptyMoveGenerator<'i, N, X>(PhantomData<&'i (N, X)>)
where
    N: Neighborhood,
    X: Criterion<Problem = N::Problem>;

impl<'i, N, X> Default for EmptyMoveGenerator<'i, N, X>
where
    N: Neighborhood,
    X: Criterion<Problem = N::Problem>,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'i, N, X> MoveGenerator<'i> for EmptyMoveGenerator<'i, N, X>
where
    N: Neighborhood,
    X: Criterion<Problem = N::Problem>,
{
    type Neighborhood = N;

    type X = X;

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
