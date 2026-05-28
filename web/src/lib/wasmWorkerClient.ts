import { browser } from '$app/environment';

export type FractionDto = { num: number; den: number };
export type VarDto = { is_integer: boolean; coeff: FractionDto };
export type ConstraintDto = { coeffs: FractionDto[]; relation: 'LessEq' | 'Eq' | 'GreaterEq'; rhs: FractionDto };
export type ModelDto = {
    objectiveType: 'Maximize' | 'Minimize';
    variables: VarDto[];
    constraints: ConstraintDto[];
};

let nextId = 1;
const pending = new Map<number, { resolve: (v: any) => void; reject: (e: any) => void }>();
let worker: Worker | null = null;

function getWorker() {
    if (!browser) {
        throw new Error('Wasm worker is only available in the browser');
    }

    if (!worker) {
        worker = new Worker(new URL('./solver-worker.ts', import.meta.url), { type: 'module' });
        worker.addEventListener('message', (ev) => {
            const { id, ok, result, error } = ev.data as any;
            const entry = pending.get(Number(id));
            if (!entry) return;
            pending.delete(Number(id));
            if (ok) entry.resolve(result);
            else entry.reject(new Error(error));
        });
    }

    return worker;
}

export function solveModelInWorker(model: ModelDto): Promise<any> {
    const currentWorker = getWorker();
    const id = nextId++;
    return new Promise((resolve, reject) => {
        pending.set(id, { resolve, reject });
        currentWorker.postMessage({ id, model });
    });
}
