use crate::{
    composed::ComposedMoveGenerator, empty::EmptyMoveGenerator, move_gen::MoveGenerator,
    neighborhood::Neighborhood,
};
use core::marker::PhantomData;

pub struct LocalSearch<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    move_generator: M,
    phantom: PhantomData<&'i N>,
}

impl<'i, N, M> Default for LocalSearch<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    fn default() -> Self {
        Self {
            move_generator: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<'i, N> LocalSearch<'i, N, EmptyMoveGenerator<'i, N>>
where
    N: Neighborhood,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'i, N, M> LocalSearch<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    fn compose<Q>() -> LocalSearch<'i, N, ComposedMoveGenerator<'i, M, Q>>
    where
        Q: MoveGenerator<'i, Neighborhood = N>,
    {
        Default::default()
    }
}
