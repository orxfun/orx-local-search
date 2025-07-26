use crate::{obj::Objective, problem::Problem};

type Soln<P> = <P as Problem>::Solution;
type ObjUnit<P> = <<P as Problem>::Objective as Objective>::Unit;

pub trait Criterion: Default + Clone + Copy {
    type Problem: Problem;

    type Input;

    fn evaluate(
        self,
        input: &Self::Input,
        solution: &Soln<Self::Problem>,
    ) -> ObjUnit<Self::Problem>;
}
