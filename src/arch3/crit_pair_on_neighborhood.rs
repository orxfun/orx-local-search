use crate::{
    CriterionOnNeighborhood, crit_pair::ComposedCriteria, move_gen_pair::ComposedMoveGenerator,
};

#[derive(Clone, Copy)]
pub struct ComposedCriteriaOnNeighborhood<X1, X2>(X1, X2)
where
    X1: CriterionOnNeighborhood,
    X2: CriterionOnNeighborhood<Neighborhood = X1::Neighborhood, Problem = X1::Problem>;

impl<X1, X2> CriterionOnNeighborhood for ComposedCriteriaOnNeighborhood<X1, X2>
where
    X1: CriterionOnNeighborhood,
    X2: CriterionOnNeighborhood<Neighborhood = X1::Neighborhood, Problem = X1::Problem>,
{
    type Problem = X1::Problem;

    type Criterion = ComposedCriteria<X1::Criterion, X2::Criterion>;

    type Neighborhood = X1::Neighborhood;

    type MoveGenerator<'i> = ComposedMoveGenerator<'i, X1, X2>;
}
