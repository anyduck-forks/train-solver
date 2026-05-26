use std::assert_matches;
use train_solver::{
    fraction::Fraction,
    model::{ConstraintType, Model, ObjectiveType},
    simplex::{SnapshotKind, SolveLog},
};

fn assert_pivot_log_shape(log: &SolveLog) {
    assert_eq!(log.steps.len() % 2, 0);
    for pair in log.steps.chunks(2) {
        assert_eq!(pair.len(), 2);
        assert_eq!(pair[0].kind, SnapshotKind::BeforePivot);
        assert_eq!(pair[1].kind, SnapshotKind::AfterPivot);
        assert_eq!(pair[0].phase, pair[1].phase);
        assert_eq!(pair[0].pivot_row, pair[1].pivot_row);
        assert_eq!(pair[0].pivot_col, pair[1].pivot_col);
    }
}

#[test]
fn lab2() {
    let mut model = Model::new(ObjectiveType::Maximize);
    model.add_variable(false, 5);
    model.add_variable(false, 4);

    model.add_constraint(vec![2, 3], ConstraintType::LessEq, 3);
    model.add_constraint(vec![1, 3], ConstraintType::LessEq, 4);
    model.add_constraint(vec![-1, 1], ConstraintType::LessEq, 5);
    model.add_constraint(vec![5, 4], ConstraintType::LessEq, 6);

    let result = train_solver::solve(&model);

    let x1 = Fraction::new(6, 5);
    let x2 = Fraction::from(0);
    let target = Fraction::from(6);

    match result {
        train_solver::simplex::SimplexStatus::Optimal { value, vars, log } => {
            assert_eq!(vars, vec![x1, x2]);
            assert_eq!(value, target.into());
            assert_pivot_log_shape(&log);
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}

#[test]
fn lab3() {
    let mut model = Model::new(ObjectiveType::Minimize);
    model.add_variable(false, -3);
    model.add_variable(false, 2);

    model.add_constraint(vec![1, 2], ConstraintType::GreaterEq, 10);
    model.add_constraint(vec![3, 1], ConstraintType::GreaterEq, 15);

    let result = train_solver::solve(&model);
    assert_matches!(result, train_solver::simplex::SimplexStatus::Unbounded { .. });
}

#[test]
fn lab4() {
    let mut model = Model::new(ObjectiveType::Maximize);
    model.add_variable(false, 10);
    model.add_variable(false, 15);

    model.add_constraint(vec![1, 3], ConstraintType::LessEq, -3);
    model.add_constraint(vec![2, 1], ConstraintType::LessEq, 2);

    let result = train_solver::solve(&model);
    assert_matches!(result, train_solver::simplex::SimplexStatus::Infeasible { .. });
}

#[test]
fn lab6() {
    let mut model = Model::new(ObjectiveType::Maximize);
    model.add_variable(true, 1);
    model.add_variable(true, 1);

    model.add_constraint(vec![2, 1], ConstraintType::LessEq, 18);
    model.add_constraint(vec![1, 2], ConstraintType::LessEq, 16);

    let result = train_solver::solve(&model);

    let x1 = Fraction::from(6);
    let x2 = Fraction::from(5);
    let target = Fraction::from(11);

    match result {
        train_solver::simplex::SimplexStatus::Optimal { value, vars, log } => {
            assert_eq!(vars, vec![x1, x2]);
            assert_eq!(value, target.into());
            assert_pivot_log_shape(&log);
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}

#[test]
fn lab8() {
    let mut model = Model::new(ObjectiveType::Minimize);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);

    model.add_constraint(vec![2, 0, -2, 2], ConstraintType::GreaterEq, 3);
    model.add_constraint(vec![0, 4, -1, 3], ConstraintType::GreaterEq, 8);
    model.add_constraint(vec![-2, -1, 0, 0], ConstraintType::LessEq, -8);
    model.add_constraint(vec![2, 3, 0, 0], ConstraintType::LessEq, 20);

    let result = train_solver::solve(&model);

    let x1 = Fraction::new(17, 6);
    let x2 = Fraction::new(7, 3);
    let x3 = Fraction::new(4, 3);
    let x4 = Fraction::from(0);
    let target = Fraction::from(0);

    match result {
        train_solver::simplex::SimplexStatus::Optimal { value, vars, log } => {
            assert_eq!(vars, vec![x1, x2, x3, x4]);
            assert_eq!(value, target.into());
            assert_pivot_log_shape(&log);
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}


#[test]
fn lab8_maks() {
    let mut model = Model::new(ObjectiveType::Minimize);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);

    model.add_constraint(vec![-2, 0, -1, -1, 1], ConstraintType::LessEq, -1);
    model.add_constraint(vec![0, -2, -2, -1, 1], ConstraintType::LessEq, -2);
    model.add_constraint(vec![1, 2, 0, 0, 0], ConstraintType::LessEq, 16);
    model.add_constraint(vec![1, 1, 0, 0, 0], ConstraintType::Eq, 8);

    let result = train_solver::solve(&model);

    let x1 = Fraction::new(15, 4);
    let x2 = Fraction::new(17, 4);
    let x3 = Fraction::from(0);
    let x4 = Fraction::from(0);
    let x5 = Fraction::new(13, 2);
    let target = Fraction::from(0);

    match result {
        train_solver::simplex::SimplexStatus::Optimal { value, vars, log } => {
            assert_eq!(vars, vec![x1, x2, x3, x4, x5]);
            assert_eq!(value, target.into());
            assert_pivot_log_shape(&log);
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}



#[test]
fn lab8_maks2() {
    let mut model = Model::new(ObjectiveType::Minimize);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);
    model.add_variable(false, 0);

    model.add_constraint(vec![-2, 0, 1, 1, -1], ConstraintType::LessEq, -1);
    model.add_constraint(vec![0, -2, 2, 1, -1], ConstraintType::LessEq, -2);
    model.add_constraint(vec![1, 2, 0, 0, 0], ConstraintType::LessEq, 16);
    model.add_constraint(vec![1, 1, 0, 0, 0], ConstraintType::Eq, 8);

    let result = train_solver::solve(&model);

    let x1 = Fraction::new(15, 4);
    let x2 = Fraction::new(17, 4);
    let x3 = Fraction::from(0);
    let x4 = Fraction::from(0);
    let x5 = Fraction::new(13, 2);
    let target = Fraction::from(0);

    match result {
        train_solver::simplex::SimplexStatus::Optimal { value, vars, log } => {
            assert_eq!(vars, vec![x1, x2, x3, x4, x5]);
            assert_eq!(value, target.into());
            assert_pivot_log_shape(&log);
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}





#[test]
fn labX() {
    let mut model = Model::new(ObjectiveType::Maximize);
    model.add_variable(false, 5);
    model.add_variable(false, 4);


    model.add_constraint(vec![-2, 4], ConstraintType::LessEq, 9);
    model.add_constraint(vec![-9, 3], ConstraintType::LessEq, 3);
    model.add_constraint(vec![-1, 4], ConstraintType::LessEq, 9);
    model.add_constraint(vec![-1, 4], ConstraintType::LessEq, 9);

    let result = train_solver::solve(&model);

    let x1 = Fraction::new(17, 6);
    let x2 = Fraction::new(7, 3);
    let x3 = Fraction::new(4, 3);
    let x4 = Fraction::from(0);
    let target = Fraction::from(0);

    match result {
        train_solver::simplex::SimplexStatus::Optimal { value, vars, log } => {
            assert_eq!(vars, vec![x1, x2, x3, x4]);
            assert_eq!(value, target.into());
            assert_pivot_log_shape(&log);
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}


