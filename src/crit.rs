use crate::{obj::Objective, problem::Problem};

pub trait Criterion: Default + Clone + Copy {
    type Input;

    type Solution;

    type Objective: Objective;

    type Problem: Problem<Solution = Self::Solution, Objective = Self::Objective>;

    fn evaluate(
        self,
        input: &Self::Input,
        solution: &Self::Solution,
    ) -> Option<<Self::Objective as Objective>::Unit>;
}
