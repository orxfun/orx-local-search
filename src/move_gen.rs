use crate::{eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem};

pub trait MoveGenerator<'i> {
    type Problem: Problem;

    type Neighborhood: Neighborhood<Problem = Self::Problem>;

    type Input;

    fn moves<'a>(
        &'a mut self,
        solution: &'a <Self::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a;
}
