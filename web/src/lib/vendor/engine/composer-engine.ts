import {
  Color,
  HalfFloatType,
  PerspectiveCamera,
  Scene,
  Vector2,
  WebGLRenderer,
  WebGLRenderTarget,
} from "three";
import { EffectComposer } from "three/examples/jsm/postprocessing/EffectComposer.js";
import { RenderPass } from "three/examples/jsm/postprocessing/RenderPass.js";
import { UnrealBloomPass } from "three/examples/jsm/postprocessing/UnrealBloomPass.js";
import { AfterimagePass } from "three/examples/jsm/postprocessing/AfterimagePass.js";
import { BackgroundPass } from "./background-pass.ts";
import type { SystemSettings } from "./types.ts";

function screenScale(width: number): number {
  const ref = 1440;
  return Math.min(width / ref, 1);
}

export class ComposerEngine {
  renderer: WebGLRenderer;
  scene: Scene;
  camera: PerspectiveCamera;
  composer: EffectComposer;
  afterImagePass: AfterimagePass;
  bloomPass: UnrealBloomPass;
  backgroundPass: BackgroundPass;

  private lastAppliedSettings: SystemSettings | null = null;
  private lastAppliedWidth = -1;
  private clearColor = new Color();
  private containerWidth = 1440;

  constructor(
    renderer: WebGLRenderer,
    scene: Scene,
    camera: PerspectiveCamera,
    settings: SystemSettings,
    width: number,
    height: number,
  ) {
    this.renderer = renderer;
    this.scene = scene;
    this.camera = camera;

    const composerTarget = new WebGLRenderTarget(1, 1, {
      type: HalfFloatType,
      depthBuffer: false,
      stencilBuffer: false,
    });
    this.composer = new EffectComposer(this.renderer, composerTarget);

    this.backgroundPass = new BackgroundPass();
    this.backgroundPass.setSize(width, height);
    this.composer.addPass(this.backgroundPass);

    const renderPass = new RenderPass(this.scene, this.camera);
    renderPass.clear = false;
    this.composer.addPass(renderPass);

    this.afterImagePass = new AfterimagePass(settings.trailIntensity);
    this.composer.addPass(this.afterImagePass);

    this.bloomPass = new UnrealBloomPass(
      new Vector2(width, height),
      settings.bloomStrength * screenScale(width),
      0.4,
      settings.bloomThreshold,
    );
    this.composer.addPass(this.bloomPass);

    this.containerWidth = width;
    this.clearColor.set(settings.backgroundColor);
    this.renderer.setClearColor(this.clearColor);
  }

  setSize(width: number, height: number) {
    if (width <= 0 || height <= 0) return;
    this.containerWidth = width;
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height, false);
    this.composer.setSize(width, height);
    this.backgroundPass.setSize(width, height);
  }

  getScreenScale(): number {
    return screenScale(this.containerWidth);
  }

  updateSettings(settings: SystemSettings) {
    if (
      settings === this.lastAppliedSettings &&
      this.containerWidth === this.lastAppliedWidth
    ) {
      return;
    }
    this.lastAppliedSettings = settings;
    this.lastAppliedWidth = this.containerWidth;

    const s = screenScale(this.containerWidth);
    this.clearColor.set(settings.backgroundColor);
    this.renderer.setClearColor(this.clearColor);
    this.bloomPass.strength = settings.bloomStrength * s;
    this.bloomPass.threshold = settings.bloomThreshold;

    if (this.camera.fov !== settings.cameraFov) {
      this.camera.fov = settings.cameraFov;
      this.camera.updateProjectionMatrix();
    }
  }

  render(time: number) {
    this.backgroundPass.setTime(time);
    this.composer.render();
  }

  dispose() {
    this.composer.dispose();
    this.backgroundPass.dispose();
  }
}
