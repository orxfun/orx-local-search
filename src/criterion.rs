use crate::{ComposedCriteria, ObjectiveValue, move_generator::MoveGenerator, problem::Problem};
use orx_meta::queue::{MetaQueue, TupleQueue};

pub trait Criterion: Default + Clone + Copy {
    type Problem: Problem;

    type Input;

    type MoveGenerator: MoveGenerator<Problem = Self::Problem, Input = Self::Input>;

    type InputQueue: MetaQueue;

    type ComposeWith<X>: Criterion
    where
        X: Criterion<Problem = Self::Problem>;

    fn new() -> Self {
        Self::default()
    }

    fn move_generator(self) -> Self::MoveGenerator;

    fn evaluate(
        self,
        solution: &SolutionOf<Self>,
        input: &Self::Input,
    ) -> Option<ObjectiveUnitOf<Self>>;

    fn compose<X>(self, _with: X) -> ComposedCriteria<Self, X>
    where
        X: Criterion<Problem = Self::Problem>,
    {
        Default::default()
    }

    fn input_builder(self) -> TupleQueue<Self::InputQueue> {
        Default::default()
    }
}

pub type SolutionOf<X> = <<X as Criterion>::Problem as Problem>::Solution;

pub type InputOf<X> = <X as Criterion>::Input;

pub type ObjectiveUnitOf<X> =
    <<<X as Criterion>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::Unit;
