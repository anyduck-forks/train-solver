import { derived, get, writable } from 'svelte/store';

import { solveModelInWorker, type ModelDto } from '$lib/wasmWorkerClient';

export type SolveSource = 'main' | 'advanced';

export type FractionDto = { num: number; den: number };
export type NumberDto = { m: FractionDto; val: FractionDto };
export type SolveStepDto = {
    phase: 'primal' | 'dual';
    kind: 'before_pivot' | 'after_pivot' | 'cut';
    pivot_row: number;
    pivot_col: number;
    snapshot: {
        tableau: {
            matrix: FractionDto[][];
            basic_vars: number[];
            objective_coef: NumberDto[];
            objective_type: 'maximize' | 'minimize';
        };
        estimates: NumberDto[];
        objective: NumberDto;
    };
    cut?: { coeffs: FractionDto[]; rhs: FractionDto };
};

export type SolveResultDto =
    | {
          status: 'optimal';
          value: NumberDto;
          vars: FractionDto[];
          log: { steps: SolveStepDto[] };
      }
    | {
          status: 'infeasible' | 'unbounded';
          log: { steps: SolveStepDto[] };
      };

export type MainRow = {
    id: number;
    name: string;
    values: number[];
};

export type MainEditorState = {
    columns: string[];
    rows: MainRow[];
};

export type AdvancedVariable = {
    id: number;
    name: string;
    sourceLabel?: string;
    isInteger: boolean;
};

export type AdvancedConstraint = {
    id: number;
    coeffs: string[];
    sign: '<=' | '>=' | '=';
    rhs: string;
};

export type AdvancedEditorState = {
    objectiveType: 'Maximize' | 'Minimize';
    variables: AdvancedVariable[];
    objective: string[];
    constraints: AdvancedConstraint[];
};

const initialMainEditor: MainEditorState = {
    columns: ['Парк вагонів', 'К-сть пасажирів', 'Швидкий', 'Пасажирський'],
    rows: [
        { id: 1, name: 'Багажний', values: [12, 0, 1, 1] },
        { id: 2, name: 'Поштовий', values: [18, 0, 1, 0] },
        { id: 3, name: 'Жорсткий', values: [89, 58, 5, 8] },
        { id: 4, name: 'Купейний', values: [79, 40, 6, 4] },
        { id: 5, name: 'Мʼякий', values: [35, 32, 4, 2] },
    ],
};

const initialAdvancedEditor: AdvancedEditorState = {
    objectiveType: 'Maximize',
    variables: [
        { id: 1, name: 'x1', isInteger: false },
        { id: 2, name: 'x2', isInteger: false },
    ],
    objective: ['0', '0'],
    constraints: [
        { id: 1, coeffs: ['0', '0'], sign: '<=', rhs: '0' },
        { id: 2, coeffs: ['0', '0'], sign: '>=', rhs: '0' },
    ],
};

export const mainEditor = writable<MainEditorState>(structuredClone(initialMainEditor));
export const advancedEditor = writable<AdvancedEditorState>(structuredClone(initialAdvancedEditor));
export const editorSource = writable<SolveSource>('main');
export const solveSource = writable<SolveSource>('main');
export const solveResult = writable<SolveResultDto | null>(null);
export const solveLabels = writable<string[]>([]);

function mainToAdvancedEditor(editor: MainEditorState): AdvancedEditorState {
    const trainTypeLabels = editor.columns.slice(2);
    const variables = trainTypeLabels.map((label, index) => ({
        id: index + 1,
        name: `x${index + 1}`,
        isInteger: true,
    }));

    const objective = trainTypeLabels.map((_, trainTypeIndex) => {
        const coeff = editor.rows.reduce((sum, row) => {
            const passengerPerWagon = Number(row.values[1] ?? 0) || 0;
            const wagonsPerTrain = Number(row.values[trainTypeIndex + 2] ?? 0) || 0;
            return sum + passengerPerWagon * wagonsPerTrain;
        }, 0);
        return `${coeff}`;
    });

    const constraints = editor.rows.map((row, rowIndex) => {
        const coeffs = trainTypeLabels.map(
            (_, trainTypeIndex) => `${Number(row.values[trainTypeIndex + 2] ?? 0) || 0}`,
        );
        const rhs = Number(row.values[0] ?? 0) || 0;
        return {
            id: row.id || rowIndex + 1,
            coeffs,
            sign: '<=' as const,
            rhs: `${rhs}`,
        };
    });

    return {
        objectiveType: 'Maximize',
        variables,
        objective,
        constraints,
    };
}

