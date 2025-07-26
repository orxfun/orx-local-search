use crate::{
    criterion::Criterion, move_generator::MoveGenerator, neighborhood::Neighborhood,
    problem::Problem,
};

pub trait CriterionOnNeighborhood: Default + Clone + Copy {
    type Problem: Problem;

    type Criterion: Criterion<Problem = Self::Problem>;

    type Neighborhood: Neighborhood<Problem = Self::Problem>;

    type MoveGenerator<'i>: MoveGenerator<Problem = Self::Problem>;
}
