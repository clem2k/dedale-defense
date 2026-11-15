import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { loadEnemies, loadTowers } from "./csv.mjs";

export function validateContent(data) {
  const check = (condition, message) => {
    if (!condition) throw new Error(`game content (game-content.json / towers.csv / ennemies.csv): ${message}`);
  };
  const number = (value, min = 0) => Number.isFinite(value) && value >= min;
  const integer = (value, min = 0) => Number.isInteger(value) && value >= min;
  const id = (value) => typeof value === "string" && /^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$/.test(value);
  const color = (value) => typeof value === "string" && /^#[0-9a-f]{6}$/i.test(value);
  const text = (value) => typeof value === "string" && value.length > 0;
  const unique = (values) => new Set(values).size === values.length;
  const sprite = (s, scale) => s && typeof s.image === "string"
    && /^assets\/(?:[a-z0-9_-]+\/)*[a-z0-9_ -]+\.png$/i.test(s.image)
    && integer(s.row) && integer(s.column) && integer(s.frameSize, 1)
    && number(s[scale], Number.MIN_VALUE);
  check(data && typeof data === "object" && data.version === 1, "unsupported version");
  const b = data.board;
  check(b && number(b.cell, 1) && integer(b.cols, 4) && integer(b.rows, 4), "invalid board dimensions");
  check(number(b.rangeScale, Number.MIN_VALUE), "invalid range scale");
  check(b.enemySpeedScale && ["R", "B"].every((key) => number(b.enemySpeedScale[key], Number.MIN_VALUE)), "invalid enemy speed scale");
  for (const [key, limit] of [["openRows", b.rows], ["openCols", b.cols]]) {
    check(Array.isArray(b[key]) && b[key].length > 0 && unique(b[key]) &&
      b[key].every((n) => integer(n, 1) && n < limit - 1), `invalid board.${key}`);
  }
  const r = data.rules;
  check(r && ["slowDuration", "stunDuration", "killScore"]
    .concat("winScore", "splitJitter").every((key) => number(r[key])), "invalid combat rules");
  check(integer(r.killScore) && integer(r.winScore), "score multipliers must be integers");
  check(data.enemies && Object.keys(data.enemies).length > 0, "missing enemies");
  for (const [key, e] of Object.entries(data.enemies)) {
    check(id(key) && text(e.name) && color(e.color), `invalid enemy ${key}`);
    check(number(e.hp, Number.MIN_VALUE) && number(e.speed, Number.MIN_VALUE) && number(e.r, Number.MIN_VALUE) &&
      integer(e.count) && number(e.gap) && (e.count === 0 || e.gap > 0) &&
      number(e.bounty) && integer(e.lifeLoss, 1), `invalid enemy stats: ${key}`);
    check(typeof e.flying === "boolean" && typeof e.immune === "boolean", `invalid enemy flags: ${key}`);
    check(sprite(e.sprite, "drawScale"), `invalid enemy sprite: ${key}`);
    if (e.split !== null) {
      check(e.split && data.enemies[e.split.type] && integer(e.split.count, 1) &&
        number(e.split.hp, Number.MIN_VALUE) && number(e.split.bounty), `invalid split: ${key}`);
    }
  }
  check(Array.isArray(data.towers) && data.towers.length > 0, "missing towers");
  check(unique(data.towers.map((t) => t.id)) && unique(data.towers.map((t) => t.key)), "duplicate tower id or key");
  for (const t of data.towers) {
    check(sprite(t.sprite, "drawSize"), `invalid tower sprite: ${t.id}`);
    check(id(t.id) && text(t.name) && text(t.desc) && typeof t.key === "string" && /^[1-9]$/.test(t.key) &&
      color(t.color) && color(t.accent), `invalid tower ${t.id}`);
    check(integer(t.cost, 1) &&
      number(t.speed, Number.MIN_VALUE) && integer(t.multi, 1) &&
      typeof t.ground === "boolean" && typeof t.air === "boolean" && (t.ground || t.air), `invalid tower stats: ${t.id}`);
    check(Array.isArray(t.ups) && t.ups.length === 5 && t.ups.every((n) => integer(n, 1)) &&
      Array.isArray(t.dmg) && t.dmg.length === 6 && t.dmg.every((n) => number(n)), `invalid upgrades: ${t.id}`);
    check(["range", "rate", "sell", "factor", "special"].every((key) => Array.isArray(t[key]) && t[key].length === 6),
      `invalid level stats: ${t.id}`);
    check(t.range.every((n) => number(n, 1)) && t.rate.every((n) => number(n, Number.MIN_VALUE)) &&
      t.sell.every((n) => integer(n)) && t.special.every((s) => ["none", "spread", "slow", "stun"].includes(s)) &&
      t.factor.every((n, i) => number(n) && (t.special[i] === "spread" || n <= 1)), `invalid effects: ${t.id}`);
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

export function loadContent(root = new URL("../", import.meta.url)) {
  const data = JSON.parse(readFileSync(new URL("game-content.json", root), "utf8"));
  if ("towers" in data || "enemies" in data)
    throw new Error("game-content.json: towers and enemies must be defined only in towers.csv and ennemies.csv");
  const content = validateContent({ ...data, towers: loadTowers(root), enemies: loadEnemies(root) });
  const images = new Map();
  for (const [definition, directions] of [
    ...content.towers.map((t) => [t, 8]), ...Object.values(content.enemies).map((e) => [e, 1]),
  ]) {
    const s = definition.sprite;
    if (!images.has(s.image)) {
      const png = readFileSync(new URL(s.image.split("/").map(encodeURIComponent).join("/"), root));
      if (png.length < 24 || png.subarray(0, 8).toString("hex") !== "89504e470d0a1a0a"
        || png.toString("ascii", 12, 16) !== "IHDR")
        throw new Error(`CSV sprite: invalid PNG ${s.image}`);
      images.set(s.image, [png.readUInt32BE(16), png.readUInt32BE(20)]);
    }
    const [width, height] = images.get(s.image);
    if ((s.column + directions) * s.frameSize > width || (s.row + 1) * s.frameSize > height)
      throw new Error(`CSV sprite: frame outside ${s.image}: ${definition.id || definition.name}`);
  }
  return content;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url))
  process.stdout.write(JSON.stringify(loadContent()));
