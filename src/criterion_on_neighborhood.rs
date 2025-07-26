use crate::{criterion::Criterion, move_generator::MoveGenerator, neighborhood::Neighborhood};

pub trait CriterionOnNeighborhood: Default + Clone + Copy {
    type Criterion: Criterion;

    type Neighborhood: Neighborhood;

    type MoveGenerator<'i>: MoveGenerator;
}
