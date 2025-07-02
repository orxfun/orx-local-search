use crate::{ComposedCriteria, ObjectiveValue, move_generator::MoveGenerator, problem::Problem};
use orx_meta::queue::{MetaQueue, TupleQueue};

pub trait Criterion: Default + Clone + Copy {
    type Problem: Problem;

    type Input<'i>;

    type MoveGenerator<'i>: MoveGenerator<'i, Problem = Self::Problem, Input = Self::Input<'i>>;

    type InputQueue<'i>: MetaQueue;

    fn new() -> Self {
        Self::default()
    }

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i>;

    fn evaluate(
        self,
        solution: &SolutionOf<Self>,
        input: &Self::Input<'_>,
    ) -> Option<ObjectiveUnitOf<Self>>;

    fn compose<X>(self, _with: X) -> ComposedCriteria<Self, X>
    where
        X: Criterion<Problem = Self::Problem>,
    {
        Default::default()
    }

    fn input_builder<'i>(self) -> TupleQueue<Self::InputQueue<'i>> {
        Default::default()
    }
}

pub type SolutionOf<X> = <<X as Criterion>::Problem as Problem>::Solution;

pub type InputOf<'i, X> = <X as Criterion>::Input<'i>;

pub type ObjectiveUnitOf<X> =
    <<<X as Criterion>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::Unit;
