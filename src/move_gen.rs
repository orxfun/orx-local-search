use crate::{
    criteria::{Criteria, PairX, SingleX},
    eval_move::EvalMove,
    inputs::Inputs,
    moves::Moves,
    neighborhood::Neighborhood,
    problem::Problem,
    sorted_intersecting_iter::SortedIntersectingIter,
};
use core::marker::PhantomData;

pub trait MoveGenerator<'i>: Default {
    type Neighborhood: Neighborhood;

    type X: Criteria<Problem = <Self::Neighborhood as Neighborhood>::Problem>;

    type Front: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Back: MoveGenerator<'i, Neighborhood = Self::Neighborhood>;

    type PushBack<Y>: MoveGenerator<'i, Neighborhood = Self::Neighborhood>
    where
        Y: Moves<'i, Neighborhood = Self::Neighborhood>;

    fn push_back<Y>(self, x: Y) -> Self::PushBack<Y>
    where
        Y: Moves<'i, Neighborhood = Self::Neighborhood>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Inputs<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a;
}

#[derive(Default)]
pub struct SingleM<'i, F>(F, PhantomData<&'i ()>)
where
    F: Moves<'i>;

impl<'i, F> MoveGenerator<'i> for SingleM<'i, F>
where
    F: Moves<'i>,
{
    type Neighborhood = F::Neighborhood;

    type Back = Self;

    type PushBack<Y>
        = PairM<'i, F, SingleM<'i, Y>>
    where
        Y: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Front = F;

    type X = SingleX<F::X>;

    fn push_back<Y>(self, x: Y) -> Self::PushBack<Y>
    where
        Y: Moves<'i, Neighborhood = Self::Neighborhood>,
    {
        PairM(self.0, SingleM(x, PhantomData), PhantomData)
    }

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Inputs<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        self.0.moves(input.front(), solution)
    }
}

#[derive(Default)]
pub struct PairM<'i, F, B>(F, B, PhantomData<&'i ()>)
where
    F: Moves<'i>,
    B: MoveGenerator<'i, Neighborhood = F::Neighborhood>;

impl<'i, F, B> MoveGenerator<'i> for PairM<'i, F, B>
where
    F: Moves<'i>,
    B: MoveGenerator<'i, Neighborhood = F::Neighborhood>,
{
    type Neighborhood = F::Neighborhood;

    type Front = F;

    type Back = B;

    type PushBack<Y>
        = PairM<'i, F, B::PushBack<Y>>
    where
        Y: Moves<'i, Neighborhood = Self::Neighborhood>;

    type X = PairX<F::X, B::X>;

    fn push_back<Y>(self, x: Y) -> Self::PushBack<Y>
    where
        Y: Moves<'i, Neighborhood = Self::Neighborhood>,
    {
        PairM(self.0, self.1.push_back(x), PhantomData)
    }

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Inputs<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        let (in1, in2) = input.pop_front();

        let moves1 = self.0.moves(in1, solution);
        let moves2 = self.1.moves(in2, solution);
        SortedIntersectingIter::new(moves1, moves2)
    }
}
