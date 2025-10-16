use crate::{
    composition::{
        criteria::{Criteria, EmptyCriterion, PairOfCrit, SingleCrit},
        sorted_intersecting_iter::SortedIntersectingIter,
    },
    eval_move::EvalMove,
    moves::Moves,
    neighborhood::Neighborhood,
    problem::Problem,
};
use core::marker::PhantomData;

// traits

pub trait MoveGen<'i, P, N>: Default
where
    P: Problem,
    N: Neighborhood<P>,
{
    type X: Criteria<P>;

    type Compose<M>: MoveGen<'i, P, N>
    where
        M: Moves<'i, P, N>;

    fn compose<M>(self, m: M) -> Self::Compose<M>
    where
        M: Moves<'i, P, N>;

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as Criteria<P>>::Input<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a;
}

// empty

#[derive(Default)]
pub struct EmptyMoveGen;

impl<'i, P, N> MoveGen<'i, P, N> for EmptyMoveGen
where
    P: Problem,
    N: Neighborhood<P>,
{
    type X = EmptyCriterion;

    type Compose<M>
        = SingleMoveGen<'i, P, N, M>
    where
        M: Moves<'i, P, N>;

    fn compose<M>(self, m: M) -> Self::Compose<M>
    where
        M: Moves<'i, P, N>,
    {
        SingleMoveGen(m, PhantomData)
    }

    fn moves<'a>(
        &'a mut self,
        _: &'i <Self::X as Criteria<P>>::Input<'i>,
        _: &'a <P as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        core::iter::empty()
    }
}

// single

pub struct SingleMoveGen<'i, P, N, F>(F, PhantomData<&'i (P, N)>)
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>;

impl<'i, P, N, F> Default for SingleMoveGen<'i, P, N, F>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<'i, P, N, F> MoveGen<'i, P, N> for SingleMoveGen<'i, P, N, F>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
{
    type X = SingleCrit<P, F::X>;

    type Compose<M>
        = PairOfMoveGen<'i, P, N, F, SingleMoveGen<'i, P, N, M>>
    where
        M: Moves<'i, P, N>;

    fn compose<M>(self, m: M) -> Self::Compose<M>
    where
        M: Moves<'i, P, N>,
    {
        PairOfMoveGen(self.0, SingleMoveGen(m, PhantomData), PhantomData)
    }

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as Criteria<P>>::Input<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        self.0.moves(input.front(), solution)
    }
}

// pair

pub struct PairOfMoveGen<'i, P, N, F, B>(F, B, PhantomData<&'i (P, N)>)
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
    B: MoveGen<'i, P, N>;

impl<'i, P, N, F, B> Default for PairOfMoveGen<'i, P, N, F, B>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
    B: MoveGen<'i, P, N>,
{
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default())
    }
}

impl<'i, P, N, F, B> MoveGen<'i, P, N> for PairOfMoveGen<'i, P, N, F, B>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
    B: MoveGen<'i, P, N>,
{
    type X = PairOfCrit<P, F::X, B::X>;

    type Compose<M>
        = PairOfMoveGen<'i, P, N, F, B::Compose<M>>
    where
        M: Moves<'i, P, N>;

    fn compose<M>(self, m: M) -> Self::Compose<M>
    where
        M: Moves<'i, P, N>,
    {
        PairOfMoveGen(self.0, self.1.compose(m), PhantomData)
    }

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as Criteria<P>>::Input<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        let m1 = self.0.moves(input.front(), solution);
        let m2 = self.1.moves(input.back(), solution);
        SortedIntersectingIter::new(m1, m2)
    }
}
