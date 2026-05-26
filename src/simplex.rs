use crate::{fraction::Fraction, number::Number, tableau::Tableau};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolvePhase {
    Primal,
    Dual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotKind {
    BeforePivot,
    AfterPivot,
}

#[derive(Debug, Clone)]
pub struct SimplexSnapshot {
    pub tableau: Tableau,
    pub estimates: Vec<Number>,
    pub objective: Number,
}

#[derive(Debug, Clone)]
pub struct SolveStep {
    pub phase: SolvePhase,
    pub kind: SnapshotKind,
    pub pivot_row: usize,
    pub pivot_col: usize,
    pub snapshot: SimplexSnapshot,
}

#[derive(Debug, Clone, Default)]
pub struct SolveLog {
    pub steps: Vec<SolveStep>,
}

#[derive(Debug, Clone)]
pub enum SimplexStatus {
    Optimal {
        value: Number,
        vars: Vec<Fraction>,
        log: SolveLog,
    },
    Infeasible {
        log: SolveLog,
    },
    Unbounded {
        log: SolveLog,
    },
}
