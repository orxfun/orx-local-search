// use orx_meta::define_queue_old;

// #[derive(Clone, Copy)]
// pub enum Never {}

// define_queue_old!(
//     InputsQueue,
//     NonEmptyInputsQueue,
//     EmptyInputs,
//     SingleInput,
//     PairOfInputs,
//     InputComposition,
//     Never,
//     InputBuilder
// );

use orx_meta::define_queue;

define_queue!(
    lifetimes => [];
    generics => [];
    elements => [];
    names => {
        traits: {
            queue: InputsQueue,
            non_empty_queue: NonEmptyInputsQueue,
        },
        structs: {
            empty: EmptyInputs,
            single: SingleInput,
            pair: PairOfInputs,
            composition: InputComposition,
            builder: InputBuilder,
        },
    };
);
