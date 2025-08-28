use crate::{
    composed::{crit::ComposedCriteria, sorted_intersecting_iter::SortedIntersectingIter},
    crit::Criterion,
    eval_move::EvalMove,
    move_gen::MoveGenerator,
    problem::Problem,
};
use core::marker::PhantomData;
use orx_meta::queue::NonEmptyQueue;

#[derive(Default)]
pub struct ComposedMoveGenerator<'i, M1, M2>
where
    M1: MoveGenerator<'i>,
    M2: MoveGenerator<'i, Neighborhood = M1::Neighborhood>,
{
    m1: M1,
    m2: M2,
    phantom: PhantomData<&'i ()>,
}

impl<'i, M1, M2> ComposedMoveGenerator<'i, M1, M2>
where
    M1: MoveGenerator<'i>,
    M2: MoveGenerator<'i, Neighborhood = M1::Neighborhood>,
{
    pub(crate) fn new(m1: M1, m2: M2) -> Self {
        Self {
            m1,
            m2,
            phantom: PhantomData,
        }
    }
}

impl<'i, M1, M2> MoveGenerator<'i> for ComposedMoveGenerator<'i, M1, M2>
where
    M1: MoveGenerator<'i>,
    M2: MoveGenerator<'i, Neighborhood = M1::Neighborhood>,
{
    type Neighborhood = M1::Neighborhood;

    type X = ComposedCriteria<M1::X, M2::X>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criterion>::Input<'i>,
        solution: &'a <<Self::X as Criterion>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        let (input1, input2) = input.pop_front();
        let moves1 = self.m1.moves(input1, solution);
        let moves2 = self.m2.moves(input2, solution);
        SortedIntersectingIter::new(moves1, moves2)
    }
}
