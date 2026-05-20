<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import TrainCanvas from "./TrainCanvas.svelte";

  let { trainCount = 2, wagonCount = 6 } = $props();

  let section = $state(0);
  let sectionOne: HTMLElement | null = null;
  let sectionTwo: HTMLElement | null = null;
  let observer: IntersectionObserver | null = null;

  onMount(() => {
    const handleEntries = (entries: IntersectionObserverEntry[]) => {
      const visible = entries
        .filter((entry) => entry.isIntersecting)
        .sort((a, b) => b.intersectionRatio - a.intersectionRatio);
      if (visible.length === 0) return;
      section = visible[0].target === sectionTwo ? 1 : 0;
    };

    observer = new IntersectionObserver(handleEntries, {
      root: null,
      rootMargin: "0px",
      threshold: [0.2, 0.4, 0.6, 0.8],
    });

    if (sectionOne) observer.observe(sectionOne);
    if (sectionTwo) observer.observe(sectionTwo);
  });

  onDestroy(() => {
    observer?.disconnect();
  });
</script>

<div class="landing">
  <TrainCanvas {trainCount} {wagonCount} {section} />

  <main class="content">
    <section class="panel hero" bind:this={sectionOne}>
      <div class="hero-card">
        <p class="kicker">Train Solver</p>
        <h1>Two lines. One rhythm.</h1>
        <p class="lede">
          Twin trains drift in sync. Wagons follow in a clean cadence. This is
          the new landing motion scaffold.
        </p>
        <div class="controls">
          <div>
            <span>Train count</span>
            <strong>{trainCount}</strong>
          </div>
          <div>
            <span>Wagons per train</span>
            <strong>{wagonCount}</strong>
          </div>
        </div>
      </div>
    </section>

    <section class="panel detail" bind:this={sectionTwo}>
      <div class="detail-card">
        <p class="kicker">Particle pass</p>
        <h2>Signal in the noise.</h2>
        <p>
          The train collapses into particles and floats in the background.
          Camera slows, spin eases, and the surface glow stays intact.
        </p>
      </div>
    </section>
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    background: #020407;
    color: #e6edf5;
    font-family: "Space Grotesk", "Neue Haas Grotesk Display", "Helvetica Neue",
      Arial, sans-serif;
  }

  :global(a) {
    color: inherit;
  }

  .landing {
    position: relative;
    min-height: 200vh;
  }

  .content {
    position: relative;
    z-index: 2;
  }

  .panel {
    min-height: 100vh;
    display: grid;
    place-items: center;
    padding: 8vh 8vw;
  }

  .hero-card,
  .detail-card {
    max-width: 620px;
    padding: 32px 36px;
    border-radius: 24px;
    background: rgba(8, 12, 20, 0.76);
    box-shadow: 0 20px 60px rgba(3, 6, 12, 0.6);
    backdrop-filter: blur(12px);
  }

  .kicker {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.24em;
    margin: 0 0 12px;
    color: rgba(214, 225, 240, 0.75);
  }

  h1,
  h2 {
    margin: 0 0 16px;
    font-size: clamp(2.2rem, 3vw, 3.2rem);
  }

  .lede,
  .detail-card p {
    margin: 0 0 24px;
    font-size: 1.05rem;
    line-height: 1.6;
    color: rgba(220, 232, 245, 0.82);
  }

  .controls {
    display: flex;
    gap: 24px;
  }

  .controls div {
    display: grid;
    gap: 6px;
  }

  .controls span {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    color: rgba(198, 210, 224, 0.6);
  }

  .controls strong {
    font-size: 1.4rem;
  }

  @media (max-width: 720px) {
    .panel {
      padding: 10vh 6vw;
    }

    .hero-card,
    .detail-card {
      padding: 24px;
    }

    .controls {
      flex-direction: column;
      gap: 16px;
    }
  }
</style>
