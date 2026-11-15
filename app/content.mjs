import { readFileSync } from "node:fs";

export const contentPath = new URL("../game-content.json", import.meta.url);

export function validateContent(data) {
  const check = (condition, message) => {
    if (!condition) throw new Error(`game-content.json: ${message}`);
  };
  const number = (value, min = 0) => Number.isFinite(value) && value >= min;
  const integer = (value, min = 0) => Number.isInteger(value) && value >= min;
  const id = (value) => typeof value === "string" && /^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$/.test(value);
  const color = (value) => typeof value === "string" && /^#[0-9a-f]{6}$/i.test(value);
  const text = (value) => typeof value === "string" && value.length > 0;
  const unique = (values) => new Set(values).size === values.length;
  check(data && typeof data === "object" && data.version === 1, "unsupported version");
  const b = data.board;
  check(b && number(b.cell, 1) && integer(b.cols, 4) && integer(b.rows, 4), "invalid board dimensions");
  for (const [key, limit] of [["openRows", b.rows], ["openCols", b.cols]]) {
    check(Array.isArray(b[key]) && b[key].length > 0 && unique(b[key]) &&
      b[key].every((n) => integer(n, 1) && n < limit - 1), `invalid board.${key}`);
  }
  const r = data.rules;
  check(r && ["rangeGrowth", "rateReduction", "sellRefund", "slowDuration", "stunDuration", "killScore"]
    .concat("winScore", "splitJitter").every((key) => number(r[key])), "invalid combat rules");
  check(integer(r.killScore) && integer(r.winScore), "score multipliers must be integers");
  check(r.sellRefund <= 1, "sellRefund must not exceed 1");
  check(data.enemies && Object.keys(data.enemies).length > 0, "missing enemies");
  for (const [key, e] of Object.entries(data.enemies)) {
    check(id(key) && text(e.name) && color(e.color), `invalid enemy ${key}`);
    check(number(e.hp, Number.MIN_VALUE) && number(e.speed, Number.MIN_VALUE) && number(e.r, Number.MIN_VALUE) &&
      integer(e.count) && number(e.gap) && (e.count === 0 || e.gap > 0) &&
      number(e.bounty) && integer(e.lifeLoss, 1), `invalid enemy stats: ${key}`);
    check(typeof e.flying === "boolean" && typeof e.immune === "boolean", `invalid enemy flags: ${key}`);
    check(["circle", "tri", "wing", "square", "diamond", "boss"].includes(e.shape), `invalid enemy shape: ${key}`);
    if (e.split !== null) {
      check(e.split && data.enemies[e.split.type] && integer(e.split.count, 1) &&
        number(e.split.hp, Number.MIN_VALUE) && number(e.split.bounty), `invalid split: ${key}`);
    }
  }
  check(Array.isArray(data.towers) && data.towers.length > 0, "missing towers");
  check(unique(data.towers.map((t) => t.id)) && unique(data.towers.map((t) => t.key)), "duplicate tower id or key");
  for (const t of data.towers) {
    check(id(t.id) && text(t.name) && text(t.desc) && typeof t.key === "string" && /^[1-9]$/.test(t.key) &&
      color(t.color) && color(t.accent), `invalid tower ${t.id}`);
    check(integer(t.cost, 1) && number(t.range, 1) && number(t.rate, Number.MIN_VALUE) &&
      number(t.speed) && number(t.splash) && integer(t.multi, 1) && number(t.stun) && t.stun <= 1 &&
      typeof t.ground === "boolean" && typeof t.air === "boolean" && (t.ground || t.air), `invalid tower stats: ${t.id}`);
    check(Array.isArray(t.ups) && t.ups.length === 4 && t.ups.every((n) => integer(n, 1)) &&
      Array.isArray(t.dmg) && t.dmg.length === 5 && t.dmg.every((n) => number(n)) &&
      Array.isArray(t.slow) && t.slow.length === 5 && t.slow.every((n) => number(n) && n < 1), `invalid upgrades: ${t.id}`);
    check(1 - r.rateReduction * t.ups.length > 0, `nonpositive upgraded firing rate: ${t.id}`);
    check(["pellet", "shell", "dart", "missile", "ice", "pulse"].includes(t.shot) &&
      (t.shot === "pulse" || t.speed > 0), `invalid projectile: ${t.id}`);
  }
  check(data.waveSets && Object.keys(data.waveSets).length > 0, "missing wave sets");
  for (const [key, waves] of Object.entries(data.waveSets)) {
    check(id(key) && Array.isArray(waves) && waves.length > 0, `invalid wave set: ${key}`);
    for (const [i, w] of waves.entries()) {
      check(data.enemies[w.type]?.count > 0 && number(w.hp, Number.MIN_VALUE) &&
        number(w.bountyMultiplier) && Array.isArray(w.targets) && w.targets.length > 0 &&
        w.targets.every((target) => target === "R" || target === "B"), `invalid wave: ${key}[${i}]`);
    }
  }
  check(Array.isArray(data.levels) && data.levels.length > 0 && unique(data.levels.map((l) => l.id)), "invalid levels");
  for (const l of data.levels) {
    check(id(l.id) && text(l.name) && number(l.hp, Number.MIN_VALUE) && integer(l.lives, 1) &&
      integer(l.gold) && number(l.gap, Number.MIN_VALUE) && integer(l.waves, 1) &&
      l.waves <= (data.waveSets[l.waveSet]?.length ?? 0), `invalid level: ${l.id}`);
  }
  check(data.levels.some((l) => l.id === "normal"), "missing default level: normal");
  return data;
}

export function loadContent() {
  return validateContent(JSON.parse(readFileSync(contentPath, "utf8")));
}
