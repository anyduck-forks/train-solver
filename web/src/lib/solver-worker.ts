/// <reference path="./train_solver.d.ts" />
// @ts-ignore - generated at build time
import init, { Model, ObjectiveType, ConstraintType, solveModel } from '../../../pkg/train_solver.js';
import wasmUrl from '../../../pkg/train_solver_bg.wasm?url';

type FractionDto = { num: number; den: number };
type VarDto = { is_integer: boolean; coeff: FractionDto };
type ConstraintDto = { coeffs: FractionDto[]; relation: 'LessEq' | 'Eq' | 'GreaterEq'; rhs: FractionDto };

export type ModelDto = {
    objectiveType: 'Maximize' | 'Minimize';
    variables: VarDto[];
    constraints: ConstraintDto[];
};

let wasmReady: Promise<void> | null = null;

async function ensureWasm() {
    if (!wasmReady) {
        wasmReady = (async () => {
            await init(wasmUrl);
        })();
    }
    return wasmReady;
}

function toObjectiveType(o: 'Maximize' | 'Minimize') {
    return o === 'Maximize' ? ObjectiveType.Maximize : ObjectiveType.Minimize;
}

function toConstraintType(r: '<=' | '=' | '>=') {
    switch (r) {
        case '<=':
            return ConstraintType.LessEq;
        case '=':
            return ConstraintType.Eq;
        case '>=':
            return ConstraintType.GreaterEq;
    }
}

self.addEventListener('message', async (ev: MessageEvent) => {
    const { id, model } = ev.data as { id: string | number; model: ModelDto };
    try {
        await ensureWasm();

        const m = new Model(toObjectiveType(model.objectiveType));

        // add variables
        for (const v of model.variables) {
            const num = BigInt(v.coeff.num);
            const den = BigInt(v.coeff.den);
            m.add_variable_fraction(v.is_integer, num, den);
        }

        // add constraints
        for (const c of model.constraints) {
            const nums = c.coeffs.map(f => BigInt(f.num));
            const dens = c.coeffs.map(f => BigInt(f.den));
            const rhs_num = BigInt(c.rhs.num);
            const rhs_den = BigInt(c.rhs.den);
            // call add_constraint_fraction
            // the generated API expects BigInt64Array for coeffs
            m.add_constraint_fraction(BigInt64Array.from(nums), BigInt64Array.from(dens), toConstraintType(c.relation as any), rhs_num, rhs_den);
        }

        const res = solveModel(m);
        // res is a JS object (serde mapped) — post back to main thread
        self.postMessage({ id, ok: true, result: res });
    } catch (err) {
        self.postMessage({ id, ok: false, error: String(err) });
    }
});
