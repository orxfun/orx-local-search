use crate::{
    EvalSoln, Objective, Problem,
    composition2::inputs::{
        EmptyInputs, InputsQueue, NonEmptyInputsQueue, PairOfInputs, SingleInput,
    },
};
use orx_meta::define_queue;

pub trait Criteria<P>
where
    P: Problem,
{
    type Input<'i>: InputsQueue;

    fn evaluate(input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P>;
}

define_queue!(
    lifetimes => [];
    generics => [P: Problem];
    elements => [Criteria<P>];
    names => {
        traits: {
            queue: CritsQueue,
            non_empty_queue: NonEmptyCritsQueue,
        },
        structs: {
            empty: EmptyCrits,
            single: SingleCrit,
            pair: PairOfCrits,
            composition: CritComposition,
            builder: CritBuilder,
        },
    };
);

impl<P: Problem> Criteria<P> for EmptyCrits<P> {
    type Input<'i> = EmptyInputs;
    fn evaluate(_: &Self::Input<'_>, _: &<P as Problem>::Solution) -> EvalSoln<P> {
        EvalSoln::Infeasible
    }
}

impl<P: Problem, F: Criteria<P>> Criteria<P> for SingleCrit<F, P> {
    type Input<'i> = SingleInput<F::Input<'i>>;
    fn evaluate(input: &Self::Input<'_>, solution: &<P as Problem>::Solution) -> EvalSoln<P> {
        F::evaluate(input.front(), solution)
    }
}

impl<P: Problem, F: Criteria<P>, B: CritsQueue<P>> Criteria<P> for PairOfCrits<F, B, P> {
    type Input<'i> = PairOfInputs<F::Input<'i>, B::Input<'i>>;
    fn evaluate(input: &Self::Input<'_>, solution: &<P as Problem>::Solution) -> EvalSoln<P> {
        let (in1, in2) = input.front_back();
        let eval1 = F::evaluate(in1, solution);
        let eval2 = B::evaluate(in2, solution);
        match (eval1, eval2) {
            (EvalSoln::Feasible(val1), EvalSoln::Feasible(val2)) => {
                let val = <P::Objective as Objective>::compose(val1, val2);
                EvalSoln::Feasible(val)
            }
            _ => EvalSoln::Infeasible,
        }
    }
}
