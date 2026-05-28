<script lang="ts">
  type SolveActionProps = {
    isSolving: boolean;
    onSolve: () => void | Promise<void>;
    secondaryHref?: string;
    secondaryLabel?: string;
    solveLabel?: string;
  };

  let {
    isSolving,
    onSolve,
    secondaryHref,
    secondaryLabel,
    solveLabel = 'Розв’язати',
  }: SolveActionProps = $props();
</script>

<div class="solve-row">
  <progress
    class={`solve-progress${isSolving ? ' is-active' : ''}`}
    aria-label="Розв’язання"
  ></progress>
  <div class="solve-actions">
    {#if secondaryHref && secondaryLabel}
      <a class="btn btn-secondary" href={secondaryHref}>{secondaryLabel}</a>
    {/if}
    <button class="btn btn-primary" type="button" onclick={onSolve}>
      {isSolving ? 'Розв’язання...' : solveLabel}
    </button>
  </div>
</div>

<style>
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
</style>
