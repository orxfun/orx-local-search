use crate::{
    crit::Criterion,
    empty::{EmptyCriterion, EmptyMoveGenerator},
    move_gen::MoveGenerator,
    neighborhood::Neighborhood,
};
use core::marker::PhantomData;

pub struct LocalSearch<'i, N, M>(PhantomData<&'i (N, M)>)
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>;

impl<'i, N, M> Default for LocalSearch<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'i, N> LocalSearch<'i, N, EmptyMoveGenerator<'i, N>>
where
    N: Neighborhood,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
