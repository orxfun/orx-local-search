use orx_meta::define_queue;

pub trait Input<'i> {}

impl<'i, I: Copy> Input<'i> for I {}

#[derive(Clone, Copy)]
pub enum Never {}

define_queue!(
    InputsQueue,
    NonEmptyInputsQueue,
    EmptyInputs,
    SingleInput,
    PairOfInputs,
    InputComposition,
    Never,
    InputBuilder,
    Input,
    'i
);

impl<'i, F: Copy> Clone for SingleInput<'i, F> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<'i, F: Copy> Copy for SingleInput<'i, F> {}

impl<'i, F: Copy, B: InputsQueue<'i> + Copy> Clone for PairOfInputs<'i, F, B> {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2)
    }
}

impl<'i, F: Copy, B: InputsQueue<'i> + Copy> Copy for PairOfInputs<'i, F, B> {}
