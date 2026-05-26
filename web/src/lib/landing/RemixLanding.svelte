<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Matrix4, Vector3 } from "three";
  import { ControlManager } from "./engine/controls.ts";
  import { Engine } from "./engine/engine.ts";
  import {
    projectLabelsInto,
    type ProjectedLabel,
  } from "./engine/label-projection.ts";
  import { MouseSim } from "./engine/mouse-sim.ts";
  import { ParticleSystem } from "./engine/particles.ts";
  import { RestBaker } from "./engine/rest-baker.ts";
  import { getMorphBlend, type MorphBlend } from "./engine/morph.ts";
  import { createModelTexture } from "./engine/model-texture.ts";
  import { loadModelPoints, type ModelData } from "./engine/model-loader.ts";
  import { presets } from "./engine/presets.ts";
  import { DEFAULT_SETTINGS, type ControlDef, type Preset, type ShaderId } from "./engine/types.ts";
  import { clamp, clamp01, lerp } from "./utils/math.ts";
  import { initReducedMotion, reducedMotion } from "./utils/reduced-motion.ts";

  let { presetIndex = null, showContent = false } = $props();

  const PARTICLE_INTRO_DELAY_S = 1;
  const DEFAULT_CAM_POS: [number, number, number] = [0, 30, 80];
  const DEFAULT_CAM_TARGET: [number, number, number] = [0, 0, 0];
  const CAM_LERP_SPEED = 0.025;

  const SHADER_ID_TO_INT: Record<ShaderId, number> = {
    racetrack: 0,
    racecar: 1,
    runner: 2,
    remixLogo: 3,
    racetrackCar: 4,
  };

  function resolveShaderInt(preset: Preset): number {
    return SHADER_ID_TO_INT[preset.shaderId];
  }

  type PresetRuntimeData = {
    presets: Preset[];
    controls: number[][];
    shaderInts: number[];
    racetrackIndex: number;
    driveIndex: number;
    driveCarPosY: number;
  };

  function setDesiredCameraInto(
    presets: Preset[],
    morphValue: number,
    outPos: Vector3,
    outTarget: Vector3,
  ) {
    const maxIdx = presets.length - 1;
    const clamped = clamp(morphValue, 0, maxIdx);
    const fromIdx = Math.min(Math.floor(clamped), maxIdx);
    const toIdx = Math.min(fromIdx + 1, maxIdx);
    const blend = clamped - fromIdx;

    const fromPos = presets[fromIdx].cameraPosition ?? DEFAULT_CAM_POS;
    const fromTarget = presets[fromIdx].cameraTarget ?? DEFAULT_CAM_TARGET;
    const toPos = presets[toIdx].cameraPosition ?? DEFAULT_CAM_POS;
    const toTarget = presets[toIdx].cameraTarget ?? DEFAULT_CAM_TARGET;

    outPos.set(
      lerp(fromPos[0], toPos[0], blend),
      lerp(fromPos[1], toPos[1], blend),
      lerp(fromPos[2], toPos[2], blend),
    );
    outTarget.set(
      lerp(fromTarget[0], toTarget[0], blend),
      lerp(fromTarget[1], toTarget[1], blend),
      lerp(fromTarget[2], toTarget[2], blend),
    );
  }

  function copyControlsInto(source: number[], target: number[]) {
    for (let i = 0; i < 8; i++) {
      target[i] = source[i] ?? 0;
    }
  }

  function copyManagedControlsInto(
    preset: Preset,
    controlMgr: ControlManager,
    target: number[],
  ) {
    for (let i = 0; i < 8; i++) {
      const control = preset.controls[i];
      target[i] = control
        ? (controlMgr.controls.get(control.id)?.value ?? control.initial)
        : 0;
    }
  }

  function buildInitialControls(preset: Preset): number[] {
    const controls = [0, 0, 0, 0, 0, 0, 0, 0];
    for (let i = 0; i < Math.min(preset.controls.length, 8); i++) {
      controls[i] = preset.controls[i].initial;
    }
    return controls;
  }

  function getControlInitial(preset: Preset, id: string, fallback = 0): number {
    return (
      preset.controls.find((control) => control.id === id)?.initial ?? fallback
    );
  }

  function buildPresetRuntimeData(presets: Preset[]): PresetRuntimeData {
    const driveIndex = presets.findIndex((preset) => preset.name === "Drive");
    return {
      presets,
      controls: presets.map(buildInitialControls),
      shaderInts: presets.map(resolveShaderInt),
      racetrackIndex: presets.findIndex((preset) => preset.name === "Racetrack"),
      driveIndex,
      driveCarPosY:
        driveIndex >= 0
          ? getControlInitial(presets[driveIndex], "_carPosY", 0)
          : 0,
    };
  }

  let containerEl: HTMLDivElement | null = null;
  let canvasEl: HTMLCanvasElement | null = null;
  let engine: Engine | null = null;
  let particles: ParticleSystem | null = null;
  let restBaker: RestBaker | null = null;
  let mouseSim: MouseSim | null = null;
  const appliedModelSlots = new Set<number>();
  let frameId = 0;
  let startTime = 0;
  let frozenTime: number | null = null;
  let previousNearest = -1;
  let hasReportedReady = false;
  let initFailed = false;
  const labelControlMgr = new ControlManager();
  const desiredCameraPos = new Vector3();
  const desiredCameraTarget = new Vector3();
  const scratchViewProj = new Matrix4();
  const scratchCamRight = new Vector3();
  const scratchCamUp = new Vector3();
  let lastFrameNow = 0;
  const scratchControlsA = [0, 0, 0, 0, 0, 0, 0, 0];
  const scratchControlsB = [0, 0, 0, 0, 0, 0, 0, 0];
  const scratchLabelControls = [0, 0, 0, 0, 0, 0, 0, 0];
  const morphBlend: MorphBlend = { fromIndex: 0, toIndex: 0, blend: 0 };
  let presetRuntimeData: PresetRuntimeData | null = null;
  const settings = DEFAULT_SETTINGS;

  let labels = $state<ProjectedLabel[]>([]);
  let labelOpacity = $state(0);
  let controlList = $state<ControlDef[]>([]);

  const morphValueRef = { current: 0 };
  let targetPreset = $state<number | null>(null);
  let activePreset = 0;
  let transitionBlend = 0;
  let transitionFrom = 0;
  let transitionTo = 0;
  let transitionActive = false;
  let hasPresetSelection = false;
  const modelData: (ModelData | undefined)[] = Array.from({
    length: presets.length,
  });

  let mouseNormX = 0;
  let mouseNormY = 0;
  let prevMouseNormX = 0;
  let prevMouseNormY = 0;
  let mouseVelPrimed = false;
  let mouseNdcSpeedSmoothed = 0;
  let mouseBrushSmoothed = 0;
  let smoothMouseOffsetX = 0;
  let smoothCarLane = 0;
  let prevCarLane = 0;
  let laneActivity = 0;

  const MOUSE_RANGE = 1;
  const MOUSE_LERP = 0.04;
  const CAR_LANE_LERP = 0.06;
  const ACTIVITY_DECAY = 0.97;
  const ACTIVITY_GAIN = 20.0;
  const RACETRACK_MOUSE_STRAFE_ATTENUATION = 0.4;
  const RACETRACK_MOUSE_STRAFE_OF_TRACKW = 0.18;

  const MOUSE_SIM_STRENGTH_SCALE = 3900;
  const MOUSE_SIM_REPULSION_REF = 0.2;
  const MOUSE_SIM_PEAK_DISP = 17.0;
  const MOUSE_SIM_FOLLOW_TAU = 10;
  const MOUSE_SIM_NDC_RADIUS = 0.154;
  const MOUSE_SIM_VEL_SMOOTH_TAU = 22;
  const MOUSE_SIM_VEL_GATE = 0.14;
  const MOUSE_SIM_VEL_FULL = 5.5;
  const MOUSE_SIM_BRUSH_SMOOTH_TAU = 14;
  const MOUSE_SIM_PUSH_GAIN =
    MOUSE_SIM_PEAK_DISP / (MOUSE_SIM_STRENGTH_SCALE * MOUSE_SIM_REPULSION_REF);

  function setMousePosition(clientX: number, clientY: number) {
    const vp = containerEl ?? canvasEl;
    if (vp) {
      const rect = vp.getBoundingClientRect();
      const rw = rect.width > 1e-4 ? rect.width : window.innerWidth;
      const rh = rect.height > 1e-4 ? rect.height : window.innerHeight;
      mouseNormX = ((clientX - rect.left) / rw) * 2 - 1;
      mouseNormY = ((clientY - rect.top) / rh) * 2 - 1;
    } else {
      mouseNormX = (clientX / window.innerWidth) * 2 - 1;
      mouseNormY = (clientY / window.innerHeight) * 2 - 1;
    }
  }

  function getPresetRuntimeData(presets: Preset[]) {
    if (presetRuntimeData?.presets === presets) return presetRuntimeData;
    presetRuntimeData = buildPresetRuntimeData(presets);
    return presetRuntimeData;
  }

  function disposeScene() {
    if (typeof cancelAnimationFrame !== "undefined") {
      cancelAnimationFrame(frameId);
    }
    if (particles && engine) {
      particles.dispose(engine.scene);
    }
    mouseSim?.dispose();
    restBaker?.dispose();
    appliedModelSlots.clear();
    engine?.dispose();
    particles = null;
    restBaker = null;
    mouseSim = null;
    engine = null;
    mouseVelPrimed = false;
    mouseNdcSpeedSmoothed = 0;
    mouseBrushSmoothed = 0;
  }

  function syncModelTextures() {
    if (!restBaker) return;

    for (const preset of presets) {
      if (
        preset.modelUrl == null ||
        preset.modelSlot == null ||
        appliedModelSlots.has(preset.modelSlot)
      ) {
        continue;
      }

      const model = modelData[presets.indexOf(preset)];
      if (!model) continue;

      restBaker.setModelTexture(
        preset.modelSlot,
        createModelTexture(model),
        model.positions.length / 3,
      );
      appliedModelSlots.add(preset.modelSlot);
    }
  }

  function maybeInit() {
    if (initFailed || engine || !containerEl || !canvasEl) {
      return;
    }

    try {
      engine = new Engine();
      engine.init(canvasEl, containerEl, settings);

      restBaker = new RestBaker(engine.renderer, settings.particleCount);
      restBaker.setCount(settings.particleCount);

      particles = new ParticleSystem();
      particles.init(engine.scene, settings.particleCount, settings.pointSize);
      particles.setRestTextures(
        restBaker.getPosTexture(0),
        restBaker.getColTexture(0),
        restBaker.getPosTexture(1),
        restBaker.getColTexture(1),
      );
      syncModelTextures();

      mouseSim = new MouseSim(engine.renderer, settings.particleCount);
      mouseSim.setRestTextures(
        restBaker.getPosTexture(0),
        restBaker.getPosTexture(1),
      );
      mouseSim.setPushGain(MOUSE_SIM_PUSH_GAIN);
      mouseSim.setFollowTau(MOUSE_SIM_FOLLOW_TAU);
      particles.setDispTexture(mouseSim.getDispTexture());

      startTime = performance.now() / 1000;
      setDesiredCameraInto(
        presets,
        morphValueRef.current,
        desiredCameraPos,
        desiredCameraTarget,
      );
      engine.camera.position.copy(desiredCameraPos);
      engine.controls.target.copy(desiredCameraTarget);

      const initialPresetData = getPresetRuntimeData(presets);
      const initialIndex = Math.min(
        Math.max(0, Math.floor(morphValueRef.current)),
        initialPresetData.presets.length - 1,
      );
      copyControlsInto(
        initialPresetData.controls[initialIndex],
        scratchControlsA,
      );
      restBaker.bake(
        0,
        initialPresetData.shaderInts[initialIndex],
        scratchControlsA,
        0,
      );
      engine.renderer.compile(engine.scene, engine.camera);
    } catch (error) {
      initFailed = true;
      disposeScene();
      console.error(error);
      return;
    }

    const animate = () => {
      if (!engine || !particles || !restBaker || !mouseSim) {
        return;
      }

      const now = performance.now();
      const time = now / 1000 - startTime;
      const dtSeconds =
        lastFrameNow === 0 ? 1 / 60 : (now - lastFrameNow) / 1000;
      lastFrameNow = now;
      if (targetPreset !== null && transitionActive) {
        const speed = 2.8;
        transitionBlend = Math.min(transitionBlend + dtSeconds * speed, 1);
        if (transitionBlend >= 1) {
          transitionActive = false;
          activePreset = transitionTo;
        }
      }
      const presetData = getPresetRuntimeData(presets);
      const morphValue = transitionActive
        ? lerp(transitionFrom, transitionTo, transitionBlend)
        : activePreset;
      const reduceMotion = reducedMotion.current;

      if (reduceMotion) {
        frozenTime ??= Math.max(time, PARTICLE_INTRO_DELAY_S + 3.5);
      } else {
        frozenTime = null;
      }
      const visualTime = frozenTime ?? time;

      engine.updateSettings(settings);

      const screenScale = engine.getScreenScale();
      particles.setPointSize(settings.pointSize);
      particles.setHdrIntensity(settings.hdrIntensity * screenScale);
      const effectiveMouseNormX = reduceMotion ? 0 : mouseNormX;
      const effectiveMouseNormY = reduceMotion ? 0 : mouseNormY;

      let mouseBrushFactor = 0;
      const dtClamp = Math.max(dtSeconds, 1e-4);
      if (reduceMotion) {
        mouseVelPrimed = false;
        mouseNdcSpeedSmoothed = 0;
        mouseBrushSmoothed = 0;
      } else {
        if (!mouseVelPrimed) {
          prevMouseNormX = effectiveMouseNormX;
          prevMouseNormY = effectiveMouseNormY;
          mouseVelPrimed = true;
        } else {
          const speed = Math.hypot(
            (effectiveMouseNormX - prevMouseNormX) / dtClamp,
            (effectiveMouseNormY - prevMouseNormY) / dtClamp,
          );
          const kVel = 1 - Math.exp(-MOUSE_SIM_VEL_SMOOTH_TAU * dtClamp);
          mouseNdcSpeedSmoothed += (speed - mouseNdcSpeedSmoothed) * kVel;
        }
        prevMouseNormX = effectiveMouseNormX;
        prevMouseNormY = effectiveMouseNormY;
        const span = Math.max(MOUSE_SIM_VEL_FULL - MOUSE_SIM_VEL_GATE, 1e-4);
        const linear = clamp01(
          (mouseNdcSpeedSmoothed - MOUSE_SIM_VEL_GATE) / span,
        );
        const brushTarget = linear * linear * (3 - 2 * linear);
        const kBrush = 1 - Math.exp(-MOUSE_SIM_BRUSH_SMOOTH_TAU * dtClamp);
        mouseBrushSmoothed += (brushTarget - mouseBrushSmoothed) * kBrush;
        mouseBrushFactor = mouseBrushSmoothed;
      }

      particles.setColorMode(settings.colorMode);
      particles.setDof(settings.dofAmount, settings.dofFocus);
      const introTime = Math.max(0, visualTime - PARTICLE_INTRO_DELAY_S);
      particles.setIntroProgress(
        reduceMotion ? 1.5 : Math.min(introTime / 3.5, 1.5),
      );
      particles.setTime(visualTime);

      const maxValue = presets.length - 1;
      if (targetPreset === null) {
        getMorphBlend(morphValue, maxValue, morphBlend);
      } else {
        morphBlend.fromIndex = transitionActive ? transitionFrom : activePreset;
        morphBlend.toIndex = transitionActive ? transitionTo : activePreset;
        morphBlend.blend = transitionActive ? transitionBlend : 0;
      }
      const { fromIndex, toIndex, blend } = morphBlend;

      let separation: number;

      if (blend < 0.001) {
        copyControlsInto(presetData.controls[fromIndex], scratchControlsA);
        copyControlsInto(presetData.controls[fromIndex], scratchControlsB);
        separation = presets[fromIndex].separation;
      } else {
        copyControlsInto(presetData.controls[fromIndex], scratchControlsA);
        copyControlsInto(presetData.controls[toIndex], scratchControlsB);
        const easedBlend = blend * blend * (3 - 2 * blend);
        separation =
          presets[fromIndex].separation * (1 - easedBlend) +
          presets[toIndex].separation * easedBlend;
      }

      const racetrackIndex = presetData.racetrackIndex;
      const racetrackDist =
        racetrackIndex >= 0 ? Math.abs(morphValue - racetrackIndex) : 0;
      const departingRacetrack =
        !reduceMotion && racetrackDist > 0.01 && racetrackDist < 1.0;

      if (departingRacetrack) {
        const surge = racetrackDist * racetrackDist * 32;
        if (blend < 0.001) {
          if (fromIndex === racetrackIndex || toIndex === racetrackIndex) {
            scratchControlsA[7] = surge;
            scratchControlsB[7] = surge;
          }
        } else {
          if (fromIndex === racetrackIndex) scratchControlsA[7] = surge;
          if (toIndex === racetrackIndex) scratchControlsB[7] = surge;
        }
      }

      let morphT = 0;
      if (blend > 0.001) {
        const ease = settings.morphEase;
        const tk = Math.pow(blend, ease);
        morphT = tk / (tk + Math.pow(1 - blend, ease));
      }
      particles.setBlend(blend);
      particles.setMorphT(morphT);
      particles.setSeparation(separation);

      const overridesA = presets[fromIndex].systemOverrides;
      const overridesB = presets[toIndex].systemOverrides;
      const easedBlend = blend * blend * (3 - 2 * blend);
      const effectiveTrail =
        (1 - easedBlend) *
          (overridesA?.trailIntensity ?? settings.trailIntensity) +
        easedBlend * (overridesB?.trailIntensity ?? settings.trailIntensity);
      const effectiveRepulsion =
        (1 - easedBlend) *
          (overridesA?.cursorRepulsion ?? settings.cursorRepulsion) +
        easedBlend * (overridesB?.cursorRepulsion ?? settings.cursorRepulsion);
      const trailBoost = departingRacetrack
        ? Math.sin(racetrackDist * Math.PI) * 0.75
        : 0;
      engine.afterImagePass.uniforms.damp.value = reduceMotion
        ? 0
        : Math.min(effectiveTrail + trailBoost, 0.97);

      const driveIndex = presetData.driveIndex;
      const racetrackFogDist =
        racetrackIndex >= 0 ? Math.abs(morphValue - racetrackIndex) : Infinity;
      const driveFogDist =
        driveIndex >= 0 ? Math.abs(morphValue - driveIndex) : Infinity;
      const fogProximity = Math.max(
        0,
        1 - Math.min(racetrackFogDist, driveFogDist),
      );
      particles.setFog(fogProximity, 10, 180);

      const driveProximity =
        driveIndex >= 0 ? clamp01(1 - Math.abs(morphValue - driveIndex)) : 0;
      const racetrackRoadLock =
        racetrackIndex >= 0
          ? clamp01(1 - racetrackDist) * (1 - driveProximity)
          : 0;
      if (!reduceMotion && driveProximity > 0) {
        smoothCarLane += (effectiveMouseNormX - smoothCarLane) * CAR_LANE_LERP;
      } else {
        smoothCarLane += (0 - smoothCarLane) * CAR_LANE_LERP;
      }

      const laneDelta = Math.abs(smoothCarLane - prevCarLane);
      laneActivity = Math.max(
        laneActivity * ACTIVITY_DECAY,
        clamp01(laneDelta * ACTIVITY_GAIN),
      );
      prevCarLane = smoothCarLane;

      const carLaneOffset = smoothCarLane * driveProximity;
      const carLaneActivity = laneActivity * driveProximity;
      const carPosY =
        driveIndex >= 0 ? presetData.driveCarPosY * driveProximity : 0;
      restBaker.setCarUniforms(carLaneOffset, carLaneActivity, carPosY);

      restBaker.bake(
        0,
        presetData.shaderInts[fromIndex],
        scratchControlsA,
        visualTime,
      );
      if (blend > 0.001) {
        restBaker.bake(
          1,
          presetData.shaderInts[toIndex],
          scratchControlsB,
          visualTime,
        );
      }

      engine.controls.enabled = !reduceMotion && driveProximity < 0.5;

      setDesiredCameraInto(
        presets,
        morphValue,
        desiredCameraPos,
        desiredCameraTarget,
      );
      if (reduceMotion) {
        engine.camera.position.copy(desiredCameraPos);
        engine.controls.target.copy(desiredCameraTarget);
      } else {
        engine.camera.position.lerp(desiredCameraPos, CAM_LERP_SPEED);
        engine.controls.target.lerp(desiredCameraTarget, CAM_LERP_SPEED);
      }

      const parallaxScale = 1 - driveProximity;
      smoothMouseOffsetX +=
        (effectiveMouseNormX * MOUSE_RANGE - smoothMouseOffsetX) * MOUSE_LERP;
      if (!reduceMotion) {
        const parallaxUncapped = smoothMouseOffsetX * parallaxScale;
        let parallaxX = parallaxUncapped;
        if (racetrackIndex >= 0 && racetrackRoadLock > 0) {
          const trackW = presetData.controls[racetrackIndex][1] ?? 40;
          const strafeCap = trackW * RACETRACK_MOUSE_STRAFE_OF_TRACKW;
          const parallaxRacetrack = clamp(
            parallaxUncapped * RACETRACK_MOUSE_STRAFE_ATTENUATION,
            -strafeCap,
            strafeCap,
          );
          parallaxX = lerp(
            parallaxUncapped,
            parallaxRacetrack,
            racetrackRoadLock,
          );
        }
        engine.camera.position.x += parallaxX;
      }

      engine.controls.update();
      scratchViewProj.multiplyMatrices(
        engine.camera.projectionMatrix,
        engine.camera.matrixWorldInverse,
      );
      const ew = engine.camera.matrixWorld.elements;
      scratchCamRight.set(ew[0], ew[1], ew[2]).normalize();
      scratchCamUp.set(ew[4], ew[5], ew[6]).normalize();
      mouseSim.setViewProj(scratchViewProj);
      mouseSim.setCamBasis(scratchCamRight, scratchCamUp);
      mouseSim.setMouseNDC(effectiveMouseNormX, -effectiveMouseNormY);
      mouseSim.setBlend(blend);
      mouseSim.setMorphT(morphT);
      mouseSim.setMouseNdcRadius(MOUSE_SIM_NDC_RADIUS * mouseBrushFactor);
      mouseSim.setMouseStrength(
        reduceMotion
          ? 0
          : effectiveRepulsion * MOUSE_SIM_STRENGTH_SCALE * mouseBrushFactor,
      );
      mouseSim.step(dtSeconds);
      particles.setDispTexture(mouseSim.getDispTexture());

      const nearest = Math.round(clamp(morphValue, 0, maxValue));
      if (nearest !== previousNearest) {
        previousNearest = nearest;
        labelControlMgr.loadPreset(presets[nearest]);
        controlList = Array.from(labelControlMgr.controls.values());
      }

      const nearestPreset = presets[nearest];
      if (nearestPreset?.labels && nearestPreset.labels.length > 0 && containerEl) {
        let activeCtrls = presetData.controls[nearest];
        if (blend < 0.001) {
          copyManagedControlsInto(
            nearestPreset,
            labelControlMgr,
            scratchLabelControls,
          );
          activeCtrls = scratchLabelControls;
        }

        projectLabelsInto(
          labels,
          nearestPreset,
          labelControlMgr,
          activeCtrls,
          visualTime,
          engine.camera,
          containerEl.clientWidth,
          containerEl.clientHeight,
        );

        const distFromNearest = Math.abs(morphValue - nearest);
        labelOpacity = Math.max(0, 1 - distFromNearest * 4);
      } else {
        labels.length = 0;
        labelOpacity = 0;
      }

      engine.render(time);
      if (!hasReportedReady) {
        hasReportedReady = true;
      }
      frameId = requestAnimationFrame(animate);
    };

    frameId = requestAnimationFrame(animate);
  }

  function updateScroll() {
    const maxValue = presets.length - 1;
    const maxScroll = Math.max(
      1,
      document.documentElement.scrollHeight - window.innerHeight,
    );
    const scrollTop = window.scrollY || document.documentElement.scrollTop;
    morphValueRef.current = clamp((scrollTop / maxScroll) * maxValue, 0, maxValue);
  }

  $effect(() => {
    if (presetIndex === null || Number.isNaN(presetIndex)) {
      targetPreset = null;
      return;
    }
    const maxValue = presets.length - 1;
    const nextPreset = clamp(presetIndex, 0, maxValue);
    targetPreset = nextPreset;

    if (!hasPresetSelection) {
      activePreset = nextPreset;
      transitionFrom = nextPreset;
      transitionTo = nextPreset;
      transitionBlend = 0;
      transitionActive = false;
      hasPresetSelection = true;
      morphValueRef.current = nextPreset;
      return;
    }

    if (nextPreset !== activePreset) {
      transitionFrom = activePreset;
      transitionTo = nextPreset;
      transitionBlend = 0;
      transitionActive = true;
    }
  });

  onMount(() => {
    const controller = new AbortController();
    const { signal } = controller;

    initReducedMotion(signal, () => {
      hasReportedReady = false;
    });

    window.addEventListener(
      "pointermove",
      (event) => {
        if (event.pointerType !== "mouse") return;
        setMousePosition(event.clientX, event.clientY);
      },
      { signal },
    );
    window.addEventListener(
      "mousemove",
      (event) => {
        if (!window.matchMedia("(hover: hover) and (pointer: fine)").matches) {
          return;
        }
        setMousePosition(event.clientX, event.clientY);
      },
      { signal },
    );

    if (presetIndex === null) {
      window.addEventListener("scroll", updateScroll, { signal, passive: true });
      updateScroll();
    }

    const load = async () => {
      await Promise.all(
        presets.map(async (preset, index) => {
          if (!preset.modelUrl) return;
          try {
            modelData[index] = await loadModelPoints(preset.modelUrl);
          } catch (error) {
            console.error(error);
          }
        }),
      );

      if (!engine && !initFailed) {
        maybeInit();
      }
    };
    void load();

    return () => {
      controller.abort();
      disposeScene();
    };
  });

  onDestroy(() => {
    disposeScene();
  });
