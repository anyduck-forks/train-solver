import type { Preset } from "./types.ts";


const racecar: Preset = {
  name: "Racecar",
  shaderId: "racecar",
  preloadEager: true,
  modelUrl: "/landing/models/head.pts",
  modelSlot: 1,
  glowColor: [0.3, 0.35, 0.55],
  separation: 0,
  controls: [
    { id: "scale", label: "Scale", min: 5, max: 150, initial: 48 },
    { id: "spin", label: "Spin Speed", min: 0, max: 1, initial: 0.23 },
    { id: "shimmer", label: "Shimmer", min: 0, max: 2, initial: 0.6 },
    { id: "rotZ", label: "Rotate Z", min: -180, max: 180, initial: 15 },
  ],
  labels: [],
  labelColor: "#BFC7E2",
  info: {
    title: "",
    description: "",
  },
};

const kupe: Preset = {
  name: "Kupe",
  shaderId: "runner",
  modelUrl: "/landing/models/kupe.pts",
  modelSlot: 2,
  cameraPosition: [-5, 30, 80],
  cameraTarget: [0, 0, 0],
  glowColor: [0.3, 0.35, 0.55],
  separation: 0,
  controls: [
    { id: "scale", label: "Scale", min: 5, max: 150, initial: 58 },
    { id: "spin", label: "Spin Speed", min: 0, max: 1, initial: 0.23 },
    { id: "shimmer", label: "Shimmer", min: 0, max: 2, initial: 0.5 },
    { id: "rotX", label: "Rotate X", min: -180, max: 180, initial: 170 },
    { id: "rotY", label: "Rotate Y", min: -180, max: 180, initial: 0 },
    { id: "rotZ", label: "Rotate Z", min: -180, max: 180, initial: 0 },
  ],
  labels: [],
  labelColor: "#BFC7E2",
  info: {
    title: "",
    description: "",
  },
};

const underTheHood: Preset = {
  name: "Under The Hood",
  shaderId: "racecar",
  modelUrl: "/landing/models/head.pts",
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
    title: "",
    description: "",
  },
};

export const presets: Preset[] = [
  racecar,
  kupe,
  underTheHood,
];
