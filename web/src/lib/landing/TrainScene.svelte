<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { useTask, useThrelte } from "@threlte/core";
  import { Matrix4, PerspectiveCamera, Vector3 } from "three";
  import { ComposerEngine } from "./engine/composer-engine.ts";
  import { ParticleSystem } from "./engine/particles.ts";
  import { RestBaker } from "./engine/rest-baker.ts";
  import { MouseSim } from "./engine/mouse-sim.ts";
  import { loadModelPoints, type ModelData } from "./engine/model-loader.ts";
  import { createModelTexture } from "./engine/model-texture.ts";
  import { DEFAULT_SETTINGS, type SystemSettings } from "./engine/types.ts";

  let { trainCount = 2, wagonCount = 6, section = 0 } = $props();

  const { renderer, scene, camera, invalidate } = useThrelte();

  let settings: SystemSettings = {
    ...DEFAULT_SETTINGS,
    pointSize: 0.28,
    bloomStrength: 0.9,
    trailIntensity: 0.28,
    cursorRepulsion: 0.15,
  };

  const viewProj = new Matrix4();
  const camRight = new Vector3();
  const camUp = new Vector3();
  const desiredCamPos = new Vector3();
  const desiredCamTarget = new Vector3();

  let composer: ComposerEngine | null = null;
  let particles: ParticleSystem | null = null;
  let restBaker: RestBaker | null = null;
  let mouseSim: MouseSim | null = null;
  let baseModel: ModelData | null = null;
  let handlePointerMove: ((event: PointerEvent) => void) | null = null;
  let handleMouseMove: ((event: MouseEvent) => void) | null = null;
  let lastWidth = 0;
  let lastHeight = 0;
  let ready = $state(false);
  let elapsed = 0;

  let mouseNormX = 0;
  let mouseNormY = 0;

  const MAX_MODEL_POINTS = 512 * 256;
  const WAGON_SPACING = 12;
  const TRAIN_SPACING = 16;

  function setMousePosition(clientX: number, clientY: number) {
    const rect = renderer.domElement.getBoundingClientRect();
    const rw = rect.width > 1e-4 ? rect.width : window.innerWidth;
    const rh = rect.height > 1e-4 ? rect.height : window.innerHeight;
    mouseNormX = ((clientX - rect.left) / rw) * 2 - 1;
    mouseNormY = ((clientY - rect.top) / rh) * 2 - 1;
  }

  function buildTrainModel(model: ModelData): ModelData {
    const baseCount = model.positions.length / 3;
    const segmentCount = wagonCount + 1;
    const totalSegments = segmentCount * trainCount;
    const pointsPerSegment = Math.max(
      1,
      Math.floor(MAX_MODEL_POINTS / Math.max(totalSegments, 1)),
    );
    const sampleStep = Math.max(1, Math.floor(baseCount / pointsPerSegment));
    const segmentPoints = Math.min(baseCount, pointsPerSegment);
    const totalPoints = Math.min(segmentPoints * totalSegments, MAX_MODEL_POINTS);

    const positions = new Float32Array(totalPoints * 3);
    const colors = model.colors ? new Float32Array(totalPoints * 3) : null;

    let writeIndex = 0;
    let colorIndex = 0;

    for (let t = 0; t < trainCount; t++) {
      const trainOffsetX = (t - (trainCount - 1) / 2) * TRAIN_SPACING;
      for (let s = 0; s < segmentCount; s++) {
        const segOffsetZ = -s * WAGON_SPACING;
        for (let i = 0; i < segmentPoints; i++) {
          if (writeIndex >= totalPoints * 3) break;
          const sourceIndex = ((i * sampleStep) % baseCount) * 3;
          positions[writeIndex++] = model.positions[sourceIndex] + trainOffsetX;
          positions[writeIndex++] = model.positions[sourceIndex + 1];
          positions[writeIndex++] = model.positions[sourceIndex + 2] + segOffsetZ;

          if (colors) {
            colors[colorIndex++] = model.colors?.[sourceIndex] ?? 1;
            colors[colorIndex++] = model.colors?.[sourceIndex + 1] ?? 1;
            colors[colorIndex++] = model.colors?.[sourceIndex + 2] ?? 1;
          }
        }
      }
    }

    return { positions, colors };
  }

  function disposeScene() {
    if (particles && scene) particles.dispose(scene);
    restBaker?.dispose();
    mouseSim?.dispose();
    composer?.dispose();
    particles = null;
    restBaker = null;
    mouseSim = null;
    composer = null;
  }

  function rebuildPipeline() {
    if (!baseModel) return;

    const combined = buildTrainModel(baseModel);
    const count = Math.min(combined.positions.length / 3, MAX_MODEL_POINTS);
    settings = { ...settings, particleCount: count };

    disposeScene();

    restBaker = new RestBaker(renderer, count);
    restBaker.setCount(count);

    particles = new ParticleSystem();
    particles.init(scene, count, settings.pointSize);

    const texture = createModelTexture(combined);
    restBaker.setModelTexture(0, texture, count);

    particles.setRestTextures(
      restBaker.getPosTexture(0),
      restBaker.getColTexture(0),
      restBaker.getPosTexture(1),
      restBaker.getColTexture(1),
    );

    mouseSim = new MouseSim(renderer, count);
    mouseSim.setRestTextures(restBaker.getPosTexture(0), restBaker.getPosTexture(1));
    particles.setDispTexture(mouseSim.getDispTexture());

    const width = renderer.domElement.clientWidth || window.innerWidth;
    const height = renderer.domElement.clientHeight || window.innerHeight;
    lastWidth = width;
    lastHeight = height;
    const cam = camera.current as PerspectiveCamera;
    composer = new ComposerEngine(renderer, scene, cam, settings, width, height);
    cam.position.set(0, 28, 86);
    cam.lookAt(0, 0, 0);
  }

  onMount(async () => {
    baseModel = await loadModelPoints("/landing/models/racecar.pts");
    ready = true;

    handlePointerMove = (event: PointerEvent) => {
      if (event.pointerType !== "mouse") return;
      setMousePosition(event.clientX, event.clientY);
    };
    handleMouseMove = (event: MouseEvent) => {
      if (!window.matchMedia("(hover: hover) and (pointer: fine)").matches) {
        return;
      }
      setMousePosition(event.clientX, event.clientY);
    };

    window.addEventListener("pointermove", handlePointerMove);
    window.addEventListener("mousemove", handleMouseMove);
  });

  onDestroy(() => {
    if (handlePointerMove) {
      window.removeEventListener("pointermove", handlePointerMove);
    }
    if (handleMouseMove) {
      window.removeEventListener("mousemove", handleMouseMove);
    }
    disposeScene();
  });

  $effect(() => {
    if (!ready) return;
    trainCount;
    wagonCount;
    rebuildPipeline();
    invalidate();
  });

  useTask((delta) => {
    if (!particles || !restBaker || !mouseSim || !composer) return;

    const dt = Number.isFinite(delta) ? delta : 1 / 60;
    elapsed += dt;
    const time = elapsed;
    const width = renderer.domElement.clientWidth || window.innerWidth;
    const height = renderer.domElement.clientHeight || window.innerHeight;
    if (width !== lastWidth || height !== lastHeight) {
      lastWidth = width;
      lastHeight = height;
      composer.setSize(width, height);
    }

    const intro = Math.min((time + 1) / 3, 1.5);
    const screenScale = composer.getScreenScale();

    particles.setPointSize(settings.pointSize);
    particles.setHdrIntensity(settings.hdrIntensity * screenScale);
    particles.setIntroProgress(intro);
    particles.setTime(time);
    particles.setBlend(0);
    particles.setMorphT(0);
    particles.setSeparation(0);
    particles.setColorMode(0);
    particles.setDof(settings.dofAmount, settings.dofFocus);
    particles.setFog(0, 10, 180);

    const isDetail = section === 1;
    const baseSpin = 0;
    const shimmer = isDetail ? 0.35 : 0.5;
    const wiggleZ = isDetail ? 0.2 : 0.35;
    const wiggleY = isDetail ? 1.9 : 2.8;
    const wiggleSpeed = isDetail ? 0.55 : 0.75;

    const controls = [
      48,
      baseSpin,
      shimmer,
      10,
      wiggleZ,
      wiggleY,
      wiggleSpeed,
      0,
    ];

    restBaker.bake(0, 0, controls, time);

    const cam = camera.current as PerspectiveCamera;
    desiredCamPos.set(0, isDetail ? 36 : 28, isDetail ? 120 : 86);
    desiredCamTarget.set(0, isDetail ? 10 : 0, isDetail ? -18 : -6);
    cam.position.lerp(desiredCamPos, 0.04);
    cam.lookAt(desiredCamTarget);
    cam.updateMatrixWorld();

    viewProj.multiplyMatrices(cam.projectionMatrix, cam.matrixWorldInverse);
    const ew = cam.matrixWorld.elements;
    camRight.set(ew[0], ew[1], ew[2]).normalize();
    camUp.set(ew[4], ew[5], ew[6]).normalize();

    mouseSim.setViewProj(viewProj);
    mouseSim.setCamBasis(camRight, camUp);
    mouseSim.setMouseNDC(mouseNormX, -mouseNormY);
    mouseSim.setBlend(0);
    mouseSim.setMorphT(0);
    mouseSim.setMouseNdcRadius(0.154);
    mouseSim.setMouseStrength(settings.cursorRepulsion * 3900);
    mouseSim.step(dt);
    particles.setDispTexture(mouseSim.getDispTexture());

    composer.afterImagePass.uniforms.damp.value = settings.trailIntensity;
    composer.updateSettings(settings);
    composer.render(time);
  });
</script>
