use crate::{crit::Criterion, eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem};

pub trait MoveGenerator<'i>: Default {
    type Neighborhood: Neighborhood;

    type X: Criterion<Problem = Prob<'i, Self>>;

    fn moves<'a>(
        &'a mut self,
        input: Input<'i, Self>,
        solution: &'a Soln<'i, Self>,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a;
}

type Prob<'i, M> = <<M as MoveGenerator<'i>>::Neighborhood as Neighborhood>::Problem;
type Soln<'i, M> = <Prob<'i, M> as Problem>::Solution;
type Input<'i, M> = <<M as MoveGenerator<'i>>::X as Criterion>::Input<'i>;
