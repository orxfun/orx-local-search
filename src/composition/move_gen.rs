use crate::{
    composition::{Criteria, CriteriaQueue, InputsQueue, PairOfCriteria, SingleCriterion},
    eval_move::EvalMove,
    moves::Moves,
    neighborhood::Neighborhood,
    problem::Problem,
};
use core::marker::PhantomData;

// traits

pub trait MoveGen<'i>: Default {
    type Neighborhood: Neighborhood;

    type X: Criteria<Problem = <Self::Neighborhood as Neighborhood>::Problem> + CriteriaQueue;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Input<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a;
}

pub trait Queue<'i>: MoveGen<'i> {
    type PushBack<X>: Queue<'i, Neighborhood = Self::Neighborhood>
    where
        X: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Front: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Back: Queue<'i> + MoveGen<'i, Neighborhood = Self::Neighborhood>;

    fn push_back<X>(self, x: X) -> Self::PushBack<X>
    where
        X: Moves<'i, Neighborhood = Self::Neighborhood>;

    fn front(&self) -> &Self::Front;

    fn into_front(self) -> Self::Front;
}

// single

pub struct Single<'i, F>(F, PhantomData<&'i ()>)
where
    F: Moves<'i>;

impl<'i, F> Default for Single<'i, F>
where
    F: Moves<'i>,
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<'i, F> Queue<'i> for Single<'i, F>
where
    F: Moves<'i>,
{
    type PushBack<X>
        = Self
    // TODO
    where
        X: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Front = F;

    type Back = Self;

    fn push_back<X>(self, x: X) -> Self::PushBack<X>
    where
        X: Moves<'i, Neighborhood = Self::Neighborhood>,
    {
        todo!()
    }

    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn into_front(self) -> Self::Front {
        self.0
    }
}

impl<'i, F> MoveGen<'i> for Single<'i, F>
where
    F: Moves<'i>,
{
    type Neighborhood = F::Neighborhood;

    type X = SingleCriterion<F::X>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Input<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        let input = input.into_front();
        self.0.moves(input, solution)
    }
}

// pair

pub struct Pair<'i, F, B>(F, B, PhantomData<&'i ()>)
where
    F: Moves<'i>,
    B: Queue<'i, Neighborhood = F::Neighborhood> + MoveGen<'i, Neighborhood = F::Neighborhood>;

impl<'i, F, B> Default for Pair<'i, F, B>
where
    F: Moves<'i>,
    B: Queue<'i, Neighborhood = F::Neighborhood> + MoveGen<'i, Neighborhood = F::Neighborhood>,
{
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default())
    }
}

impl<'i, F, B> Queue<'i> for Pair<'i, F, B>
where
    F: Moves<'i>,
    B: Queue<'i, Neighborhood = F::Neighborhood> + MoveGen<'i, Neighborhood = F::Neighborhood>,
{
    type PushBack<X>
        = Pair<'i, F, B::PushBack<X>>
    where
        X: Moves<'i, Neighborhood = Self::Neighborhood>;

    type Front = F;

    type Back = B;

    fn push_back<X>(self, x: X) -> Self::PushBack<X>
    where
        X: Moves<'i, Neighborhood = Self::Neighborhood>,
    {
        todo!()
    }

    fn front(&self) -> &Self::Front {
        todo!()
    }

    fn into_front(self) -> Self::Front {
        todo!()
    }
}

impl<'i, F, B> MoveGen<'i> for Pair<'i, F, B>
where
    F: Moves<'i>,
    B: Queue<'i, Neighborhood = F::Neighborhood> + MoveGen<'i, Neighborhood = F::Neighborhood>,
{
    type Neighborhood = F::Neighborhood;

    type X = PairOfCriteria<F::X, B::X>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Input<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        todo!();
        core::iter::empty()
    }
}
