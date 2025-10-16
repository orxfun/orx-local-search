use crate::{objective::Objective, problem::Problem};
use core::fmt::Debug;

type ObjUnit<P> = <<P as Problem>::Objective as Objective>::Unit;

pub enum EvalSoln<P: Problem> {
    Infeasible,
    Feasible(ObjUnit<P>),
}

impl<P: Problem> PartialEq for EvalSoln<P> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Feasible(l0), Self::Feasible(r0)) => l0 == r0,
            (Self::Infeasible, Self::Infeasible) => true,
            _ => false,
        }
    }
}

impl<P: Problem> Eq for EvalSoln<P> {}

impl<P: Problem> Debug for EvalSoln<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Infeasible => write!(f, "Infeasible"),
            Self::Feasible(arg0) => f.debug_tuple("Feasible").field(arg0).finish(),
        }
    }
}
