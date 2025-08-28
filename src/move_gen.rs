use crate::{crit::Criterion, eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem};

pub trait MoveGenerator<'i>: Default {
    type Neighborhood: Neighborhood;

    type X: Criterion<Problem = <Self::Neighborhood as Neighborhood>::Problem>;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criterion>::Input<'i>,
        solution: &'a <<Self::X as Criterion>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a;
}
