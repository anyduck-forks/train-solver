<svelte:head>
  <title>Advanced Entry — Лінійне програмування</title>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
  <link
    href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Literata:wght@400;600&display=swap"
    rel="stylesheet"
  />
</svelte:head>

<script lang="ts">
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';
  import SolveActions from '$lib/components/SolveActions.svelte';
  import {
    advancedEditor,
    editorSource,
    resetAdvancedFromMain,
    solveAndStore,
  } from '$lib/solverState';
    import Hero from '$lib/components/Hero.svelte';

  resetAdvancedFromMain();
  const initial = get(advancedEditor);

  let objectiveType = $state<'Maximize' | 'Minimize'>(initial.objectiveType);
  let variables = $state(initial.variables.map((variable) => ({ ...variable })));
  let objective = $state([...initial.objective]);
  let constraints = $state(
    initial.constraints.map((constraint) => ({
      ...constraint,
      coeffs: [...constraint.coeffs],
    })),
  );

  let isSolving = $state(false);

  const normalizeVariables = () =>
    variables.map((variable, index) => ({
      ...variable,
      name: `x${index + 1}`,
    }));

  $effect(() => {
    const normalized = normalizeVariables();
    if (normalized.some((variable, index) => variable.name !== variables[index].name)) {
      variables = normalized;
      return;
    }

    editorSource.set('advanced');
    advancedEditor.set({
      objectiveType,
      variables: normalized,
      objective: [...objective],
      constraints: constraints.map((constraint) => ({
        ...constraint,
        coeffs: [...constraint.coeffs],
      })),
    });
  });

  const addVariable = () => {
    const nextIndex = variables.length + 1;
    variables = [
      ...variables,
      { id: Date.now(), name: `x${nextIndex}`, isInteger: false },
    ];
    objective = [...objective, '0'];
    constraints = constraints.map((row) => ({ ...row, coeffs: [...row.coeffs, '0'] }));
  };

  const removeVariable = (index: number) => {
    if (variables.length <= 1) return;
    variables = variables.filter((_, i) => i !== index);
    objective = objective.filter((_, i) => i !== index);
    constraints = constraints.map((row) => ({
      ...row,
      coeffs: row.coeffs.filter((_, i) => i !== index)
    }));
  };

  const addConstraint = () => {
    constraints = [
      ...constraints,
      {
        id: Date.now(),
        coeffs: variables.map(() => '0'),
        sign: '<=',
        rhs: '0'
      }
    ];
  };

  const removeConstraint = (id: number) => {
    if (constraints.length <= 1) return;
    constraints = constraints.filter((row) => row.id !== id);
  };

  const startSolve = async () => {
    if (isSolving) return;
    isSolving = true;
    try {
      await solveAndStore('advanced');
      await goto('/log');
    } finally {
      isSolving = false;
    }
  };
</script>

