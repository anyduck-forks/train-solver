import type { Preset } from "./types.ts";

const websiteMockups: Preset = {
  name: "Website Mockups",
  shaderId: "remixLogo",
  modelUrl: "/landing/models/mockup-websites.pts",
  modelSlot: 0,
  glowColor: [0.3, 0.35, 0.55],
  separation: 0,
  controls: [
    { id: "scale", label: "Scale", min: 5, max: 80, initial: 55 },
    { id: "rotX", label: "Rotate X", min: -180, max: 180, initial: 18 },
    { id: "rotY", label: "Rotate Y", min: -180, max: 180, initial: 0 },
    { id: "rotZ", label: "Rotate Z", min: -180, max: 180, initial: -14.4 },
    { id: "spin", label: "Spin Speed", min: 0, max: 1, initial: 0.23 },
  ],
  info: {
    title: "Website Mockups",
    description: "5 website mockups spinning as a particle cloud",
  },
};

const racecar: Preset = {
  name: "Racecar",
  shaderId: "racecar",
  preloadEager: true,
  modelUrl: "/landing/models/racecar.pts",
  modelSlot: 1,
  glowColor: [0.3, 0.35, 0.55],
  separation: 0,
  controls: [
    { id: "scale", label: "Scale", min: 5, max: 150, initial: 48 },
    { id: "spin", label: "Spin Speed", min: 0, max: 1, initial: 0.23 },
    { id: "shimmer", label: "Shimmer", min: 0, max: 2, initial: 0.6 },
    { id: "rotZ", label: "Rotate Z", min: -180, max: 180, initial: 15 },
  ],
  labels: [
    {
      id: "frontend",
      text: "FRONTEND",
      anchor: [0, 0.08, 0.5],
      offset: [-90, -58],
    },
    {
      id: "between",
      text: "EVERYTHING IN BETWEEN",
      anchor: [0.0, 0.04, -0.04],
      offset: [-110, -72],
    },
    {
      id: "backend",
      text: "BACKEND",
      anchor: [0, 0.18, -0.25],
      offset: [-160, -104],
    },
  ],
  labelColor: "#BFC7E2",
  info: {
    title: "Racecar",
    description: "Race car rendered as a particle cloud",
  },
};


const runner: Preset = {
  name: "Model Kit Runner",
  shaderId: "runner",
  modelUrl: "/landing/models/model-kit-runner.pts",
  modelSlot: 2,
  cameraPosition: [-5, 30, 80],
  cameraTarget: [0, 0, 0],
  glowColor: [0.3, 0.35, 0.55],
  separation: 0,
  controls: [
    { id: "scale", label: "Scale", min: 5, max: 150, initial: 58 },
    { id: "spin", label: "Spin Speed", min: 0, max: 1, initial: 0.23 },
    { id: "shimmer", label: "Shimmer", min: 0, max: 2, initial: 0.5 },
    { id: "rotZ", label: "Rotate Z", min: -180, max: 180, initial: 15 },
  ],
  labels: [
    {
      id: "humans",
      text: "EASY FOR HUMANS AND MODELS",
      anchor: [0.28, 0.5, -0.04],
      offset: [-130, -84],
    },
    {
      id: "parts",
      text: "ALL THE PARTS YOU NEED",
      anchor: [0.36, -0.2, -0.06],
      offset: [-110, -72],
    },
  ],
  labelColor: "#BFC7E2",
  info: {
    title: "Model Kit Runner",
    description: "Runner figure rendered as a particle cloud",
  },
};

const underTheHood: Preset = {
  name: "Under The Hood",
  shaderId: "racecar",
  modelUrl: "/landing/models/racecar.pts",
  modelSlot: 1,
  cameraPosition: [0, 12, -55],
  cameraTarget: [0, -2, 0],
  glowColor: [0.3, 0.35, 0.55],
  separation: 0,
  controls: [
    { id: "scale", label: "Scale", min: 5, max: 200, initial: 90 },
    { id: "spin", label: "Spin Speed", min: 0, max: 1, initial: 0 },
    { id: "shimmer", label: "Shimmer", min: 0, max: 2, initial: 0.6 },
    { id: "rotZ", label: "Rotate Z", min: -180, max: 180, initial: 0 },
  ],
  info: {
    title: "Under The Hood",
    description: "Race car viewed from the rear",
  },
};

export const presets: Preset[] = [
  racecar,
  runner,
  underTheHood,
  websiteMockups,
];
