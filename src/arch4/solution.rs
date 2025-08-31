use crate::{Objective, problem::Problem};

pub enum Solution<P: Problem> {
    LocalOptimum {
        solution: P::Solution,
        value: <P::Objective as Objective>::Unit,
    },
    FeasibleSolution {
        solution: P::Solution,
        value: <P::Objective as Objective>::Unit,
    },
    InfeasibleSolution {
        solution: P::Solution,
    },
}

impl<P: Problem> Solution<P> {
    pub fn is_feasible(&self) -> bool {
        match self {
            Self::InfeasibleSolution { solution: _ } => false,
            _ => true,
        }
    }

    pub fn is_infeasible(&self) -> bool {
        match self {
            Self::InfeasibleSolution { solution: _ } => true,
            _ => false,
        }
    }

    pub fn is_local_optimum(&self) -> bool {
        matches!(
            self,
            Self::LocalOptimum {
                solution: _,
                value: _
            }
        )
    }

    pub fn into_local_optimum(self) -> Option<(P::Solution, <P::Objective as Objective>::Unit)> {
        match self {
            Self::LocalOptimum { solution, value } => Some((solution, value)),
            _ => None,
        }
    }
}
