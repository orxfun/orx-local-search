use crate::{eval_soln::EvalSoln, objective::Objective, problem::Problem, symbolic::Symbolic};
use orx_meta::queue::Queue;

pub trait Criterion: Symbolic {
    type Problem: Problem;

    type Input<'i>: Queue + Copy;

    fn evaluate(
        self,
        input: Self::Input<'_>,
        solution: &Solution<Self::Problem>,
    ) -> EvalSoln<Self::Problem>;
}

type ObjUnit<P> = <<P as Problem>::Objective as Objective>::Unit;
type Solution<P> = <P as Problem>::Solution;
