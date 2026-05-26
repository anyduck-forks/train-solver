use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    fraction::Fraction,
    model::{ConstraintType as CoreConstraintType, Model as CoreModel, ObjectiveType as CoreObjectiveType},
    number::Number,
    simplex::{SimplexSnapshot, SimplexStatus, SnapshotKind, SolveLog, SolvePhase, SolveStep},
};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ObjectiveType {
    Maximize,
    Minimize,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ConstraintType {
    LessEq,
    Eq,
    GreaterEq,
}

#[wasm_bindgen(js_name = Model)]
pub struct Model {
    inner: CoreModel,
}

#[wasm_bindgen(js_class = Model)]
impl Model {
    #[wasm_bindgen(constructor)]
    pub fn new(objective_type: ObjectiveType) -> Self {
        Self {
            inner: CoreModel::new(to_core_objective_type(objective_type)),
        }
    }

    pub fn add_variable_int(&mut self, is_integer: bool, obj_coeff: i64) -> usize {
        self.inner.add_variable(is_integer, obj_coeff)
    }

    pub fn add_variable_fraction(&mut self, is_integer: bool, num: i64, den: i64) -> usize {
        self.inner.add_variable(is_integer, Fraction::new(num, den))
    }

    pub fn add_constraint_int(&mut self, coeffs: Vec<i64>, relation: ConstraintType, rhs: i64) {
        self.inner
            .add_constraint(coeffs, to_core_constraint_type(relation), rhs);
    }

    pub fn add_constraint_fraction(
        &mut self,
        coeffs_num: Vec<i64>,
        coeffs_den: Vec<i64>,
        relation: ConstraintType,
        rhs_num: i64,
        rhs_den: i64,
    ) -> Result<(), JsValue> {
        if coeffs_num.len() != coeffs_den.len() {
            return Err(JsValue::from_str("coeffs_num and coeffs_den length mismatch"));
        }

        let coeffs = coeffs_num
            .into_iter()
            .zip(coeffs_den)
            .map(|(num, den)| Fraction::new(num, den))
            .collect::<Vec<_>>();

        self.inner.add_constraint(
            coeffs,
            to_core_constraint_type(relation),
            Fraction::new(rhs_num, rhs_den),
        );

        Ok(())
    }
}

#[wasm_bindgen(js_name = solveModel)]
pub fn solve_model(model: &Model) -> Result<JsValue, JsValue> {
    let status = crate::solve(&model.inner);
    let out = SolveStatusDto::from_status(status);
    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn to_core_objective_type(value: ObjectiveType) -> CoreObjectiveType {
    match value {
        ObjectiveType::Maximize => CoreObjectiveType::Maximize,
        ObjectiveType::Minimize => CoreObjectiveType::Minimize,
    }
}

fn to_core_constraint_type(value: ConstraintType) -> CoreConstraintType {
    match value {
        ConstraintType::LessEq => CoreConstraintType::LessEq,
        ConstraintType::Eq => CoreConstraintType::Eq,
        ConstraintType::GreaterEq => CoreConstraintType::GreaterEq,
    }
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum SolveStatusDto {
    Optimal {
        value: NumberDto,
        vars: Vec<FractionDto>,
        log: SolveLogDto,
    },
    Infeasible {
        log: SolveLogDto,
    },
    Unbounded {
        log: SolveLogDto,
    },
}

impl SolveStatusDto {
    fn from_status(status: SimplexStatus) -> Self {
        match status {
            SimplexStatus::Optimal { value, vars, log } => SolveStatusDto::Optimal {
                value: NumberDto::from(value),
                vars: vars.into_iter().map(FractionDto::from).collect(),
                log: SolveLogDto::from(log),
            },
            SimplexStatus::Infeasible { log } => SolveStatusDto::Infeasible {
                log: SolveLogDto::from(log),
            },
            SimplexStatus::Unbounded { log } => SolveStatusDto::Unbounded {
                log: SolveLogDto::from(log),
            },
        }
    }
}

#[derive(Serialize)]
struct SolveLogDto {
    steps: Vec<SolveStepDto>,
}

impl From<SolveLog> for SolveLogDto {
    fn from(value: SolveLog) -> Self {
        Self {
            steps: value.steps.into_iter().map(SolveStepDto::from).collect(),
        }
    }
}

#[derive(Serialize)]
struct SolveStepDto {
    phase: SolvePhaseDto,
    kind: SnapshotKindDto,
    pivot_row: usize,
    pivot_col: usize,
    snapshot: SimplexSnapshotDto,
}

impl From<SolveStep> for SolveStepDto {
    fn from(value: SolveStep) -> Self {
        Self {
            phase: SolvePhaseDto::from(value.phase),
            kind: SnapshotKindDto::from(value.kind),
            pivot_row: value.pivot_row,
            pivot_col: value.pivot_col,
            snapshot: SimplexSnapshotDto::from(value.snapshot),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum SolvePhaseDto {
    Primal,
    Dual,
}

impl From<SolvePhase> for SolvePhaseDto {
    fn from(value: SolvePhase) -> Self {
        match value {
            SolvePhase::Primal => SolvePhaseDto::Primal,
            SolvePhase::Dual => SolvePhaseDto::Dual,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum SnapshotKindDto {
    BeforePivot,
    AfterPivot,
}

impl From<SnapshotKind> for SnapshotKindDto {
    fn from(value: SnapshotKind) -> Self {
        match value {
            SnapshotKind::BeforePivot => SnapshotKindDto::BeforePivot,
            SnapshotKind::AfterPivot => SnapshotKindDto::AfterPivot,
        }
    }
}

#[derive(Serialize)]
struct SimplexSnapshotDto {
    tableau: TableauDto,
    estimates: Vec<NumberDto>,
    objective: NumberDto,
}

impl From<SimplexSnapshot> for SimplexSnapshotDto {
    fn from(value: SimplexSnapshot) -> Self {
        Self {
            tableau: TableauDto::from(value.tableau),
            estimates: value.estimates.into_iter().map(NumberDto::from).collect(),
            objective: NumberDto::from(value.objective),
        }
    }
}

#[derive(Serialize)]
struct TableauDto {
    matrix: Vec<Vec<FractionDto>>,
    basic_vars: Vec<usize>,
    objective_coef: Vec<NumberDto>,
    objective_type: ObjectiveTypeDto,
}

impl From<crate::tableau::Tableau> for TableauDto {
    fn from(value: crate::tableau::Tableau) -> Self {
        Self {
            matrix: value
                .matrix
                .into_iter()
                .map(|row| row.into_iter().map(FractionDto::from).collect())
                .collect(),
            basic_vars: value.basic_vars,
            objective_coef: value
                .objective_coef
                .into_iter()
                .map(NumberDto::from)
                .collect(),
            objective_type: ObjectiveTypeDto::from(value.objective_type),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum ObjectiveTypeDto {
    Maximize,
    Minimize,
}

impl From<CoreObjectiveType> for ObjectiveTypeDto {
    fn from(value: CoreObjectiveType) -> Self {
        match value {
            CoreObjectiveType::Maximize => ObjectiveTypeDto::Maximize,
            CoreObjectiveType::Minimize => ObjectiveTypeDto::Minimize,
        }
    }
}

#[derive(Serialize)]
struct NumberDto {
    m: FractionDto,
    val: FractionDto,
}

impl From<Number> for NumberDto {
    fn from(value: Number) -> Self {
        Self {
            m: FractionDto::from(value.m),
            val: FractionDto::from(value.val),
        }
    }
}

#[derive(Serialize)]
struct FractionDto {
    num: i64,
    den: i64,
}

impl From<Fraction> for FractionDto {
    fn from(value: Fraction) -> Self {
        Self {
            num: value.num,
            den: value.den,
        }
    }
}
