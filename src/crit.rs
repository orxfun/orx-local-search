use crate::{ComposedCriteria, problem::Problem};

pub trait Criterion: Default + Clone + Copy {
    type Problem: Problem;

    type Input<'i>;

    // type InputQueue<'i>: MetaQueue;

    fn new() -> Self {
        Self::default()
    }

    fn evaluate(
        self,
        solution: &SolutionOf<Self>,
        input: &Self::Input<'_>,
    ) -> Option<<Self::Problem as Problem>::ObjectiveUnit>;

    // fn input_builder<'i>(self) -> TupleQueue<Self::InputQueue<'i>> {
    //     Default::default()
    // }

    fn compose<X>(self, _with: X) -> ComposedCriteria<Self, X>
    where
        X: Criterion<Problem = Self::Problem>,
    {
        Default::default()
    }
}

pub type SolutionOf<X> = <<X as Criterion>::Problem as Problem>::Solution;
