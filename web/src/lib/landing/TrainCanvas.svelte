<script lang="ts">
  import { Canvas } from "@threlte/core";
  import TrainScene from "./TrainScene.svelte";

  let { trainCount = 2, wagonCount = 5, section = 0 } = $props();
</script>

<div class="canvas-shell" aria-hidden="true">
  <Canvas>
    <TrainScene {trainCount} {wagonCount} {section} />
  </Canvas>
</div>

<style>
  .canvas-shell {
    position: fixed;
    inset: 0;
    z-index: 1;
    pointer-events: none;
  }

  .canvas-shell :global(canvas) {
    display: block;
    width: 100%;
    height: 100%;
  }
</style>
