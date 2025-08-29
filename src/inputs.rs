pub trait Inputs: Copy {
    type Front: Copy;

    type Back: Inputs;

    type PushBack<X: Copy>: Inputs;

    fn push_back<X: Copy>(self, x: X) -> Self::PushBack<X>;

    fn front(self) -> Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}

#[derive(Clone, Copy)]
pub struct Input<I: Copy>(I);

impl<I: Copy> Inputs for Input<I> {
    type Front = I;

    type Back = Self;

    type PushBack<X: Copy> = InputPair<I, Input<X>>;

    fn push_back<X: Copy>(self, x: X) -> Self::PushBack<X> {
        InputPair(self.0, Input(x))
    }

    fn front(self) -> Self::Front {
        self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        panic!("need to eliminate this call, define empty inputs");
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct InputPair<I: Copy, J: Inputs>(I, J);

impl<I: Copy, J: Inputs> Inputs for InputPair<I, J> {
    type Front = I;

    type Back = J;

    type PushBack<X: Copy> = InputPair<I, J::PushBack<X>>;

    fn push_back<X: Copy>(self, x: X) -> Self::PushBack<X> {
        InputPair(self.0, self.1.push_back(x))
    }

    fn front(self) -> Self::Front {
        self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}
