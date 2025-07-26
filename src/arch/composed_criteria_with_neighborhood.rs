use crate::{
    ComposedCriteria, CriterionWithNeighborhood, composed_move_generator::ComposedMoveGenerator,
    criterion::Criterion,
};

#[derive(Clone, Copy)]
pub struct ComposedCriteriaWithNeighborhood<X1, X2>(X1, X2)
where
    X1: CriterionWithNeighborhood,
    X2: CriterionWithNeighborhood<Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>;

impl<X1, X2> Default for ComposedCriteriaWithNeighborhood<X1, X2>
where
    X1: CriterionWithNeighborhood,
    X2: CriterionWithNeighborhood<Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<X1, X2> ComposedCriteriaWithNeighborhood<X1, X2>
where
    X1: CriterionWithNeighborhood,
    X2: CriterionWithNeighborhood<Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>,
{
    pub fn new() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<X1, X2> CriterionWithNeighborhood for ComposedCriteriaWithNeighborhood<X1, X2>
where
    X1: CriterionWithNeighborhood,
    X2: CriterionWithNeighborhood<Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>,
{
    type Criterion = ComposedCriteria<X1::Criterion, X2::Criterion>;

    type Neighborhood = X1::Neighborhood;

    type MoveGenerator<'i> = ComposedMoveGenerator<'i, X1, X2>;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        ComposedMoveGenerator::new(self.0.move_generator(), self.1.move_generator())
    }
}
