use crate::{r#move::Move, objective_value::ObjectiveValue};

pub trait Problem: 'static {
    type ObjectiveValue: ObjectiveValue;

    type Move: Move;

    fn neighborhood<'a, 'b, 'c>(
        solution: &'b <Self::Move as Move>::Solution,
    ) -> impl Iterator<Item = Self::Move> + 'a + 'b + 'c;
}
