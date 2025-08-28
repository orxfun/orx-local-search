use crate::{objective::Objective, problem::Problem};

pub enum EvalSoln<P: Problem> {
    Infeasible,
    Feasible(ObjUnit<P>),
}

type ObjUnit<P> = <<P as Problem>::Objective as Objective>::Unit;
