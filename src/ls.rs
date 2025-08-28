use crate::{
    crit::Criterion,
    empty::{EmptyCriterion, EmptyMoveGenerator},
    move_gen::MoveGenerator,
    neighborhood::Neighborhood,
};
use core::marker::PhantomData;
use orx_meta::queue::EmptyQueue;

pub struct LocalSearch<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    input: <M::X as Criterion>::Input<'i>,
}

impl<'i, N> LocalSearch<'i, N, EmptyMoveGenerator<'i, N>>
where
    N: Neighborhood,
{
    pub fn new() -> Self {
        Self { input: EmptyQueue }
    }
}

impl<'i, N, M> LocalSearch<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    fn with<Q>(input: usize) {
        //
    }
}
