use crate::{
    cand_move::CandidateMove, crit::Criterion, r#move::Move, neighborhood::Neighborhood,
    problem::Problem,
};

pub trait MoveGenerator: Default {
    type Criterion: Criterion;

    type Neighborhood: Neighborhood<Solution = SolnOfCrit<Self::Criterion>>;

    fn new() -> Self;

    fn moves<'a>(
        &'a mut self,
        solution: &'a Solution<Self>,
        input: &'a Input<Self>,
    ) -> impl Iterator<Item = CandidateMove<Mv<Self>, Obj<Self>>> + 'a;
}

type Solution<M> = <<<M as MoveGenerator>::Neighborhood as Neighborhood>::Move as Move>::Solution;
type Input<M> = <<M as MoveGenerator>::Criterion as Criterion>::Input;
type Obj<M> = <<<M as MoveGenerator>::Criterion as Criterion>::Problem as Problem>::Objective;
type Mv<M> = <<M as MoveGenerator>::Neighborhood as Neighborhood>::Move;

type SolnOfCrit<X> = <<X as Criterion>::Problem as Problem>::Solution;
