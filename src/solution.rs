use crate::{obj::Objective, problem::Problem};

pub enum Solution<P: Problem> {
    LocalOptimum {
        solution: P::Solution,
        value: <P::Objective as Objective>::Unit,
    },
    Feasible {
        solution: P::Solution,
        value: <P::Objective as Objective>::Unit,
    },
    Infeasible {
        solution: P::Solution,
    },
}
