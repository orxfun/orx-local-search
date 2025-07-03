use crate::{
    ComposedCriteria, Neighborhood, ObjectiveValue, move_generator::MoveGenerator, problem::Problem,
};
use orx_meta::queue::{MetaQueue, TupleQueue};

pub trait Criterion: Default + Clone + Copy {
    type Neighborhood: Neighborhood;

    type Input<'i>;

    type MoveGenerator<'i>: MoveGenerator<'i, Neighborhood = Self::Neighborhood, Input = Self::Input<'i>>;

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
        X: Criterion<Neighborhood = Self::Neighborhood>,
    {
        Default::default()
    }

    fn input_builder<'i>(self) -> TupleQueue<Self::InputQueue<'i>> {
        Default::default()
    }
}

pub type SolutionOf<X> =
    <<<X as Criterion>::Neighborhood as Neighborhood>::Problem as Problem>::Solution;

pub type InputOf<'i, X> = <X as Criterion>::Input<'i>;

pub type ObjectiveUnitOf<X> =
    <<<<X as Criterion>::Neighborhood as Neighborhood>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::Unit;
