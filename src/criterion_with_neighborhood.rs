use crate::{Criterion, Neighborhood, move_generator::MoveGenerator};

pub trait CriterionWithNeighborhood {
    type Criterion: Criterion;

    type Neighborhood: Neighborhood;

    type MoveGenerator<'i>: MoveGenerator<
            'i,
            Neighborhood = Self::Neighborhood,
            Input = <Self::Criterion as Criterion>::Input<'i>,
        >;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i>;
}

pub type InputOf<'i, X> = <<X as CriterionWithNeighborhood>::Criterion as Criterion>::Input<'i>;
