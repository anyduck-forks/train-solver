<script lang="ts">
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';
  import SolveActions from '$lib/components/SolveActions.svelte';
  import Hero from '$lib/components/Hero.svelte';
  import { editorSource, mainEditor, solveAndStore } from '$lib/solverState';

  type Row = { id: number; name: string; values: number[] };

  const initial = get(mainEditor);

  let columns = $state<string[]>([...initial.columns]);
  let rows = $state<Row[]>(
    initial.rows.map((row) => ({
      ...row,
      values: [...row.values],
    })),
  );
  let isSolving = $state(false);

  $effect(() => {
    editorSource.set('main');
    mainEditor.set({
      columns: [...columns],
      rows: rows.map((row) => ({
        ...row,
        values: [...row.values],
      })),
    });
  });

  const addRowAt = (index: number) => {
    const nextIndex = rows.length + 1;
    const newRow = {
      id: Date.now(),
      name: `Рядок ${nextIndex}`,
      values: columns.map(() => 0),
    };
    rows = [...rows.slice(0, index), newRow, ...rows.slice(index)];
  };

  const addRowAtEnd = () => addRowAt(rows.length);

  const removeRow = (id: number) => {
    if (rows.length <= 1) return;
    rows = rows.filter((row) => row.id !== id);
  };

  const addColumnAt = (index: number) => {
    const nextIndex = columns.length + 1;
    columns = [
      ...columns.slice(0, index),
      `Колонка ${nextIndex}`,
      ...columns.slice(index),
    ];
    rows = rows.map((row) => ({
      ...row,
      values: [...row.values.slice(0, index), 0, ...row.values.slice(index)],
    }));
  };

  const addColumnAtEnd = () => addColumnAt(columns.length);

  const removeColumn = (index: number) => {
    if (columns.length <= 2 || index < 2) return;
    columns = columns.filter((_, i) => i !== index);
    rows = rows.map((row) => ({
      ...row,
      values: row.values.filter((_, i) => i !== index),
    }));
  };

  const startSolve = async () => {
    if (isSolving) return;
    isSolving = true;
    try {
      await solveAndStore('main');
      await goto('/log');
    } finally {
      isSolving = false;
    }
  };
</script>

<svelte:head>
  <title>Ввід даних — Задача про потяги</title>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link
    rel="preconnect"
    href="https://fonts.gstatic.com"
    crossorigin="anonymous"
  />
  <link
    href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Literata:wght@400;600&display=swap"
    rel="stylesheet"
  />
</svelte:head>

