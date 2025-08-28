use crate::{eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem};

pub trait MoveGenerator<'i>: Default {
    type Neighborhood: Neighborhood;

    type Input: Copy;

    fn moves<'a>(
        &'a mut self,
        input: Self::Input,
        solution: &'a Sol<Self::Neighborhood>,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a;
}

type Sol<N> = <<N as Neighborhood>::Problem as Problem>::Solution;