<section class="page">
  <Hero kicker="ММДО · Ввід даних">Лінійне програмування</Hero>
  <div class="panel">
    <div class="panel-head">
      <div class="objective-head">
        <h2>Цільова функція</h2>
        <select class="sign" bind:value={objectiveType} aria-label="Тип цільової функції">
          <option value="Maximize">max</option>
          <option value="Minimize">min</option>
        </select>
      </div>
      <button class="ghost" type="button" onclick={addVariable}>+ Додати змінну</button>
    </div>

    <div class="scroll-x">
      <div class="equation">
        {#each variables as variable, index}
          <div class="term">
            <input
              class="coef"
              type="text"
              bind:value={objective[index]}
              inputmode="decimal"
              aria-label={`Коефіцієнт x${index + 1}`}
            />
            <span class="mul">×</span>
            <div class="var-chip">
              <span class="var-label-fixed">x{index + 1}</span>
              
              <button
                class="remove"
                type="button"
                onclick={() => removeVariable(index)}
                aria-label="Видалити змінну"
              >
                ×
              </button>
            </div>
            {#if index < variables.length - 1}
              <span class="plus">+</span>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  <!-- </div>

  <div class="panel"> -->
    <div class="panel-head">
      <h2>Цілочисельні обмеження</h2>
    </div>
    <div class="integer-grid">
      {#each variables as variable, index}
        <label class="int-row">
          <span>x{index + 1} ∈ Z</span>
          <input type="checkbox" bind:checked={variables[index].isInteger} />
        </label>
      {/each}
    </div>
  <!-- </div>

  <div class="panel"> -->
    <div class="panel-head">
      <h2>Обмеження</h2>
      <button class="ghost" type="button" onclick={addConstraint}>+ Додати обмеження</button>
    </div>

    <div class="constraints">
      {#each constraints as constraint}
        <div class="constraint">
          <div class="scroll-x">
            <div class="equation">
              {#each variables as variable, index}
                <div class="term">
                  <input
                    class="coef"
                    type="text"
                    bind:value={constraint.coeffs[index]}
                    inputmode="decimal"
                    aria-label={`Коефіцієнт x${index + 1}`}
                  />
                  <span class="mul">×</span>
                  <span class="var-label">x{index + 1}</span>
                  {#if index < variables.length - 1}
                    <span class="plus">+</span>
                  {/if}
                </div>
              {/each}
            </div>
          </div>

          <select class="sign" bind:value={constraint.sign} aria-label="Знак обмеження">
            <option value="<=">≤</option>
            <option value=">=">≥</option>
            <option value="=">=</option>
          </select>

          <input
            class="rhs"
            type="text"
            bind:value={constraint.rhs}
            inputmode="decimal"
            aria-label="Права частина"
          />

          <button
            class="remove constraint-remove"
            type="button"
            onclick={() => removeConstraint(constraint.id)}
            aria-label="Видалити обмеження"
          >
            ×
          </button>
        </div>
      {/each}
    </div>
  </div>

  <SolveActions
    {isSolving}
    onSolve={startSolve}
    secondaryHref="/"
    secondaryLabel="Назад до таблиці"
  />
</section>

<style>
  :global(body) {
    margin: 0;
    font-family: 'Space Grotesk', system-ui, sans-serif;
    color: #f1f4f9;
    background: radial-gradient(circle at 20% 20%, #17324a 0%, #0a0d18 55%, #07090e 100%);
  }

  .page {
    min-height: 100vh;
    padding: 6vh 6vw 8vh;
    display: grid;
    gap: 18px;
    position: relative;
    overflow: hidden;
    align-content: start;
  }

  .page::before,
  .page::after {
    content: '';
    position: absolute;
    width: 520px;
    height: 520px;
    border-radius: 40% 60% 50% 50%;
    background: radial-gradient(circle, rgba(86, 170, 255, 0.25), rgba(86, 170, 255, 0));
    filter: blur(20px);
    z-index: 0;
  }

  .page::before {
    top: -180px;
    right: -160px;
  }

  .page::after {
    bottom: -200px;
    left: -180px;
    background: radial-gradient(circle, rgba(255, 196, 120, 0.25), rgba(255, 196, 120, 0));
  }

  .panel {
    position: relative;
    z-index: 1;
    background: rgba(14, 20, 32, 0.82);
    border-radius: 16px;
    padding: 14px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(10px);
  }

  .panel-head {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
    margin-bottom: 1rem;
    margin-top: 1rem;
  }

  .objective-head {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  h2 {
    margin: 0;
    font-size: 1rem;
  }

  .ghost {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #f1f4f9;
    padding: 7px 12px;
    border-radius: 999px;
    font-weight: 600;
    cursor: pointer;
    transition: border 160ms ease, transform 160ms ease;
  }

  .ghost:hover {
    border-color: rgba(255, 255, 255, 0.4);
    transform: translateY(-1px);
  }

  .scroll-x {
    overflow-x: auto;
  }

  .equation {
    display: inline-flex;
    gap: 8px;
    align-items: center;
    padding: 2px 0;
    min-width: 520px;
  }

  .term {
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }

  .coef {
    width: 62px;
    padding: 4px 6px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: rgba(255, 255, 255, 0.04);
    color: #f1f4f9;
    font-family: 'Literata', serif;
    font-size: 0.82rem;
  }

  .mul,
  .plus {
    color: rgba(160, 198, 230, 0.9);
    font-weight: 600;
  }

  .var-chip {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 3px 5px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.08);
  }

  .var-label-fixed {
    min-width: 24px;
    font-weight: 700;
    color: rgba(230, 242, 255, 0.92);
    text-align: center;
  }

  .integer-grid {
    display: grid;
    gap: 6px;
  }

  .int-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 7px 10px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .var-label {
    font-weight: 600;
    color: rgba(230, 242, 255, 0.9);
  }

  .constraints {
    display: grid;
    gap: 14px;
  }

  .constraint {
    display: grid;
    grid-template-columns: 1fr auto auto auto;
    gap: 12px;
  }

  .sign {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #f1f4f9;
    padding: 4px 6px;
    border-radius: 8px;
    font-weight: 600;
    font-size: 0.82rem;
  }

  .rhs {
    width: 76px;
    padding: 4px 6px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: rgba(255, 255, 255, 0.04);
    color: #f1f4f9;
    font-family: 'Literata', serif;
    font-size: 0.82rem;
  }

  .remove {
    opacity: 0;
    border: none;
    background: rgba(255, 255, 255, 0.12);
    color: #f1f4f9;
    width: 26px;
    height: 26px;
    border-radius: 50%;
    cursor: pointer;
    transition: opacity 160ms ease, transform 160ms ease;
  }

  .var-chip:hover .remove,
  .constraint:hover .constraint-remove {
    opacity: 1;
    transform: scale(1.05);
  }

  .constraint-remove {
    justify-self: center;
  }

  .coef:focus,
  .rhs:focus {
    outline: 2px solid rgba(127, 208, 255, 0.7);
    outline-offset: 2px;
  }

  @media (max-width: 900px) {
    .constraint {
      grid-template-columns: 1fr;
    }

    .sign,
    .rhs {
      width: 100%;
    }

    .constraint-remove {
      justify-self: flex-end;
    }
  }
</style>
