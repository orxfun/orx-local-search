use crate::{Criterion, Neighborhood, ObjectiveUnitOf, SolutionOf, move_generator::MoveGenerator};
pub trait CriterionWithNeighborhood: Default + Clone + Copy {
    type Criterion: Criterion;

    type Neighborhood: Neighborhood<Problem = <Self::Criterion as Criterion>::Problem>;

    type MoveGenerator<'i>: MoveGenerator<
            'i,
            Neighborhood = Self::Neighborhood,
            Input = <Self::Criterion as Criterion>::Input<'i>,
        >;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i>;

    // provided from criterion

    fn evaluate(
        self,
        solution: &SolutionOf<Self::Criterion>,
        input: &InputOf<'_, Self>,
    ) -> Option<ObjectiveUnitOf<Self::Criterion>> {
        Self::Criterion::default().evaluate(solution, input)
    }
}

pub type InputOf<'i, X> = <<X as CriterionWithNeighborhood>::Criterion as Criterion>::Input<'i>;
