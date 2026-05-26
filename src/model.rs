use crate::fraction::Fraction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveType {
    Maximize,
    Minimize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    LessEq,
    Eq,
    GreaterEq,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub is_integer: bool,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub coeffs: Vec<Fraction>,
    pub relation: ConstraintType,
    pub rhs: Fraction,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub variables: Vec<Variable>,
    pub objective_type: ObjectiveType,
    pub objective_coeffs: Vec<Fraction>,
    pub constraints: Vec<Constraint>,
}

impl Model {
    pub fn new(objective_type: ObjectiveType) -> Self {
        Model {
            variables: Vec::new(),
            objective_type,
            objective_coeffs: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_variable<T: Into<Fraction>>(&mut self, is_integer: bool, obj_coeff: T) -> usize {
        let index = self.variables.len();
        self.variables.push(Variable { is_integer });
        self.objective_coeffs.push(obj_coeff.into());
        index
    }

    pub fn add_constraint<T: Into<Fraction>, I: IntoIterator<Item = T>>(
        &mut self,
        coeffs: I,
        relation: ConstraintType,
        rhs: T,
    ) {
        // assert_eq!(coeffs.collect::<Vec<_>>().len(), self.variables.len(), "Each constraint must have a coefficient for every variable");
        let mut constraint = Constraint {
            coeffs: coeffs.into_iter().map(Into::into).collect(),
            relation,
            rhs: rhs.into(),
        };

        if constraint.rhs < Fraction::from(0) {
            constraint.coeffs.iter_mut().for_each(|c| *c = -*c);
            constraint.rhs = -constraint.rhs;
            constraint.relation = match constraint.relation {
                ConstraintType::LessEq => ConstraintType::GreaterEq,
                ConstraintType::GreaterEq => ConstraintType::LessEq,
                ConstraintType::Eq => ConstraintType::Eq,
            };
        }

        self.constraints.push(constraint);
    }
}
