use crate::{fraction::Fraction, number::Number};

#[derive(Debug, Clone)]
pub enum SimplexStatus {
    Optimal(Number, Vec<(usize, Fraction)>),
    Infeasible,
    Unbounded,
}
