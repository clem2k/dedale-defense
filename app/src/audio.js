export const TOWER_FX = {
  pellet: "01", dart: "02", shell: "03",
  missile: "04", ice: "05", pulse: "06",
};
export const FX_NAMES = ["01", "02", "03", "04", "05", "06", "HIT"];

export class FxAudio {
  constructor(reportError) {
    this.volume = 0.5;
    this.muted = false;
    this.context = null;
    this.gain = null;
    this.buffers = new Map();
    this.loading = null;
    this.failed = false;
    this.reportError = reportError;
  }

  async unlock() {
    if (this.failed) return;
    try {
      if (!this.context) {
        this.context = new AudioContext();
        this.gain = this.context.createGain();
        this.gain.connect(this.context.destination);
        this.applyVolume();
        this.loading = Promise.all(FX_NAMES.map(async (name) => {
          const response = await fetch(new URL(`./assets/fx/${name}.mp3`, import.meta.url));
          if (!response.ok) throw new Error(`FX ${name}: HTTP ${response.status}`);
          const buffer = await this.context.decodeAudioData(await response.arrayBuffer());
          this.buffers.set(name, buffer);
        }));
      }
      await Promise.all([this.context.resume(), this.loading]);
    } catch (error) {
      if (!this.failed) {
        this.failed = true;
        this.buffers.clear();
        console.error("Impossible de charger ou d'activer les sons FX", error);
        this.reportError("Sons FX indisponibles. Rechargez la page pour réessayer.");
      }
    }
  }

  applyVolume() {
    if (this.gain) this.gain.gain.value = this.muted ? 0 : this.volume;
  }

  setVolume(volume) {
    if (!Number.isFinite(volume) || volume < 0 || volume > 1)
      throw new RangeError("FX volume must be between 0 and 1");
    this.volume = volume;
    this.applyVolume();
  }

  toggleMute() {
    this.muted = !this.muted;
    this.applyVolume();
  }

  play(name) {
    if (!FX_NAMES.includes(name)) throw new Error(`Unknown FX: ${name}`);
    const buffer = this.buffers.get(name);
    if (this.failed || this.muted || this.volume === 0 || !buffer || this.context.state !== "running") return;
    const source = this.context.createBufferSource();
    source.buffer = buffer;
    source.connect(this.gain);
    source.onended = () => source.disconnect();
    source.start();
  }
}
