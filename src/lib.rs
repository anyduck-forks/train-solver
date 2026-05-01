use crate::fraction::Fraction;

pub mod fraction;
pub mod model;
pub mod number;
pub mod simplex;
pub mod tableau;

pub fn solve(model: &model::Model) -> simplex::SimplexStatus {
    let mut tableau = tableau::Tableau::from_model(model);

    while !tableau.is_optimal() {
        if let Some((row, col)) = tableau.choose_standart_pivot() {
            tableau.pivot(row, col);
        } else {
            return simplex::SimplexStatus::Unbounded;
        }
    }

    for &i in tableau.basic_vars.iter() {
        if tableau.objective_coef[i].has_m() {
            return simplex::SimplexStatus::Infeasible;
        }
    }

    let vars = (0..model.variables.len())
        .map(|i| {
            if let Some(row) = tableau.basic_vars.iter().position(|&j| j == i) {
                return tableau.matrix[row][0];
            } else {
                return Fraction::from(0);
            }
        })
        .enumerate()
        .collect::<Vec<_>>();

    simplex::SimplexStatus::Optimal(tableau.objective(), vars)
}
