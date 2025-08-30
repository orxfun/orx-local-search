use crate::{neighborhood::Neighborhood, objective::Objective, problem::Problem};

pub struct EvalMove<N: Neighborhood> {
    pub(crate) mv: N::Move,
    pub(crate) value: ObjUnit<N>,
}

impl<N: Neighborhood> EvalMove<N> {
    pub fn new(mv: N::Move, value: ObjUnit<N>) -> Self {
        Self { mv, value }
    }

    pub fn compose(self, other: Self) -> Self {
        debug_assert_eq!(&self.mv, &other.mv);
        let objective_value = <Obj<N> as Objective>::compose(self.value, other.value);
        Self::new(self.mv, objective_value)
    }
}

type Obj<N> = <<N as Neighborhood>::Problem as Problem>::Objective;
type ObjUnit<N> = <Obj<N> as Objective>::Unit;
