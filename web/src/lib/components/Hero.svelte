<script lang="ts">
  // use `$props()` for runes-mode compatibility
  // @ts-ignore
  const { kicker = '' } = $props();
</script>

<div class="hero">
  <p class="kicker">{kicker}</p>
  <h1><slot /></h1>
</div>

<style>
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
</style>
