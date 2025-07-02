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

impl<X: Criterion> LocalSearchResult<X> {
    pub fn is_local_optimum(&self) -> bool {
        matches!(
            self,
            Self::LocalOptimum {
                solution: _,
                value: _
            }
        )
    }

    pub fn into_local_optimum(self) -> Option<(SolutionOf<X>, ObjectiveUnitOf<X>)> {
        match self {
            Self::LocalOptimum { solution, value } => Some((solution, value)),
            _ => None,
        }
    }
}
