use crate::problem::Problem;

pub trait Criterion: Default + Clone + Copy {
    type Problem: Problem;

    type Input;

    fn evaluate(
        self,
        input: &Self::Input,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> <Self::Problem as Problem>::Objective;
}
