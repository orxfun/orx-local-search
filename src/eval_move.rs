use crate::{neighborhood::Neighborhood, objective::Objective, problem::Problem};

pub struct EvalMove<N: Neighborhood> {
    pub(crate) mv: N::Move,
    pub(crate) value: <N::Problem as Problem>::ObjectiveUnit,
}

impl<N: Neighborhood> EvalMove<N> {
    pub fn new(mv: N::Move, value: <N::Problem as Problem>::ObjectiveUnit) -> Self {
        Self { mv, value }
    }

    pub fn compose(self, other: Self) -> Self {
        debug_assert_eq!(&self.mv, &other.mv);
        let objective_value =
            <<N::Problem as Problem>::Objective as Objective>::reduce(self.value, other.value);
        Self::new(self.mv, objective_value)
    }
}
