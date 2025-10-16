use orx_meta::define_queue;

define_queue!(
    queue => [ InputsQueue ; EmptyInputs, PairOfInputs ];
    builder => InputBuilder;
);
