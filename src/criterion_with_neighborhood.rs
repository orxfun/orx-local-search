use crate::{
    ComposedCriteria, Criterion, Neighborhood, ObjectiveValue, move_generator::MoveGenerator,
    problem::Problem,
};
use orx_meta::queue::{MetaQueue, TupleQueue};

pub trait CriterionWithNeighborhood {
    type Criterion: Criterion;

    type Neighborhood: Neighborhood;

    type MoveGenerator<'i>: MoveGenerator<
            'i,
            Neighborhood = Self::Neighborhood,
            Input = <Self::Criterion as Criterion>::Input<'i>,
        >;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i>;

    // fn compose<X>(self, _with: X) -> ComposedCriteria<Self, X>
    // where
    //     X: Criterion<Neighborhood = Self::Neighborhood>,
    // {
    //     Default::default()
    // }
}
