pub trait Criterion {
    type On<'a>;

    type Input<'a>;

    type Value;

    type Move;

    fn moves<'a>(
        &self,
        object: Self::On<'a>,
        input: Self::Input<'a>,
    ) -> impl Iterator<Item = Self::Move>;
}
