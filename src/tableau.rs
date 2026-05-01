use std::iter;

use crate::{fraction::Fraction, number::Number};
use crate::model::{ConstraintType, Model, ObjectiveType};

#[derive(Debug, Clone)]
pub struct Tableau {
    pub matrix: Vec<Vec<Fraction>>,
    pub basic_vars: Vec<usize>,
    pub objective_coef: Vec<Number>,
    pub objective_type: ObjectiveType,
}

impl Tableau {
    pub fn objective(&self) -> Number {
        let objective = self.esstimates_inner(0);
        match self.objective_type {
            ObjectiveType::Maximize => objective,
            ObjectiveType::Minimize => -objective,
        }
    }

    pub fn esstimates(&self) -> impl Iterator<Item = Number> {
        (0..self.objective_coef.len()).map(|i| self.esstimates_inner(i + 1) - self.objective_coef[i])
    }

    fn esstimates_inner(&self, idx: usize) -> Number {
        let mut sum = Number::from(0);
        for (&index, row) in self.basic_vars.iter().zip(self.matrix.iter()) {
            // FIXME: trait AddAssign for Number to simplify this
            sum = sum + self.objective_coef[index] * row[idx];
        }
        sum
    }

    fn add_slack_variable(&mut self, constraint_index: usize) {
        self.objective_coef.push(Number::from(0));
        for (i, row) in self.matrix.iter_mut().enumerate() {
            let coeff = if i == constraint_index {
                Fraction::from(1)
            } else {
                Fraction::from(0)
            };
            row.push(coeff);
        }
    }

    fn add_surplus_variable(&mut self, constraint_index: usize) {
        self.objective_coef.push(Number::from_m(-1));
        for (i, row) in self.matrix.iter_mut().enumerate() {
            let coeff = if i == constraint_index {
                Fraction::from(-1)
            } else {
                Fraction::from(0)
            };
            row.push(coeff);
        }
    }

    // FIXME: haven't tested it yet, it's 100% wrong
    pub fn add_row(&mut self, coeffs: &[Fraction], rhs: Fraction, constraint_type: ConstraintType) {
        let mut row = vec![rhs];
        row.extend(coeffs);
        if self.matrix.first().is_some() && self.matrix[0].len() > row.len() {
            let count = self.matrix[0].len() - row.len();
            row.extend(iter::repeat_n(Fraction::from(0), count));
        }
        self.matrix.push(row);

        match constraint_type {
            ConstraintType::LessEq => {
                self.add_slack_variable(self.matrix.len() - 1);
                self.basic_vars.push(self.objective_coef.len() - 1);
            }
            ConstraintType::GreaterEq => {
                self.add_slack_variable(self.matrix.len() - 1);
                self.add_surplus_variable(self.matrix.len() - 1);
                self.basic_vars.push(self.objective_coef.len() - 1);
            }
            ConstraintType::Eq => {
                self.add_surplus_variable(self.matrix.len() - 1);
                self.basic_vars.push(self.objective_coef.len() - 1);
            }
        }
    }

    pub fn from_model(model: &Model) -> Self {
        println!("Creating tableau from model");

        let mut matrix = vec![vec![Fraction::from(0); 0]; 0];
        
        let mut objective_coef: Vec<Number> = model.objective_coeffs.iter().map(|&c| Number::from(c)).collect();
        if model.objective_type == ObjectiveType::Minimize {
            objective_coef.iter_mut().for_each(|c| *c = -*c);
        }

        let mut tableau = Self {
            matrix,
            objective_type: model.objective_type,
            objective_coef,
            basic_vars: Vec::new()
        };

        for (i, constraint) in model.constraints.iter().enumerate() {
            let constraint_type = constraint.relation;
            tableau.add_row(&constraint.coeffs, constraint.rhs, constraint_type);
        }

        tableau
    }


    pub fn is_optimal(&self) -> bool {
        self.esstimates().all(|e| e >= 0.into())
    }

    pub fn is_feasible(&self) -> bool {
        self.matrix.iter().all(|row| row[0] >= 0.into())
    }

    fn choose_pivot_row(&self, col: usize) -> Option<usize> {
        let mut leaving_row = None;
        let mut min_ratio = None;

        for (i, row) in self.matrix.iter().enumerate() {
            let coeff = row[col];
            if coeff > 0.into() {
                let ratio = row[0] / coeff; // rhs / coeff
                if min_ratio.is_none_or(|r| ratio < r) {
                    min_ratio = Some(ratio);
                    leaving_row = Some(i);
                }
            }
        }

        leaving_row
    }

    pub fn choose_pivot_col(&self) -> Option<usize> {
        self.esstimates().enumerate().find_map(|(j, e)| if e < 0.into() { Some(j + 1) } else { None })
    }

    pub fn choose_standart_pivot(&self) -> Option<(usize, usize)> {
        let col = self.choose_pivot_col()?;
        let row = self.choose_pivot_row(col)?;
        Some((row, col))
    }

    pub fn pivot(&mut self, row: usize, col: usize) {
        let pivot = self.matrix[row][col];

        let rows = (0..self.matrix.len()).filter(|&i| i != row);

        for i in rows {
            let save = self.matrix[i][col];
            for j in 0..self.matrix[0].len() {
                self.matrix[i][j] = self.matrix[i][j] - self.matrix[row][j] * save / pivot;
            }
        }

        for c in self.matrix[row].iter_mut() {
           *c = *c / pivot;
        }

        let leaving_index = self.basic_vars[row];
        self.basic_vars[row] = col;
        if self.objective_coef[leaving_index].has_m() {
            self.remove_m_variables(leaving_index);
        }
    }

    fn remove_m_variables(&mut self, index: usize) {
        assert!(self.objective_coef[index].has_m(), "Only variables with m can be removed");
        self.objective_coef.remove(index);
        for row in self.matrix.iter_mut() {
            row.remove(index + 1);
        }
        for basic in self.basic_vars.iter_mut() {
            if *basic > index {
                *basic -= 1;
            }
        }
    }
}
