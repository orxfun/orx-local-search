use crate::composition::input_queue::{EmptyInputs, Inputs, InputsQueue};
use crate::{Criterion, EvalSoln, Objective, Problem};

pub trait CriterionX<P>
where
    P: Problem,
{
    type Inputs<'i>: InputsQueue;

    fn evaluate(input: &Self::Inputs<'_>, solution: &P::Solution) -> EvalSoln<P>;
}

orx_meta::define_queue!(
    lt => [];
    generics => [ P: Problem ];
    elements => [ CriterionX<P> ];
    queue => [ CriteriaQueue ; EmptyCriteria, Criteria ];
    queue_of => criteria_of;
    builder => CriteriaBuilder;
);

impl<P> CriterionX<P> for EmptyCriteria<P>
where
    P: Problem,
{
    type Inputs<'i> = EmptyInputs;

    fn evaluate(_: &Self::Inputs<'_>, _: &<P as Problem>::Solution) -> EvalSoln<P> {
        EvalSoln::Feasible(<P::Objective as Objective>::identity())
    }
}

impl<P, F, B> CriterionX<P> for Criteria<P, F, B>
where
    P: Problem,
    F: CriterionX<P>,
    B: CriteriaQueue<P>,
{
    type Inputs<'i> = Inputs<F::Inputs<'i>, B::Inputs<'i>>;

    fn evaluate(inputs: &Self::Inputs<'_>, solution: &<P as Problem>::Solution) -> EvalSoln<P> {
        let (in1, in2) = (inputs.front(), inputs.back());
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
