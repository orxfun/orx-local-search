use orx_meta::define_non_empty_queue;

pub trait Input<'i> {}

impl<'i, I: Copy> Input<'i> for I {}

define_non_empty_queue!(
    InputsQueue,
    MultiInputsQueue,
    SingleInput,
    PairOfInputs,
    InputComposition,
    Input,
    'i
);

impl<'i, F: Copy> Clone for SingleInput<'i, F> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<'i, F: Copy> Copy for SingleInput<'i, F> {}

impl<'i, F: Copy, B: InputsQueue<'i> + Copy> Clone for PairOfInputs<'i, F, B> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<'i, F: Copy, B: InputsQueue<'i> + Copy> Copy for PairOfInputs<'i, F, B> {}
