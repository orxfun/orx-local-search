use crate::{eval_soln::EvalSoln, problem::Problem, symbolic::Symbolic};
use orx_meta::queue::Queue;

pub trait Criterion: Symbolic {
    type Problem: Problem;

    type Input<'i>: Queue + Copy;

    fn evaluate(self, input: Self::Input<'_>, solution: &Soln<Self>) -> EvalSoln<Self::Problem>;
}

type Soln<X> = <<X as Criterion>::Problem as Problem>::Solution;
