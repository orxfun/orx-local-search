use crate::{
    ComposedCriteriaOnNeighborhood, Criterion, Neighborhood, Problem, SolutionOf,
    move_generator::MoveGenerator,
};
pub trait CriterionOnNeighborhood: Default + Clone + Copy {
    type Criterion: Criterion;

    type Neighborhood: Neighborhood<Problem = <Self::Criterion as Criterion>::Problem>;

    type MoveGenerator<'i>: MoveGenerator<
            'i,
            Neighborhood = Self::Neighborhood,
            Input = <Self::Criterion as Criterion>::Input<'i>,
        >;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i>;

    // provided

    fn compose<X>(self, _with: X) -> ComposedCriteriaOnNeighborhood<Self, X>
    where
        X: CriterionOnNeighborhood<Neighborhood = Self::Neighborhood>,
        X::Criterion: Criterion<Problem = <Self::Criterion as Criterion>::Problem>,
    {
        Default::default()
    }

    fn evaluate(
        self,
        solution: &SolutionOf<Self::Criterion>,
        input: &InputOf<'_, Self>,
    ) -> Option<<<Self::Criterion as Criterion>::Problem as Problem>::ObjectiveUnit> {
        Self::Criterion::default().evaluate(solution, input)
    }
}

pub type InputOf<'i, X> = <<X as CriterionOnNeighborhood>::Criterion as Criterion>::Input<'i>;
