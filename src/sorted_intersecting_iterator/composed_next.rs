use std::cmp::Ordering;

pub enum ComposedNext<T>
where
    T: Ord,
{
    OneIteratorConsumed,
    BothYieldedSameValue(Option<T>),
    FirstIteratorYieldedSmaller { next2: Option<T> },
    FirstIteratorYieldedGreater { next1: Option<T> },
}

impl<T> ComposedNext<T>
where
    T: Ord,
{
    pub fn new(next1: Option<T>, next2: Option<T>) -> Self {
        match (next1.as_ref(), next2.as_ref()) {
            (Some(value1), Some(value2)) => match value1.cmp(value2) {
                Ordering::Equal => Self::BothYieldedSameValue(next1),
                Ordering::Greater => Self::FirstIteratorYieldedGreater { next1 },
                Ordering::Less => Self::FirstIteratorYieldedSmaller { next2 },
            },
            _ => Self::OneIteratorConsumed,
        }
    }
}
