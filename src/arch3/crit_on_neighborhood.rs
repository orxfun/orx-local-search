use crate::{
    crit::Criterion, move_gen::MoveGenerator, neighborhood::Neighborhood, problem::Problem,
};

pub trait CriterionOnNeighborhood {
    type Problem: Problem;

    type Criterion: Criterion<Problem = Self::Problem>;

    type Neighborhood: Neighborhood<Problem = Self::Problem>;

    type MoveGenerator<'i>: MoveGenerator<
            'i,
            Problem = Self::Problem,
            Neighborhood = Self::Neighborhood,
            Input = <Self::Criterion as Criterion>::Input<'i>,
        >;
}
