use crate::{
    criteria::Criteria, eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem,
};
use core::marker::PhantomData;
use orx_meta::queue::NonEmptyQueue;

pub trait MoveGenerator<'i>: Default {
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

#[derive(Default)]
pub struct SingleM<'i, F>(F, PhantomData<&'i ()>)
where
    F: MoveGenerator<'i>;

impl<'i, F> MoveGenerator<'i> for SingleM<'i, F>
where
    F: MoveGenerator<'i>,
{
    type Neighborhood = F::Neighborhood;

    type X = F::X;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criteria>::Input<'i>,
        solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        let a = input;
        let b = input;
        let c = input.front();
        // let input = *input.front();

        core::iter::empty()
    }
}
