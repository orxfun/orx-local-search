use crate::{Criterion, ObjectiveUnitOf, SolutionOf};

pub enum LocalSearchResult<X: Criterion> {
    LocalOptimum {
        solution: SolutionOf<X>,
        value: ObjectiveUnitOf<X>,
    },
    InfeasibleInitialSolution {
        initial_solution: SolutionOf<X>,
    },
}
