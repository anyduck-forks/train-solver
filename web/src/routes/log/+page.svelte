<script lang="ts">
  import {
    solveLabels,
    solveResult,
    solveSource,
    formatFraction,
    mainEditor,
    advancedEditor,
  } from '$lib/solverState';

  const sourceLabel = {
    main: 'Головна таблиця',
    advanced: 'Розширене введення',
  } as const;

  const backHref = $derived($solveSource === 'advanced' ? '/advanced' : '/');

  type CutDto = { coeffs: { num: number; den: number }[]; rhs: { num: number; den: number } };

  let openStep = $state<'start' | number | null>('start');

  const methodLabel = (phase: 'primal' | 'dual') =>
    phase === 'primal' ? 'Прямий симплекс' : 'Двоїстий симплекс';

  const kindLabel = (kind: 'before_pivot' | 'after_pivot') =>
    kind === 'before_pivot' ? 'Перед pivot' : 'Після pivot';

  const getCut = (step: (typeof $solveResult) extends infer R
    ? R extends { log: { steps: (infer S)[] } }
      ? S
      : never
    : never) => (step as { cut?: CutDto }).cut;

  const buildCutEquation = (cut: CutDto, labels: string[]) => {
    const terms = cut.coeffs.map((coeff, index) => {
      const value = formatFraction(coeff);
      const label = labels[index] ?? `x${index + 1}`;
      return `${value}·${label}`;
    });
    return `${terms.join(' + ')} \le ${formatFraction(cut.rhs)}`;
  };

  const startValueLabel = (source: 'main' | 'advanced') =>
    source === 'main' ? 'Парк вагонів' : 'Коефіцієнт цілі';

  const startValue = (source: 'main' | 'advanced', index: number) => {
    if (source === 'main') {
      const value = $mainEditor.rows[index]?.values[0] ?? 0;
      return formatFraction({ num: Number(value) || 0, den: 1 });
    }
    const value = $advancedEditor.objective[index] ?? '0';
    return formatFraction({ num: Number(value) || 0, den: 1 });
  };

  const basisLabel = (basisIndex: number | undefined, labels: string[]) => {
    if (basisIndex === undefined || basisIndex === null) return '—';
    return labels[basisIndex] ?? `x${basisIndex + 1}`;
  };

  const pivotSteps = $derived(
    ($solveResult?.log.steps ?? []).filter((step) => step.kind === 'after_pivot'),
  );

  const startStep = $derived(
    $solveResult?.log.steps.find((step) => step.kind === 'before_pivot') ??
      $solveResult?.log.steps[0] ??
      null,
  );
</script>

<svelte:head>
  <title>Результат — Задача про потяги</title>
</svelte:head>

