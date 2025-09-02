use crate::{
    Moves,
    composition3::{
        criteria::{CriteriaQueue, CriterionOf, CriterionUp, PairOfCriteria, SingleCriterion},
        inputs::{NonEmptyInputsQueue, PairOfInputs, SingleInput},
        sorted_intersecting_iter::SortedIntersectingIter,
    },
    criterion::Criterion,
    eval_move::EvalMove,
    neighborhood::Neighborhood,
    problem::Problem,
};
use core::marker::PhantomData;

pub trait MoveGen<'i, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type I: NonEmptyInputsQueue;

    type X: CriteriaQueue<P, Input<'i> = Self::I>;

    type Compose<M>
    where
        M: Moves<'i, P, N>;

    fn compose<M>(self, move_gen: M) -> Self::Compose<M>
    where
        M: Moves<'i, P, N>;

    fn moves<'a>(
        &'a mut self,
        input: &'i Self::I,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a;
}

// single
pub struct SingleMoveGen<'i, P, N, F>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
{
    f: F,
    phantom: PhantomData<&'i (P, N)>,
}

impl<'i, P, N, F> MoveGen<'i, P, N> for SingleMoveGen<'i, P, N, F>
where
    P: Problem,
    N: Neighborhood<P>,
    F: Moves<'i, P, N>,
{
    type I = SingleInput<<F::X as Criterion<P>>::Input<'i>>;

    type X = SingleCriterion<CriterionOf<P, F::X>, P>;

    type Compose<M>
        = Self
    //PairMoveGen<'i, P, N, F, SingleMoveGen<'i, P, N, M>>
    where
        M: Moves<'i, P, N>;

    fn compose<M>(self, move_gen: M) -> Self::Compose<M>
    where
        M: Moves<'i, P, N>,
    {
        todo!()
    }

    fn moves<'a>(
        &'a mut self,
        input: &'i Self::I,
        solution: &'a <P as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a,
    {
        // self.f.moves(input, solution)
        core::iter::empty()
    }
}

// // pair

// pub struct PairMoveGen<'i, P, N, F, B>
// where
//     P: Problem,
//     N: Neighborhood<P>,
//     F: MoveGen<'i, P, N>,
//     B: MoveGen<'i, P, N>,
// {
//     f: F,
//     b: B,
//     phantom: PhantomData<&'i (P, N)>,
// }

// impl<'i, P, N, F, B> MoveGen<'i, P, N> for PairMoveGen<'i, P, N, F, B>
// where
//     P: Problem,
//     N: Neighborhood<P>,
//     F: MoveGen<'i, P, N>,
//     B: MoveGen<'i, P, N>,
// {
//     type I = PairOfInputs<<F::X as CriterionUp<P>>::Input<'i>, <B::X as CriterionUp<P>>::Input<'i>>;

//     type X = PairOfCriteria<F::X, B::X, P>;

//     type Compose<M>
//         = Self
//     where
//         M: Moves<'i, P, N>;

//     fn compose<M>(self, move_gen: M) -> Self::Compose<M>
//     where
//         M: Moves<'i, P, N>,
//     {
//         todo!()
//     }

//     fn moves<'a>(
//         &'a mut self,
//         input: &'i Self::I,
//         solution: &'a <P as Problem>::Solution,
//     ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
//     where
//         'i: 'a,
//     {
//         let (in1, in2) = input.front_back();
//         let m1 = self.f.moves(in1, solution);
//         let m2 = self.b.moves(in2, solution);
//         SortedIntersectingIter::new(m1, m2)
//     }
// }
