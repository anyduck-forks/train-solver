use crate::fraction::Fraction;

pub mod fraction;
pub mod model;
pub mod number;
pub mod simplex;
pub mod tableau;

pub fn solve(model: &model::Model) -> simplex::SimplexStatus {
    let mut tableau = tableau::Tableau::from_model(model);

    println!("Initial tableau: {:?}", tableau);
    println!("Initial estimates: {:?}", tableau.esstimates().collect::<Vec<_>>());

    while !tableau.is_optimal() {
        if let Some((row, col)) = tableau.choose_standart_pivot() {
            println!("Pivoting on row {}, col {}", row, col);
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

    loop {
        // Dual simplex to resolve any infeasibility (introduced by Gomory cuts)
        while !tableau.is_feasible() {
            if let Some(row) = tableau.choose_dual_pivot_row() {
                if let Some(col) = tableau.choose_dual_pivot_col(row) {
                    tableau.pivot(row, col);
                } else {
                    return simplex::SimplexStatus::Infeasible;
                }
            } else {
                break;
            }
        }

        let mut cut_added = false;

        for (var_idx, _) in model
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_integer)
        {
            if let Some(row_idx) = tableau.basic_vars.iter().position(|&j| j == var_idx) {
                let val = tableau.matrix[row_idx][0];
                if !val.is_integer() {
                    tableau.add_gomory_cut(row_idx);
                    cut_added = true;
                    break;
                }
            }
        }

        if !cut_added {
            break;
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
