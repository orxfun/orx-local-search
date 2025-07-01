use crate::{ObjectiveValue, move_generator::MoveGenerator, problem::Problem};
use orx_meta::queue::MetaQueue;

pub trait Criterion {
    type Problem: Problem;

    type Input;

    type InputQueue: MetaQueue;

    type MoveGenerator: MoveGenerator<Problem = Self::Problem, Input = Self::Input>;

    fn move_generator() -> Self::MoveGenerator;

    fn evaluate(solution: &SolutionOf<Self>, input: &Self::Input) -> Option<ObjectiveUnitOf<Self>>;
}

pub type SolutionOf<X> = <<X as Criterion>::Problem as Problem>::Solution;

pub type InputOf<X> = <X as Criterion>::Input;

pub type ObjectiveUnitOf<X> =
    <<<X as Criterion>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::Unit;
