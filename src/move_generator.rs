use crate::{
    cand_move::CandidateMove, criterion::Criterion,
    criterion_on_neighborhood::CriterionOnNeighborhood, r#move::Move, neighborhood::Neighborhood,
    obj::Objective, problem::Problem,
};

pub trait MoveGenerator {
    type Problem: Problem;

    type Criterion: Criterion<Problem = Self::Problem>;

    type Neighborhood: Neighborhood<Problem = Self::Problem>;

    type Move: Move<Problem = Self::Problem>;

    fn new() -> Self;

    fn moves<'a>(
        &'a mut self,
        solution: &'a <Self::Problem as Problem>::Solution,
        input: &'a <Self::Criterion as Criterion>::Input,
    ) -> impl Iterator<Item = CandidateMove<Self::Move, <Self::Problem as Problem>::Objective>> + 'a;
}
