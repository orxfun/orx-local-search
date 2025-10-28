use crate::composition::criteria::{Criteria, CriteriaQueue, CriteriaSingle};
use crate::composition::inputs::InputsQueue;
use crate::composition::sorted_intersecting_iter::SortedIntersectingIter;
use crate::core::{EvalMove, Moves, Neighborhood, Problem};
use core::marker::PhantomData;

// traits

pub trait MoveGenQueue<'i, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type X: CriteriaQueue<P>;

    type PushBack<M>: MoveGenQueue<'i, P, N>
    where
        M: Moves<'i, P, N>;

    fn push<M>(self, m: M) -> Self::PushBack<M>
    where
        M: Moves<'i, P, N>;

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as CriteriaQueue<P>>::Input<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a;
}

// single

pub struct MoveGenSingle<'i, P, N, F>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
{
    front: F,
    phantom: PhantomData<&'i (P, N)>,
}

impl<'i, P, N, F> MoveGenSingle<'i, P, N, F>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
{
    pub fn new(front: F) -> Self {
        Self {
            front,
            phantom: PhantomData,
        }
    }
}

impl<'i, P, N, F> MoveGenQueue<'i, P, N> for MoveGenSingle<'i, P, N, F>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
{
    type X = CriteriaSingle<P, F::X>;

    type PushBack<M>
        = MoveGen<'i, P, N, F, MoveGenSingle<'i, P, N, M>>
    where
        M: Moves<'i, P, N>;

    fn push<M>(self, m: M) -> Self::PushBack<M>
    where
        M: Moves<'i, P, N>,
    {
        MoveGen {
            front: self.front,
            back: MoveGenSingle {
                front: m,
                phantom: PhantomData,
            },
            phantom: PhantomData,
        }
    }

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as CriteriaQueue<P>>::Input<'i>,
        solution: &'a <P as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        self.front.moves(input.front(), solution)
    }
}

// multi

pub struct MoveGen<'i, P, N, F, B>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
    B: MoveGenQueue<'i, P, N>,
{
    front: F,
    back: B,
    phantom: PhantomData<&'i (P, N)>,
}

impl<'i, P, N, F, B> MoveGenQueue<'i, P, N> for MoveGen<'i, P, N, F, B>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
    B: MoveGenQueue<'i, P, N>,
{
    type X = Criteria<P, F::X, B::X>;

    type PushBack<M>
        = MoveGen<'i, P, N, F, B::PushBack<M>>
    where
        M: Moves<'i, P, N>;

    fn push<M>(self, m: M) -> Self::PushBack<M>
    where
        M: Moves<'i, P, N>,
    {
        MoveGen {
            front: self.front,
            back: self.back.push(m),
            phantom: PhantomData,
        }
    }

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as CriteriaQueue<P>>::Input<'i>,
        solution: &'a <P as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        let m1 = self.front.moves(input.front(), solution);
        let m2 = self.back.moves(input.back(), solution);
        SortedIntersectingIter::new(m1, m2)
    }
}
