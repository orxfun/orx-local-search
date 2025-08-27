use crate::{eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem};

pub trait MoveGenerator<'i>: Default {
    type Problem: Problem;

    type Neighborhood: Neighborhood<Problem = Self::Problem>;

    type Input: Copy;

    fn moves<'a>(
        &'a mut self,
        input: Self::Input,
        solution: &'a <Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a;
}
