pub trait Symbolic: 'static + Default + Clone + Copy {}

impl<X: 'static + Default + Clone + Copy> Symbolic for X {}
