use crate::{Criterion, CriterionWithNeighborhood, LocalSearchOnNeighborhood, Neighborhood};

pub struct LocalSearch<X>
where
    X: Criterion,
{
    criterion: X,
}

impl<X> LocalSearch<X>
where
    X: Criterion,
{
    pub fn new(criterion: X) -> Self {
        Self { criterion }
    }

    pub fn with_neighborhood<'i, N>(
        self,
        criterion_with_neighborhood: N,
    ) -> LocalSearchOnNeighborhood<'i, N>
    where
        N: CriterionWithNeighborhood<Criterion = X>,
    {
        LocalSearchOnNeighborhood::new(criterion_with_neighborhood)
    }
}
