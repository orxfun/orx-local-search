use crate::{
    objective_value::ObjectiveValue,
    problem::{MoveOf, ObjectiveUnitOf, Problem},
};

pub struct CandidateMove<P>
where
    P: Problem,
{
    r#move: MoveOf<P>,
    objective_value: ObjectiveUnitOf<P>,
}

impl<P> CandidateMove<P>
where
    P: Problem,
{
    pub fn new(r#move: MoveOf<P>, objective_value: ObjectiveUnitOf<P>) -> Self {
        Self {
            r#move,
            objective_value,
        }
    }

    pub fn compose(self, other: CandidateMove<P>) -> Self {
        debug_assert_eq!(&self.r#move, &other.r#move);
        let objective_value = <P::ObjectiveValue as ObjectiveValue>::reduce(
            self.objective_value,
            other.objective_value,
        );
        Self::new(self.r#move, objective_value)
    }
}
