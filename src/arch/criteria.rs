use crate::{
    criterion::Criterion,
    eval_soln::EvalSoln,
    inputs::{Inputs, NoInput},
    objective::Objective,
    problem::Problem,
};

orx_meta::define_queue!(
    lt => [];
    generics => [ P: Problem ];
    elements => [ Criterion<P> ];
    queue => [ CriteriaQueue ; NoCriteria, Criteria ];
    queue_of => criteria_of;
    builder => CriteriaBuilder;
);

pub type Criteria1<P, X> = Criteria<P, X, NoCriteria<P>>;

impl<P> Criterion<P> for NoCriteria<P>
where
    P: Problem,
{
    type Input<'i> = NoInput;

    fn evaluate(_: &Self::Input<'_>, _: &<P as Problem>::Solution) -> EvalSoln<P> {
        EvalSoln::Feasible(<P::Objective as Objective>::identity())
    }
}

impl<P, F, B> Criterion<P> for Criteria<P, F, B>
where
    P: Problem,
    F: Criterion<P>,
    B: CriteriaQueue<P>,
{
    type Input<'i> = Inputs<F::Input<'i>, B::Input<'i>>;

    fn evaluate(input: &Self::Input<'_>, solution: &<P as Problem>::Solution) -> EvalSoln<P> {
        let (in1, in2) = (input.front(), input.back());
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
