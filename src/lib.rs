use crate::fraction::Fraction;

pub mod fraction;
pub mod model;
pub mod number;
pub mod simplex;
pub mod tableau;
pub mod wasm;

fn snapshot(tableau: &tableau::Tableau) -> simplex::SimplexSnapshot {
    simplex::SimplexSnapshot {
        tableau: tableau.clone(),
        estimates: tableau.esstimates().collect(),
        objective: tableau.objective(),
    }
}

fn push_pivot_snapshot(
    log: &mut simplex::SolveLog,
    tableau: &tableau::Tableau,
    phase: simplex::SolvePhase,
    kind: simplex::SnapshotKind,
    row: usize,
    col: usize,
) {
    log.steps.push(simplex::SolveStep {
        phase,
        kind,
        pivot_row: row,
        pivot_col: col,
        snapshot: snapshot(tableau),
        cut: None,
    });
}

fn push_cut_snapshot(log: &mut simplex::SolveLog, tableau: &tableau::Tableau, rhs: Fraction, coeffs: Vec<Fraction>) {
    log.steps.push(simplex::SolveStep {
        phase: simplex::SolvePhase::Primal,
        kind: simplex::SnapshotKind::Cut,
        pivot_row: 0,
        pivot_col: 0,
        snapshot: snapshot(tableau),
        cut: Some(simplex::Cut { rhs, coeffs }),
    });
}

pub fn solve(model: &model::Model) -> simplex::SimplexStatus {
    const SNAPSHOT_ITERATION_LIMIT: usize = 10;

    let mut tableau = tableau::Tableau::from_model(model);
    let mut log = simplex::SolveLog::default();
    let mut pivot_iteration = 0usize;

    while !tableau.is_optimal() {
        if let Some((row, col)) = tableau.choose_standart_pivot() {
            let should_log_snapshot = pivot_iteration < SNAPSHOT_ITERATION_LIMIT;
            if should_log_snapshot {
                push_pivot_snapshot(
                    &mut log,
                    &tableau,
                    simplex::SolvePhase::Primal,
                    simplex::SnapshotKind::BeforePivot,
                    row,
                    col,
                );
            }
            tableau.pivot(row, col);
            if should_log_snapshot {
                push_pivot_snapshot(
                    &mut log,
                    &tableau,
                    simplex::SolvePhase::Primal,
                    simplex::SnapshotKind::AfterPivot,
                    row,
                    col,
                );
            }
            pivot_iteration += 1;
        } else {
            return simplex::SimplexStatus::Unbounded { log };
        }
    }

    for &i in tableau.basic_vars.iter() {
        if tableau.objective_coef[i].has_m() {
            return simplex::SimplexStatus::Infeasible { log };
        }
    }

    loop {
        // Dual simplex to resolve any infeasibility (introduced by Gomory cuts)
        while !tableau.is_feasible() {
            if let Some(row) = tableau.choose_dual_pivot_row() {
                if let Some(col) = tableau.choose_dual_pivot_col(row) {
                    let should_log_snapshot = pivot_iteration < SNAPSHOT_ITERATION_LIMIT;
                    if should_log_snapshot {
                        push_pivot_snapshot(
                            &mut log,
                            &tableau,
                            simplex::SolvePhase::Dual,
                            simplex::SnapshotKind::BeforePivot,
                            row,
                            col,
                        );
                    }
                    tableau.pivot(row, col);
                    if should_log_snapshot {
                        push_pivot_snapshot(
                            &mut log,
                            &tableau,
                            simplex::SolvePhase::Dual,
                            simplex::SnapshotKind::AfterPivot,
                            row,
                            col,
                        );
                    }
                    pivot_iteration += 1;
                } else {
                    return simplex::SimplexStatus::Infeasible { log };
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
                    let (rhs, coeffs) = tableau.add_gomory_cut(row_idx);
                    let should_log_snapshot = pivot_iteration < SNAPSHOT_ITERATION_LIMIT;
                    if should_log_snapshot {
                        push_cut_snapshot(&mut log, &tableau, rhs, coeffs.clone());
                    }
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
                tableau.matrix[row][0]
            } else {
                Fraction::from(0)
            }
        })
        .collect::<Vec<_>>();

    simplex::SimplexStatus::Optimal {
        value: tableau.objective(),
        vars,
        log,
    }
}
