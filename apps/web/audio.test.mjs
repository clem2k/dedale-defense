import assert from "node:assert/strict";
import test from "node:test";
import { FxAudio, FX_NAMES } from "./src/audio.js";

function setup(t, failedResponse = false) {
  const oldContext = globalThis.AudioContext;
  const sources = [];
  let contexts = 0;
  class Context {
    state = "suspended";
    destination = {};
    constructor() { contexts++; }
    createGain() { return { gain: { value: 1 }, connect() {} }; }
    async resume() { this.state = "running"; }
    async decodeAudioData(bytes) { return bytes; }
    createBufferSource() {
      const source = {
        start() { this.started = true; },
        connect(gain) { this.gain = gain; },
        disconnect() { this.disconnected = true; },
      };
      sources.push(source);
      return source;
    }
  }
  globalThis.AudioContext = Context;
  t.after(() => {
    if (oldContext === undefined) delete globalThis.AudioContext;
    else globalThis.AudioContext = oldContext;
  });
  const requests = [];
  t.mock.method(globalThis, "fetch", async (url) => {
    requests.push(url.pathname);
    return { ok: !failedResponse, status: 404, async arrayBuffer() { return new ArrayBuffer(8); } };
  });
  const errors = [];
  const audio = new FxAudio((message) => errors.push(message));
  return { audio, errors, requests, sources, get contexts() { return contexts; } };
}

test("FX loads once on interaction and overlaps salvos through one live gain", async (t) => {
  const fixture = setup(t);
  const { audio, sources, requests } = fixture;
  audio.play("01");
  assert.equal(sources.length, 0);
  await Promise.all([audio.unlock(), audio.unlock()]);
  assert.equal(fixture.contexts, 1);
  assert.equal(requests.length, 7);
  for (const name of FX_NAMES) assert.ok(requests.some((path) => path.endsWith(`/assets/fx/${name}.mp3`)));
  audio.play("01");
  audio.play("01");
  assert.equal(sources.length, 2);
  assert.notEqual(sources[0], sources[1]);
  assert.equal(sources[0].gain, sources[1].gain);
  assert.equal(sources[0].gain.gain.value, 0.5);
  audio.setVolume(0.27);
  assert.equal(sources[0].gain.gain.value, 0.27);
  audio.toggleMute();
  assert.equal(sources[0].gain.gain.value, 0);
  audio.play("HIT");
  assert.equal(sources.length, 2);
  audio.setVolume(0.63);
  assert.equal(audio.gain.gain.value, 0);
  audio.toggleMute();
  assert.equal(audio.gain.gain.value, 0.63);
  audio.play("HIT");
  assert.equal(sources.length, 3);
  audio.setVolume(0);
  audio.play("01");
  assert.equal(sources.length, 3);
  sources[0].onended();
  assert.equal(sources[0].disconnected, true);
  for (const value of [-1, 1.1, NaN]) assert.throws(() => audio.setVolume(value), RangeError);
});

test("failed FX loads report a visible error once without unhandled rejections", async (t) => {
  const { audio, errors } = setup(t, true);
  const log = t.mock.method(console, "error", () => {});
  await Promise.all([audio.unlock(), audio.unlock()]);
  await audio.unlock();
  assert.equal(errors.length, 1);
  assert.equal(log.mock.calls.length, 1);
  assert.equal(audio.failed, true);
  audio.play("01");
  assert.equal(audio.buffers.size, 0);
});
