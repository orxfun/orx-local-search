use crate::{
    composition::{
        criteria::{Criteria, PairOfCrit, SingleCrit},
        inputs::NonEmptyInputsQueue,
        sorted_intersecting_iter::SortedIntersectingIter,
    },
    eval_move::EvalMove,
    moves::Moves,
    neighborhood::Neighborhood,
    problem::Problem,
};
use core::marker::PhantomData;

// traits

pub trait MoveGen<'i>: Default {
    // queue

    type PushBack<M>: MoveGen<
            'i,
            Neighborhood = Self::Neighborhood,
            X = <Self::X as Criteria>::PushBack<'i, M::X>,
        >
    where
        M: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Front: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Back: MoveGen<'i, Neighborhood = Self::Neighborhood>;

    // moves

    type Neighborhood: Neighborhood;

    type X: Criteria<Problem = <Self::Neighborhood as Neighborhood>::Problem>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Input<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a;
}

// single

#[derive(Default)]
pub struct SingleMoveGen<'i, F>(F, PhantomData<&'i ()>)
where
    F: Moves<'i>;

impl<'i, F> MoveGen<'i> for SingleMoveGen<'i, F>
where
    F: Moves<'i>,
{
    type PushBack<M>
        = PairOfMoveGen<'i, F, SingleMoveGen<'i, M>>
    where
        M: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Front = F;

    type Back = Self;

    type Neighborhood = F::Neighborhood;

    type X = SingleCrit<F::X>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Input<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        self.0.moves(input.into_front(), solution)
    }
}

// pair

#[derive(Default)]
pub struct PairOfMoveGen<'i, F, B>(F, B, PhantomData<&'i ()>)
where
    F: Moves<'i>,
    B: MoveGen<'i, Neighborhood = F::Neighborhood>;

impl<'i, F, B> MoveGen<'i> for PairOfMoveGen<'i, F, B>
where
    F: Moves<'i>,
    B: MoveGen<'i, Neighborhood = F::Neighborhood>,
{
    type PushBack<M>
        = PairOfMoveGen<'i, F, B::PushBack<M>>
    where
        M: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Front = F;

    type Back = B;

    type Neighborhood = F::Neighborhood;

    type X = PairOfCrit<F::X, B::X>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Input<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        let (in1, in2) = input.pop_front();
        let m1 = self.0.moves(in1, solution);
        let m2 = self.1.moves(in2, solution);
        SortedIntersectingIter::new(m1, m2)
    }
}
