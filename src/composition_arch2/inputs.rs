orx_meta::define_queue!(
    queue => [ InputsQueue ; NoInput, Inputs ];
    builder => InputBuilder;
);

pub type Input<I> = Inputs<I, NoInput>;
