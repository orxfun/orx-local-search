pub trait ObjectiveValue {
    type Unit;

    fn reduce(a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
