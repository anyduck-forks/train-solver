use crate::{fraction::Fraction, number::Number};
use crate::tableau::Tableau;

#[derive(Debug, Clone)]
pub enum SimplexStatus {
    Optimal(Number, Vec<(usize, Fraction)>),
    Infeasible,
    Unbounded,
}


