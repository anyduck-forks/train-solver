<script lang="ts">
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';
  import { editorSource, mainEditor, solveAndStore } from '$lib/solverState';

  type Row = { id: number; name: string; values: string[] };

  const initial = get(mainEditor);

  let columns = $state<string[]>([...initial.columns]);
  let rows = $state<Row[]>(
    initial.rows.map((row) => ({
      ...row,
      values: row.values.map((value) => `${value}`),
    })),
  );
  let isSolving = $state(false);

  $effect(() => {
    editorSource.set('main');
    mainEditor.set({
      columns: [...columns],
      rows: rows.map((row) => ({
        ...row,
        values: row.values.map((value) => Number(value) || 0),
      })),
    });
  });

  const addRowAt = (index: number) => {
    const nextIndex = rows.length + 1;
    const newRow = {
      id: Date.now(),
      name: `Рядок ${nextIndex}`,
      values: columns.map(() => '0'),
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
      values: [...row.values.slice(0, index), '0', ...row.values.slice(index)],
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
  <div class="hero">
    <p class="kicker">ММДО · Ввід даних</p>
    <h1>Таблиця складу потягів</h1>
  </div>
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
  <div class="solve-row">
    <progress
      class={`solve-progress${isSolving ? " is-active" : ""}`}
      aria-label="Розв’язання"
    ></progress>
    <div class="solve-actions">
      <a class="btn btn-secondary" href="/advanced">Розширене введення</a>
      <button class="btn btn-primary" type="button" onclick={startSolve}>
        Розв’язати
      </button>
    </div>
  </div>
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
    /* min-height: 100vh; */
    padding: 8vh 7vw 12vh;
    display: grid;
    gap: 32px;
    position: relative;
    overflow: hidden;
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


  .hero {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;
  }

  .kicker {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.3em;
    color: rgba(255, 220, 200, 0.7);
    margin: 0 0 8px;
  }

  h1 {
    margin: 0 0 12px;
    font-size: clamp(2.4rem, 3.6vw, 3.6rem);
    letter-spacing: -0.02em;
  }

  .lead {
    margin: 0;
    font-size: 1.05rem;
    color: rgba(245, 238, 230, 0.82);
    max-width: 640px;
  }

  .table-card {
    position: relative;
    z-index: 1;
  }

  .solve-row {
    display: flex;
    gap: 16px;
    align-items: center;
    flex-wrap: wrap;
    position: relative;
    z-index: 1;
  }

  .solve-progress {
    appearance: none;
    flex: 1 1 220px;
    height: 10px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.08);
    overflow: hidden;
    position: relative;
    z-index: 1;
    border: none;
    opacity: 0.4;
    transition: opacity 200ms ease;
  }

  .solve-progress::-webkit-progress-bar {
    background: rgba(255, 255, 255, 0.08);
    border-radius: 999px;
  }

  .solve-progress::-webkit-progress-value {
    background: linear-gradient(
      90deg,
      rgba(255, 166, 107, 0.2),
      rgba(255, 166, 107, 0.8)
    );
    border-radius: 999px;
  }

  .solve-progress::-moz-progress-bar {
    background: linear-gradient(
      90deg,
      rgba(255, 166, 107, 0.2),
      rgba(255, 166, 107, 0.8)
    );
    border-radius: 999px;
  }

  .solve-progress.is-active {
    opacity: 1;
  }

  .solve-progress.is-active::-webkit-progress-value {
    background-size: 200% 100%;
    animation: solveIndeterminate 1.2s linear infinite;
  }

  .solve-progress.is-active::-moz-progress-bar {
    background-size: 200% 100%;
    animation: solveIndeterminate 1.2s linear infinite;
  }

  @keyframes solveIndeterminate {
    0% {
      background-position: 200% 0;
    }
    100% {
      background-position: -200% 0;
    }
  }

  .solve-actions {
    display: flex;
    gap: 16px;
    align-items: center;
    flex-wrap: wrap;
  }

  .btn {
    border-radius: 999px;
    padding: 12px 22px;
    font-weight: 600;
    font-size: 0.9rem;
    text-decoration: none;
    border: 1px solid rgba(255, 255, 255, 0.16);
    background: rgba(255, 255, 255, 0.06);
    color: #f5f0e8;
    cursor: pointer;
    transition:
      transform 160ms ease,
      box-shadow 160ms ease,
      border 160ms ease;
  }

  .btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.35);
  }

  .btn-primary {
    background: #ffa66b;
    color: #2a1a16;
    border-color: transparent;
    box-shadow: 0 18px 40px rgba(255, 166, 107, 0.3);
  }

  .btn-primary:hover {
    box-shadow: 0 24px 48px rgba(255, 166, 107, 0.4);
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
