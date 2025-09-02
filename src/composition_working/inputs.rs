use orx_meta::define_queue;

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
    InputBuilder
);