</script>

<div class="landing">
  <div class="canvas-shell" bind:this={containerEl} aria-hidden="true">
    <canvas bind:this={canvasEl} class="canvas"></canvas>
    <div class="label-layer" style:opacity={labelOpacity}>
      {#each labels as label (label.id)}
        {#if label.visible}
          <div
            class="label"
            style={`transform: translate(${label.labelX}px, ${label.labelY}px); color: ${label.color ?? "#BFC7E2"};`}
          >
            {label.text}
          </div>
          <div
            class="label-dot"
            style={`transform: translate(${label.anchorX}px, ${label.anchorY}px); color: ${label.color ?? "#BFC7E2"};`}
          ></div>
        {/if}
      {/each}
    </div>
  </div>

  {#if false}
    <aside class="control-panel">
      <h3>Controls</h3>
      {#if controlList.length === 0}
        <p class="control-empty">No active preset controls.</p>
      {:else}
        <div class="control-list">
          {#each controlList as control (control.id)}
            <label class="control-item">
              <span>{control.label}</span>
              <input
                type="range"
                min={control.min}
                max={control.max}
                step="0.01"
                value={control.value}
                oninput={(event) => {
                  const next = Number((event.currentTarget as HTMLInputElement).value);
                  labelControlMgr.setControlValue(control.id, next);
                  controlList = Array.from(labelControlMgr.controls.values());
                }}
              />
              <strong>{control.value.toFixed(2)}</strong>
            </label>
          {/each}
        </div>
      {/if}
    </aside>
  {/if}

  {#if showContent}
    <main class="content">
      <section class="panel hero">
        <div class="hero-card">
          <p class="kicker">Train Solver</p>
          <h1>Two lines. One rhythm.</h1>
          <p class="lede">
            Scroll to morph through presets. Cursor drives the parallax and track
            steering. Labels follow the model the same way as the Remix landing.
          </p>
        </div>
      </section>
      <section class="panel detail">
        <div class="detail-card">
          <p class="kicker">Particle pass</p>
          <h2>Signal in the noise.</h2>
          <p>
            Multi-preset morphing is back. This is the original Remix landing
            pipeline, running inside Svelte.
          </p>
        </div>
      </section>
    </main>
  {/if}
</div>

<style>
  :global(body) {
    margin: 0;
    background: #020407;
    color: #e6edf5;
    font-family: "Space Grotesk", "Neue Haas Grotesk Display", "Helvetica Neue",
      Arial, sans-serif;
  }

  .landing {
    position: relative;
    /* min-height: 240vh; */
  }

  .canvas-shell {
    position: fixed;
    inset: 0;
    z-index: 1;
    pointer-events: none;
  }

  .canvas {
    display: block;
    width: 100%;
    height: 100%;
  }

  .label-layer {
    position: absolute;
    inset: 0;
    pointer-events: none;
    transition: opacity 0.2s ease;
  }

  .label {
    position: absolute;
    font-size: 12px;
    letter-spacing: 0.22em;
    font-weight: 600;
    text-transform: uppercase;
    transform: translate(0, 0);
    white-space: nowrap;
  }

  .label-dot {
    position: absolute;
    width: 6px;
    height: 6px;
    border-radius: 999px;
    background: currentColor;
  }

  .control-panel {
    position: fixed;
    left: 24px;
    bottom: 24px;
    z-index: 4;
    width: min(360px, 90vw);
    max-height: 60vh;
    overflow: auto;
    padding: 16px 18px;
    background: rgba(8, 12, 20, 0.8);
    border-radius: 16px;
    box-shadow: 0 18px 40px rgba(3, 6, 12, 0.6);
    backdrop-filter: blur(12px);
    pointer-events: auto;
  }

  .control-panel h3 {
    margin: 0 0 12px;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.2em;
  }

  .control-empty {
    margin: 0;
    color: rgba(220, 232, 245, 0.7);
  }

  .control-list {
    display: grid;
    gap: 12px;
  }

  .control-item {
    display: grid;
    gap: 6px;
    font-size: 12px;
  }

  .control-item span {
    color: rgba(220, 232, 245, 0.8);
  }

  .control-item input[type="range"] {
    width: 100%;
  }

  .control-item strong {
    font-size: 11px;
    color: rgba(220, 232, 245, 0.6);
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

  @media (max-width: 720px) {
    .panel {
      padding: 10vh 6vw;
    }

    .hero-card,
    .detail-card {
      padding: 24px;
    }
  }
</style>
