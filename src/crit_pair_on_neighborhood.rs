use crate::{CriterionOnNeighborhood, crit::Criterion};

#[derive(Clone, Copy)]
pub struct ComposedCriteriaOnNeighborhood<X1, X2>(X1, X2)
where
    X1: CriterionOnNeighborhood,
    X2: CriterionOnNeighborhood<Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>;
