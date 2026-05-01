use train_solver::{fraction::Fraction, model::{ConstraintType, Model, ObjectiveType}};


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

    match result {
        train_solver::simplex::SimplexStatus::Optimal(vars) => {
           assert_eq!(vars, vec![(0, x1), (1, x2), (2, x3)]);
        }
        _ => panic!("Expected optimal solution, got {:?}", result),
    }
}
