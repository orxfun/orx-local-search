use crate::r#move::Move;

pub trait Criterion {
    type On;

    type Input<'a>;

    type Value;

    type Move: Move<On = Self::On>;

    fn moves<'a>(
        &self,
        object: Self::On,
        input: Self::Input<'a>,
    ) -> impl Iterator<Item = Self::Move>;
}