<section class="page">
  <Hero kicker="ММДО · Ввід даних">Таблиця складу потягів</Hero>
  <div class="table-card">
    <div class="table-shell">
      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th>Вагон</th>
              {#each columns as column, index}
                <th>
                  <div class="header">
                    <input
                      class="header-input"
                      type="text"
                      bind:value={columns[index]}
                      aria-label={`Назва стовпця ${index + 1}`}
                    />
                    {#if index >= 2}
                      <button
                        class="remove"
                        type="button"
                        onclick={() => removeColumn(index)}
                        aria-label="Видалити стовпець"
                      >
                        ×
                      </button>
                    {/if}
                  </div>
                </th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each rows as row, rowIndex}
              <tr>
                <td>
                  <div class="row-head">
                    <input
                      class="row-input"
                      type="text"
                      bind:value={row.name}
                      aria-label="Назва вагону"
                    />
                    <button
                      class="remove"
                      type="button"
                      onclick={() => removeRow(row.id)}
                      aria-label="Видалити рядок"
                    >
                      ×
                    </button>
                  </div>
                </td>
                {#each row.values as value, valueIndex}
                  <td>
                    <input
                      class="cell-input"
                      type="number"
                      bind:value={row.values[valueIndex]}
                      aria-label={`Значення ${row.name} ${columns[valueIndex]}`}
                    />
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
      <button
        class="edge-slot edge-add edge-col"
        type="button"
        onclick={addColumnAtEnd}
        aria-label="Додати стовпець у кінець"
      >
        +
      </button>
      <button
        class="edge-slot edge-add edge-row"
        type="button"
        onclick={addRowAtEnd}
        aria-label="Додати рядок у кінець"
      >
        +
      </button>
      <div class="edge-slot edge-slot-empty" aria-hidden="true"></div>
    </div>
  </div>
  <SolveActions
    {isSolving}
    onSolve={startSolve}
    secondaryHref="/advanced"
    secondaryLabel="Розширене введення"
  />
</section>

<style>
  :global(body) {
    margin: 0;
    font-family: "Space Grotesk", system-ui, sans-serif;
    color: #f5f0e8;
    background: radial-gradient(
      circle at 10% 20%,
      #3c2b2a,
      #0c0b10 55%,
      #08070b 100%
    );
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

  .page::before {
    content: "";
    position: absolute;
    width: 460px;
    height: 460px;
    border-radius: 50%;
    background: radial-gradient(
      circle,
      rgba(255, 165, 110, 0.35),
      rgba(255, 165, 110, 0)
    );
    filter: blur(10px);
    z-index: 0;
  }

  .page::before {
    top: -120px;
    left: -160px;
  }
 

  .table-card {
    position: relative;
    z-index: 1;
  }

  .table-shell {
    display: grid;
    grid-template-columns: 1fr 44px;
    grid-template-rows: 1fr 44px;
    gap: 12px;
    align-items: stretch;
  }

  .table-wrap {
    --first-col-width: 170px;
    --col-width: 140px;
    max-width: 100%;
    overflow: auto;
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    position: relative;
    transition:
      border 160ms ease,
      box-shadow 160ms ease;
  }

  .edge-slot {
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: rgba(12, 10, 12, 0.55);
    display: grid;
    place-items: center;
  }

  .edge-slot-empty {
    border: none;
    background: transparent;
  }

  table {
    display: table;
    border-collapse: collapse;
    width: max-content;
    min-width: 100%;
    background: rgba(12, 10, 12, 0.7);
  }

  th,
  td {
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    border-right: 1px solid rgba(255, 255, 255, 0.06);
    padding: 10px 12px;
    text-align: left;
    vertical-align: middle;
    white-space: nowrap;
    width: var(--col-width);
    position: relative;
    z-index: 1;
  }

  th {
    font-size: 0.9rem;
    font-weight: 600;
    color: rgba(255, 230, 210, 0.9);
    background: rgba(18, 14, 18, 0.9);
    position: sticky;
    top: 0;
    z-index: 2;
  }

  .header,
  .row-head {
    display: flex;
    align-items: center;
    gap: 8px;
    position: relative;
  }

  .header-input {
    width: 100%;
    background: transparent;
    border: none;
    color: inherit;
    font-weight: 600;
    padding: 0;
  }

  .row-input {
    width: 100%;
    background: transparent;
    border: none;
    color: inherit;
    font-weight: 600;
    padding: 0;
  }

  .cell-input {
    width: 100%;
    min-width: 60px;
    background: transparent;
    border: none;
    padding: 10px 12px;
    color: #f5f0e8;
    font-family: "Literata", serif;
    outline: none;
  }

  .cell-input:focus,
  .header-input:focus,
  .row-input:focus {
    outline: none;
  }

  .cell-input[type="number"] {
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .cell-input[type="number"]::-webkit-outer-spin-button,
  .cell-input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  td:focus-within {
    outline: 2px solid rgba(255, 166, 107, 0.8);
    outline-offset: -1px;
    background: rgba(255, 166, 107, 0.05);
  }

  th:focus-within {
    outline: 2px solid rgba(255, 166, 107, 0.8);
    outline-offset: -1px;
  }

  .remove {
    opacity: 0;
    border: none;
    background: rgba(255, 255, 255, 0.12);
    color: #f5f0e8;
    min-width: 26px;
    height: 26px;
    padding: 0;
    border-radius: 50%;
    cursor: pointer;
    transition:
      opacity 160ms ease,
      transform 160ms ease;
  }

  .header:hover .remove,
  .row-head:hover .remove {
    opacity: 1;
    transform: scale(1.05);
  }

  .edge-add {
    --edge-scale: 1;
    border: none;
    background: transparent;
    color: rgba(249, 231, 214, 0.9);
    font-weight: 800;
    border-radius: 16px;

    display: grid;
    place-items: center;
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.35);
    transition: all 160ms ease;
    transform: scale(var(--edge-scale));
  }

  .edge-add:hover {
    --edge-scale: 1.01;
    color: #2a1a16;
    background: rgba(255, 166, 107, 0.95);
    box-shadow: 0 18px 36px rgba(0, 0, 0, 0.35);
  }


  @media (max-width: 720px) {
    .page {
      padding: 6vh 6vw 10vh;
    }

    .table-card {
      padding: 18px;
    }
  }
</style>
