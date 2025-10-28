use super::{neighborhood::Neighborhood, objective::Objective, problem::Problem};

pub struct EvalMove<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    pub(crate) mv: N::Move,
    pub(crate) value: <P::Objective as Objective>::Unit,
}

impl<P, N> EvalMove<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    pub fn new(mv: N::Move, value: <P::Objective as Objective>::Unit) -> Self {
        Self { mv, value }
    }

    pub fn compose(self, other: Self) -> Self {
        debug_assert_eq!(&self.mv, &other.mv);
        let objective_value = <P::Objective as Objective>::compose(self.value, other.value);
        Self::new(self.mv, objective_value)
    }
}
