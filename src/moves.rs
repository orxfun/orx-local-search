use crate::{
    criteria::Criteria, eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem,
};

pub trait Moves<'i>: Default {
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
