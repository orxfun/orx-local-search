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
