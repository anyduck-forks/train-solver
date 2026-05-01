pub mod fraction;
pub mod model;
pub mod number;
pub mod simplex;
pub mod tableau;

pub fn solve(model: &model::Model) -> simplex::SimplexStatus {
    let mut tableau = tableau::Tableau::from_model(model);
    println!("Initial tableau: {:?}", tableau);

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

    let vars = tableau
        .basic_vars
        .iter()
        .zip(tableau.matrix.iter())
        .filter(|&(&i, _)| i < model.variables.len())
        .map(|(&i, row)| (i, row[0]))
        .collect::<Vec<_>>();

    simplex::SimplexStatus::Optimal(vars)
}
