use train_solver::{fraction::Fraction, model::{ConstraintType, Model, ObjectiveType}};
use std::assert_matches;

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
        train_solver::simplex::SimplexStatus::Optimal(value, vars) => {
           assert_eq!(vars, vec![(0, x1), (1, x2)]);
           assert_eq!(value, target.into());
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
    assert_matches!(result, train_solver::simplex::SimplexStatus::Unbounded);
}

#[test]
fn lab4() {
    let mut model = Model::new(ObjectiveType::Maximize);
    model.add_variable(false, 10);
    model.add_variable(false, 15);

    model.add_constraint(vec![1, 3], ConstraintType::LessEq, -3);
    model.add_constraint(vec![2, 1], ConstraintType::LessEq, 2);


    let result = train_solver::solve(&model);
    assert_matches!(result, train_solver::simplex::SimplexStatus::Infeasible);
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
        train_solver::simplex::SimplexStatus::Optimal(value, vars) => {
           assert_eq!(vars, vec![(0, x1), (1, x2), (2, x3), (3, x4)]);
           assert_eq!(value, target.into());
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}
