export const MODEL_TEX_W = 512;
export const MODEL_TEX_H = 256;

export const PRESET_GLSL = /* glsl */ `
  vec3 hsl2rgb(float h, float s, float l) {
    float c = (1.0 - abs(2.0 * l - 1.0)) * s;
    float hp = h * 6.0;
    float x = c * (1.0 - abs(mod(hp, 2.0) - 1.0));
    float m = l - c * 0.5;
    vec3 rgb;
    if      (hp < 1.0) rgb = vec3(c, x, 0.0);
    else if (hp < 2.0) rgb = vec3(x, c, 0.0);
    else if (hp < 3.0) rgb = vec3(0.0, c, x);
    else if (hp < 4.0) rgb = vec3(0.0, x, c);
    else if (hp < 5.0) rgb = vec3(x, 0.0, c);
    else               rgb = vec3(c, 0.0, x);
    return rgb + m;
  }

  vec3 sampleModelTex(sampler2D tex, float cnt, float fi) {
    float idx = (cnt > 0.0) ? mod(fi, cnt) : 0.0;
    float u = (mod(idx, ${MODEL_TEX_W}.0) + 0.5) / ${MODEL_TEX_W}.0;
    float v = (floor(idx / ${MODEL_TEX_W}.0) + 0.5) / ${MODEL_TEX_H}.0;
    return texture(tex, vec2(u, v)).xyz;
  }

  void presetTrain(float fi, int cnt, float time,
    float c0, float c1, float c2, float c3,
    float c4, float c5, float c6, float c7,
    out vec3 pos, out vec3 col)
  {
    float scale = c0;
    float spin = c1;
    float shimmer = c2;
    float rotZ = c3 * 0.01745329;
    float wiggleZ = c4;
    float wiggleY = c5;
    float wiggleSpeed = c6;

    float angle = time * spin;
    float cosA = cos(angle), sinA = sin(angle);

    vec3 mp = sampleModelTex(uModelTex0, uModelCount0, fi);
    float mx = mp.x * scale;
    float my = mp.y * scale;
    float mz = mp.z * scale;

    float px = mx * cosA - mz * sinA;
    float py = my;
    float pz = mx * sinA + mz * cosA;

    float cz = cos(rotZ), sz = sin(rotZ);
    float qx = px * cz - py * sz;
    float qy = px * sz + py * cz;

    float wiggleT = time * wiggleSpeed;
    float driftZ = sin(wiggleT + fi * 0.01) * wiggleZ;
    float driftY = sin(wiggleT * 1.7) * wiggleY;

    pos = vec3(qx, qy + driftY, pz + driftZ);

    float height = mp.y * 0.5 + 0.5;
    float pulse = 1.0 + shimmer * 0.1 * sin(time * 2.5 + fi * 0.02);
    float lum = (0.3 + 0.5 * height) * pulse;
    col = hsl2rgb(0.55 + 0.1 * height, 0.6, lum);
  }

  void computePreset(int id, float fi, int cnt, float time,
    float c0, float c1, float c2, float c3,
    float c4, float c5, float c6, float c7,
    out vec3 pos, out vec3 col)
  {
    presetTrain(fi, cnt, time, c0,c1,c2,c3,c4,c5,c6,c7, pos, col);
  }
`;

export const PRESET_UNIFORMS_GLSL = /* glsl */ `
  uniform sampler2D uModelTex0;
  uniform sampler2D uModelTex1;
  uniform sampler2D uModelTex2;
  uniform sampler2D uModelTex3;
  uniform float uModelCount0;
  uniform float uModelCount1;
  uniform float uModelCount2;
  uniform float uModelCount3;
  uniform float uCarLaneOffset;
  uniform float uCarLaneActivity;
  uniform float uCarPosY;
`;
