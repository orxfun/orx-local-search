use crate::{
    EvalMove, Neighborhood, Problem,
    composition::{
        criteria_queue::{Criteria, CriteriaQueue, CriterionX, EmptyCriteria},
        sorted_intersecting_iter::SortedIntersectingIter,
    },
};

pub trait MovesX<'i, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type X: CriteriaQueue<P>;

    fn moves<'a>(
        &'a mut self,
        inputs: &'i <Self::X as CriterionX<P>>::Inputs<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a;
}

orx_meta::define_queue!(
    lt => ['i];
    generics => [ P: Problem, N: Neighborhood<P> ];
    elements => [ MovesX<'i, P, N> ];
    queue => [ MovesQueue ; EmptyMoves, AllMoves ];
    queue_of => moves_of;
    builder => MovesBuilder;
);

impl<'i, P, N> MovesX<'i, P, N> for EmptyMoves<'i, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type X = EmptyCriteria<P>;

    fn moves<'a>(
        &'a mut self,
        inputs: &'i <Self::X as CriterionX<P>>::Inputs<'i>,
        solution: &'a <P as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        core::iter::empty()
    }
}

impl<'i, P, N, F, B> MovesX<'i, P, N> for AllMoves<'i, P, N, F, B>
where
    P: Problem,
    N: Neighborhood<P>,
    F: MovesX<'i, P, N>,
    B: MovesQueue<'i, P, N>,
{
    type X = Criteria<P, F::X, B::X>;

    fn moves<'a>(
        &'a mut self,
        inputs: &'i <Self::X as CriterionX<P>>::Inputs<'i>,
        solution: &'a <P as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        let m1 = self.f.moves(inputs.front(), solution);
        let m2 = self.b.moves(inputs.back(), solution);
        SortedIntersectingIter::new(m1, m2)
    }
}