<section class="page">
  <div class="hero">
    <p class="kicker">Журнал розв’язання</p>
    <h1>Результат збережено</h1>
    <p class="lead">
      Джерело: {sourceLabel[$solveSource]}. Нижче показано відповідь у порядку змінних та короткий слід
      pivot-ів.
    </p>
  </div>

  {#if $solveResult}
    <div class="panel status-panel">
      <div class="status-head">
        <div>
          <p class="status-label">Статус</p>
          <h2>{$solveResult.status}</h2>
        </div>
        <a class="cta" href={backHref}>Повернутись назад</a>
      </div>

      {#if $solveResult.status === 'optimal'}
        <div class="solution-card">
          <div class="objective">
            <span>Значення цілі</span>
            <strong>{formatFraction($solveResult.value)}</strong>
          </div>

          <div class="solution-list">
            {#each $solveLabels as label, index}
              <div class="solution-row">
                <span class="solution-label">{label}</span>
                <span class="solution-value">{formatFraction($solveResult.vars[index])}</span>
              </div>
            {/each}
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <p>Немає оптимального розв’язку для цього запуску.</p>
        </div>
      {/if}

      <div class="trace-head">
        <h3>Слід pivot-ів</h3>
        <span>{ pivotSteps.length } кроків</span>
      </div>

      <div class="variable-card">
        <div class="variable-head">
          <div>
            <p class="status-label">Змінні</p>
            <h3>Пояснення та стартові значення</h3>
          </div>
          <span class="variable-note">Джерело: {sourceLabel[$solveSource]}</span>
        </div>
        <p class="variable-help">Пояснює, що означає кожна змінна, та її стартове значення.</p>
        <div class="variable-list">
          {#each $solveLabels as label, index}
            <div class="variable-row">
              <div class="variable-main">
                <span class="variable-name">x{index + 1}</span>
                <span class="variable-meaning">{label}</span>
              </div>
              <div class="variable-value">
                <span class="variable-value-label">{startValueLabel($solveSource)}</span>
                <span class="variable-value-number">{startValue($solveSource, index)}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="trace-list">
        {#if startStep}
          <details
            class="trace-row"
            name="trace"
            open={openStep === 'start'}
            ontoggle={(event) => {
              const isOpen = (event.currentTarget as HTMLDetailsElement).open;
              openStep = isOpen ? 'start' : openStep === 'start' ? null : openStep;
            }}
          >
            <summary class="trace-summary">
              <div>
                <p class="trace-kind">Початкова симплекс-таблиця</p>
                <p class="trace-meta">стартове наближення</p>
              </div>
              <div class="trace-count">#0</div>
            </summary>
            <div class="trace-body">
              <div class="trace-table-wrap">
                <table class="trace-table">
                  <thead>
                    <tr>
                      <th></th>
                      {#each startStep.snapshot.tableau.matrix[0] as _, colIndex}
                        <th>A{colIndex}</th>
                      {/each}
                    </tr>
                  </thead>
                  <tbody>
                    {#each startStep.snapshot.tableau.matrix as row, rowIndex}
                      <tr>
                        <th>{basisLabel(startStep.snapshot.tableau.basic_vars[rowIndex], $solveLabels)}</th>
                        {#each row as cell}
                          <td>{formatFraction(cell)}</td>
                        {/each}
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            </div>
          </details>
        {/if}
        {#each pivotSteps as step, index}
          {@const cut = getCut(step)}
          <details
            class="trace-row"
            name="trace"
            open={openStep === index}
            ontoggle={(event) => {
              const isOpen = (event.currentTarget as HTMLDetailsElement).open;
              openStep = isOpen ? index : openStep === index ? null : openStep;
            }}
          >
            <summary class="trace-summary">
              <div>
                <p class="trace-kind">{methodLabel(step.phase)} · {kindLabel(step.kind)}</p>
                <p class="trace-meta">row {step.pivot_row}, col {step.pivot_col}</p>
              </div>
              <div class="trace-count">#{index + 1}</div>
            </summary>
            <div class="trace-body">
              <p class="trace-method">
                Метод: {methodLabel(step.phase)}. Стан: {kindLabel(step.kind)}.
              </p>
              {#if cut}
                <p class="trace-method">Додано зріз (cut), наведено рівняння.</p>
                <p class="cut-equation">
                  ${buildCutEquation(cut, $solveLabels)}$
                </p>
              {/if}
              <div class="trace-table-wrap">
                <table class="trace-table">
                  <thead>
                    <tr>
                      <th></th>
                      {#each step.snapshot.tableau.matrix[0] as _, colIndex}
                        <th>A{colIndex}</th>
                      {/each}
                    </tr>
                  </thead>
                  <tbody>
                    {#each step.snapshot.tableau.matrix as row, rowIndex}
                      <tr class={rowIndex === step.pivot_row ? 'pivot-row' : ''}>
                        <th>{basisLabel(step.snapshot.tableau.basic_vars[rowIndex], $solveLabels)}</th>
                        {#each row as cell, colIndex}
                          <td
                            class={
                              rowIndex === step.pivot_row && colIndex === step.pivot_col
                                ? 'pivot-cell'
                                : colIndex === step.pivot_col
                                  ? 'pivot-col'
                                  : ''
                            }
                          >
                            {formatFraction(cell)}
                          </td>
                        {/each}
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            </div>
          </details>
        {/each}
      </div>
    </div>
  {:else}
    <div class="panel empty-panel">
      <p>Ще немає результату. Запустіть розв’язання з головної або розширеної форми.</p>
      <a class="cta" href={backHref}>Повернутись назад</a>
    </div>
  {/if}
</section>

<style>
  :global(body) {
    margin: 0;
    font-family: 'Space Grotesk', system-ui, sans-serif;
    color: #f1f4f9;
    background: radial-gradient(circle at 20% 20%, #16283b 0%, #090c14 58%, #06070b 100%);
  }

  .page {
    min-height: 100vh;
    padding: 8vh 7vw 12vh;
    display: grid;
    gap: 28px;
    position: relative;
  }

  .hero,
  .panel {
    position: relative;
    z-index: 1;
  }

  .hero {
    display: grid;
    gap: 12px;
    max-width: 800px;
  }

  .kicker {
    margin: 0;
    font-size: 12px;
    letter-spacing: 0.3em;
    text-transform: uppercase;
    color: rgba(175, 213, 246, 0.72);
  }

  h1 {
    margin: 0;
    font-size: clamp(2.4rem, 4vw, 3.8rem);
  }

  .lead {
    margin: 0;
    max-width: 760px;
    color: rgba(220, 232, 245, 0.82);
    line-height: 1.6;
  }

  .panel {
    background: rgba(14, 20, 32, 0.84);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 24px;
    padding: 24px;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(10px);
  }

  .status-head,
  .trace-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .status-label,
  .trace-kind {
    margin: 0 0 4px;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    font-size: 11px;
    color: rgba(175, 213, 246, 0.7);
  }

  h2,
  h3 {
    margin: 0;
  }

  .cta {
    padding: 12px 20px;
    border-radius: 999px;
    background: #7fd0ff;
    color: #0c1824;
    text-decoration: none;
    font-weight: 700;
  }

  .solution-card,
  .empty-state {
    margin-top: 18px;
    padding: 18px;
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .variable-card {
    margin-top: 18px;
    padding: 18px;
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    display: grid;
    gap: 14px;
  }

  .variable-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .variable-note {
    color: rgba(220, 232, 245, 0.7);
    font-size: 0.9rem;
  }

  .variable-help {
    margin: 0;
    color: rgba(220, 232, 245, 0.82);
  }

  .variable-list {
    display: grid;
    gap: 10px;
  }

  .variable-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 12px 14px;
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.04);
  }

  .variable-main {
    display: flex;
    gap: 12px;
    align-items: baseline;
  }

  .variable-name {
    font-weight: 700;
    color: rgba(127, 208, 255, 0.95);
  }

  .variable-meaning {
    color: rgba(220, 232, 245, 0.86);
  }

  .variable-value {
    text-align: right;
    display: grid;
    gap: 2px;
  }

  .variable-value-label {
    font-size: 0.8rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: rgba(175, 213, 246, 0.68);
  }

  .variable-value-number {
    font-weight: 700;
    color: rgba(220, 232, 245, 0.92);
  }

  .objective {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: 12px;
    margin-bottom: 16px;
  }

  .objective span {
    color: rgba(220, 232, 245, 0.72);
  }

  .objective strong {
    font-size: 1.5rem;
  }

  .solution-list,
  .trace-list {
    display: grid;
    gap: 10px;
  }

  .solution-row,
  .trace-row {
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.04);
  }

  .trace-row {
    padding: 0;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .trace-summary {
    list-style: none;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 12px 14px;
  }

  .trace-summary::-webkit-details-marker {
    display: none;
  }

  .trace-body {
    padding: 0 14px 14px;
    display: grid;
    gap: 12px;
  }

  .trace-method {
    margin: 0;
    color: rgba(220, 232, 245, 0.82);
  }

  .cut-equation {
    margin: 0;
    font-weight: 600;
    color: rgba(127, 208, 255, 0.9);
  }

  .trace-table-wrap {
    overflow: auto;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .trace-table {
    width: max-content;
    min-width: 100%;
    border-collapse: collapse;
    background: rgba(10, 14, 20, 0.7);
  }

  .trace-table th,
  .trace-table td {
    padding: 8px 10px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    border-right: 1px solid rgba(255, 255, 255, 0.05);
    text-align: right;
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  .trace-table th {
    text-align: left;
    font-weight: 600;
    color: rgba(220, 232, 245, 0.8);
  }

  .pivot-row td,
  .pivot-row th {
    background: rgba(127, 208, 255, 0.12);
  }

  .pivot-col {
    background: rgba(127, 208, 255, 0.18);
  }

  .pivot-cell {
    background: rgba(255, 166, 107, 0.4);
    font-weight: 700;
    color: #fff1e6;
  }

  .solution-label {
    font-weight: 700;
  }

  .solution-value,
  .trace-meta,
  .trace-count {
    color: rgba(220, 232, 245, 0.82);
  }

  .trace-head {
    margin: 22px 0 12px;
  }

  .trace-meta {
    margin: 0;
    font-size: 0.92rem;
  }

  .trace-count {
    font-variant-numeric: tabular-nums;
    opacity: 0.9;
  }

  .empty-panel {
    display: grid;
    gap: 16px;
    max-width: 640px;
  }

  @media (max-width: 720px) {
    .page {
      padding: 6vh 6vw 10vh;
    }

    .objective,
    .solution-row,
    .trace-row {
      align-items: flex-start;
      flex-direction: column;
    }
  }
</style>
