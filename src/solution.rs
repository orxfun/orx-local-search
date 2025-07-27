use crate::{ObjectiveValue, Problem};

pub enum Solution<P: Problem> {
    LocalOptimum {
        solution: P::Solution,
        value: <P::ObjectiveValue as ObjectiveValue>::Unit,
    },
    FeasibleSolution {
        solution: P::Solution,
        value: <P::ObjectiveValue as ObjectiveValue>::Unit,
    },
    InfeasibleSolution {
        initial_solution: P::Solution,
    },
}

impl<P: Problem> Solution<P> {
    pub fn is_local_optimum(&self) -> bool {
        matches!(
            self,
            Self::LocalOptimum {
                solution: _,
                value: _
            }
        )
    }

    pub fn into_local_optimum(
        self,
    ) -> Option<(P::Solution, <P::ObjectiveValue as ObjectiveValue>::Unit)> {
        match self {
            Self::LocalOptimum { solution, value } => Some((solution, value)),
            _ => None,
        }
    }
}
