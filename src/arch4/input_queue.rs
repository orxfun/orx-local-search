use orx_meta::{Never, impl_meta_queue};

impl_meta_queue!(
    [Copy],
    [Copy],
    Never,
    MaybeInputs,
    Inputs,
    EmptyInput,
    SingleInput,
    PairOfInputs
);
