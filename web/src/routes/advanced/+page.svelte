<svelte:head>
  <title>Advanced Entry — Лінійне програмування</title>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
  <link
    href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Literata:wght@400;600&display=swap"
    rel="stylesheet"
  />
</svelte:head>

<script>
  let variables = $state([
    { id: 1, name: 'x1' },
    { id: 2, name: 'x2' }
  ]);
  let objective = $state(['0', '0']);
  let constraints = $state([
    { id: 1, coeffs: ['0', '0'], sign: '<=', rhs: '0' },
    { id: 2, coeffs: ['0', '0'], sign: '>=', rhs: '0' }
  ]);

  const addVariable = () => {
    const nextIndex = variables.length + 1;
    variables = [...variables, { id: Date.now(), name: `x${nextIndex}` }];
    objective = [...objective, '0'];
    constraints = constraints.map((row) => ({ ...row, coeffs: [...row.coeffs, '0'] }));
  };

  /** @param {number} index */
  const removeVariable = (index) => {
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

  /** @param {number} id */
  const removeConstraint = (id) => {
    if (constraints.length <= 1) return;
    constraints = constraints.filter((row) => row.id !== id);
  };
</script>

<section class="page">
  <header class="hero">
    <div>
      <p class="kicker">page 3 · advanced entry</p>
      <h1>ММДО: лінійне програмування</h1>
      <p class="lead">
        Введіть коефіцієнти для цільової функції та обмежень. Додавайте змінні й обмеження
        кнопками «+», видаляйте — хрестиком при наведенні.
      </p>
    </div>
    <a class="cta" href="/page2">Назад до таблиці</a>
  </header>

  <div class="panel">
    <div class="panel-head">
      <h2>Цільова функція (max)</h2>
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
              aria-label={`Коефіцієнт ${variable.name}`}
            />
            <span class="mul">×</span>
            <div class="var-chip">
              <input
                class="var"
                type="text"
                bind:value={variables[index].name}
                aria-label="Назва змінної"
              />
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
  </div>

  <div class="panel">
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
                    aria-label={`Коефіцієнт ${variable.name}`}
                  />
                  <span class="mul">×</span>
                  <span class="var-label">{variable.name}</span>
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
    padding: 8vh 7vw 12vh;
    display: grid;
    gap: 28px;
    position: relative;
    overflow: hidden;
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

  .hero {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .kicker {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.3em;
    color: rgba(175, 213, 246, 0.75);
    margin: 0 0 8px;
  }

  h1 {
    margin: 0 0 10px;
    font-size: clamp(2.4rem, 3.6vw, 3.6rem);
  }

  .lead {
    margin: 0;
    font-size: 1.05rem;
    color: rgba(220, 232, 245, 0.8);
    max-width: 700px;
  }

  .cta {
    align-self: flex-start;
    padding: 12px 22px;
    border-radius: 999px;
    background: #7fd0ff;
    color: #0c1824;
    font-weight: 600;
    text-decoration: none;
    box-shadow: 0 18px 40px rgba(127, 208, 255, 0.3);
    transition: transform 160ms ease, box-shadow 160ms ease;
  }

  .cta:hover {
    transform: translateY(-2px);
    box-shadow: 0 24px 48px rgba(127, 208, 255, 0.4);
  }

  .panel {
    position: relative;
    z-index: 1;
    background: rgba(14, 20, 32, 0.82);
    border-radius: 22px;
    padding: 22px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(10px);
  }

  .panel-head {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    margin-bottom: 14px;
  }

  h2 {
    margin: 0;
    font-size: 1.2rem;
  }

  .ghost {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #f1f4f9;
    padding: 10px 16px;
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
    gap: 12px;
    align-items: center;
    padding: 6px 0;
    min-width: 600px;
  }

  .term {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .coef {
    width: 80px;
    padding: 8px 10px;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: rgba(255, 255, 255, 0.04);
    color: #f1f4f9;
    font-family: 'Literata', serif;
  }

  .mul,
  .plus {
    color: rgba(160, 198, 230, 0.9);
    font-weight: 600;
  }

  .var-chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.08);
  }

  .var {
    width: 56px;
    background: transparent;
    border: none;
    color: #f1f4f9;
    font-weight: 600;
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
    /* align-items: center; */
    /* padding: 14px 12px; */
    /* border-radius: 16px; */
    /* background: rgba(9, 12, 20, 0.6); */
    /* border: 1px solid rgba(255, 255, 255, 0.08); */
  }

  .sign {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #f1f4f9;
    padding: 8px 10px;
    border-radius: 10px;
    font-weight: 600;
  }

  .rhs {
    width: 100px;
    padding: 8px 10px;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: rgba(255, 255, 255, 0.04);
    color: #f1f4f9;
    font-family: 'Literata', serif;
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
  .rhs:focus,
  .var:focus {
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