export function resetAdvancedFromMain() {
    advancedEditor.set(mainToAdvancedEditor(get(mainEditor)));
}

function toFraction(value: number | string | undefined | null): FractionDto {
    const parsed = typeof value === 'number' ? value : Number(value ?? 0);
    return { num: Number.isFinite(parsed) ? parsed : 0, den: 1 };
}

function signToRelation(sign: '<=' | '>=' | '=') {
    if (sign === '<=') return 'LessEq' as const;
    if (sign === '>=') return 'GreaterEq' as const;
    return 'Eq' as const;
}

function buildMainModel(editor: MainEditorState): { model: ModelDto; labels: string[] } {
    // Variables are the train types (columns after the first two)
    const trainTypeLabels = editor.columns.slice(2);
    const labels = trainTypeLabels.map((_, index) => `x${index + 1}`);

    // Objective coefficient for each train type = sum over rows of (passengers_per_wagon * wagons_per_train)
    const variables = trainTypeLabels.map((_, trainTypeIndex) => {
        const coeff = editor.rows.reduce((sum, row) => {
            const passengerPerWagon = Number(row.values[1] ?? 0) || 0;
            const wagonsPerTrain = Number(row.values[trainTypeIndex + 2] ?? 0) || 0;
            return sum + passengerPerWagon * wagonsPerTrain;
        }, 0);
        return { is_integer: true, coeff: toFraction(coeff) };
    });

    // Each row becomes a constraint: sum_j wagons_needed[row][j] * x_j <= park_row
    const constraints = editor.rows.map((row) => {
        const coeffs = trainTypeLabels.map((_, trainTypeIndex) => toFraction(row.values[trainTypeIndex + 2] ?? 0));
        const rhs = Number(row.values[0] ?? 0) || 0;
        return { coeffs, relation: 'LessEq' as const, rhs: { num: rhs, den: 1 } };
    });

    return {
        labels,
        model: {
            objectiveType: 'Maximize',
            variables,
            constraints,
        },
    };
}

function buildAdvancedModel(editor: AdvancedEditorState): { model: ModelDto; labels: string[] } {
    const labels = editor.variables.map((v, index) => v.name ?? `x${index + 1}`);
    const variables = editor.variables.map((variable, index) => ({
        is_integer: variable.isInteger,
        coeff: toFraction(editor.objective[index] ?? '0'),
    }));
    const constraints = editor.constraints.map((constraint) => ({
        coeffs: constraint.coeffs.map((coeff) => toFraction(coeff)),
        relation: signToRelation(constraint.sign),
        rhs: toFraction(constraint.rhs),
    }));

    return {
        labels,
        model: {
            objectiveType: editor.objectiveType,
            variables,
            constraints,
        },
    };
}

export const mainModel = derived(mainEditor, (editor) => buildMainModel(editor));
export const advancedModel = derived(advancedEditor, (editor) => buildAdvancedModel(editor));

export const currentModel = derived(
    [editorSource, mainEditor, advancedEditor],
    ([$editorSource, $mainEditor, $advancedEditor]) =>
        $editorSource === 'main' ? buildMainModel($mainEditor) : buildAdvancedModel($advancedEditor),
);

export const currentLabels = derived(
    [editorSource, mainEditor, advancedEditor],
    ([$editorSource, $mainEditor, $advancedEditor]) =>
        $editorSource === 'main'
            ? buildMainModel($mainEditor).labels
            : buildAdvancedModel($advancedEditor).labels,
);

export function formatFraction(value: FractionDto | NumberDto): string {
    if ('m' in value) {
        const val = formatFraction(value.val);
        const m = formatFraction(value.m);
        if (m === '0') return val;
        if (val === '0') return `${m}M`;
        return `${m}M ${Number(value.val.num) >= 0 ? '+' : '-'} ${formatFraction({ ...value.val, num: Math.abs(value.val.num) })}`;
    }

    if (value.den === 1) return `${value.num}`;
    return `${value.num}/${value.den}`;
}

export async function solveAndStore(source: SolveSource): Promise<SolveResultDto> {
    editorSource.set(source);
    solveSource.set(source);
    const resolved = get(currentModel);
    // For display purposes: when solving from main, use the human-friendly train-type
    // column names as solution labels. For advanced, use the derived labels.
    let labels: string[];
    if (source === 'main') {
        const main = get(mainEditor);
        labels = main.columns.slice(2).map((c) => c ?? '');
    } else {
        labels = get(currentLabels);
    }
    const result = (await solveModelInWorker(resolved.model)) as SolveResultDto;
    solveLabels.set(labels);
    solveResult.set(result);
    return result;
}
