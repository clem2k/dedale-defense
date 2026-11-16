import { readFileSync } from "node:fs";

export function parseCsv(text, file) {
  const rows = [];
  let row = [], value = "", quoted = false, closed = false;
  const fail = (message) => { throw new Error(`${file}: row ${rows.length + 1}: ${message}`); };
  text = text.replace(/^\uFEFF/, "");
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quoted) {
      if (c === '"') {
        if (text[i + 1] === '"') { value += '"'; i++; }
        else { quoted = false; closed = true; }
      } else value += c;
    } else if (c === '"') {
      if (value || closed) fail("unexpected quote");
      quoted = true;
    } else if (c === "," || c === "\n" || c === "\r") {
      row.push(value);
      value = "";
      closed = false;
      if (c !== ",") {
        if (c === "\r" && text[i + 1] === "\n") i++;
        if (row.some((cell) => cell !== "")) rows.push(row);
        row = [];
      }
    } else {
      if (closed) fail("unexpected character after closing quote");
      value += c;
    }
  }
  if (quoted) fail("unterminated quoted field");
  if (value || row.length || closed) { row.push(value); rows.push(row); }
  const headers = rows.shift();
  if (!headers || headers.some((key) => !key) || new Set(headers).size !== headers.length)
    fail("missing or duplicate headers");
  return rows.map((cells, index) => {
    if (cells.length !== headers.length) throw new Error(`${file}: row ${index + 2}: wrong column count`);
    return Object.fromEntries(headers.map((key, i) => [key, cells[i]]));
  });
}

const TOWER_STATS = ["tower", "level", "cost", "damage", "range", "speed", "sell",
  "special", "special factor", "earth", "fly"];
const TOWER_METADATA = ["id", "name", "key", "color", "accent", "desc", "shot",
  "projectile speed", "multi", "sprite file", "sprite row", "sprite column", "sprite size", "draw size"];
const ENEMY_COLUMNS = ["id", "name", "hp", "speed", "count", "gap", "bounty", "color", "r",
  "flying", "immune", "lifeLoss", "split type", "split count", "split hp", "split bounty",
  "desc", "sprite file", "sprite row", "sprite column", "sprite size", "draw scale"];

function readRows(root, file, columns) {
  const rows = parseCsv(readFileSync(new URL(file, root), "utf8"), file);
  if (!rows.length || columns.some((key) => !(key in rows[0]))
    || Object.keys(rows[0]).some((key) => !columns.includes(key)))
    throw new Error(`${file}: missing data or unexpected column headers`);
  return rows;
}

function numeric(row, key, file) {
  const value = row[key];
  if (!/^-?(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?$/i.test(value) || !Number.isFinite(Number(value)))
    throw new Error(`${file}: ${row.id || row.tower}: invalid number in ${key}: ${value}`);
  return Number(value);
}

function flag(row, key, file) {
  if (!["0", "1"].includes(row[key])) throw new Error(`${file}: invalid flag ${key}: ${row[key]}`);
  return row[key] === "1";
}

function sprite(row, file, scaleKey) {
  return {
    image: row["sprite file"],
    row: numeric(row, "sprite row", file) - 1,
    column: numeric(row, "sprite column", file) - 1,
    frameSize: numeric(row, "sprite size", file),
    [scaleKey]: numeric(row, scaleKey === "drawSize" ? "draw size" : "draw scale", file),
  };
}

export function loadTowers(root) {
  const file = "towers.csv";
  const rows = readRows(root, file, [...TOWER_STATS, ...TOWER_METADATA]);
  const groups = new Map();
  for (const row of rows) {
    const index = numeric(row, "tower", file);
    if (!Number.isInteger(index) || index < 1) throw new Error(`${file}: invalid tower number`);
    if (!groups.has(index)) groups.set(index, []);
    groups.get(index).push(row);
  }
  return [...groups].sort(([a], [b]) => a - b).map(([index, levels], position) => {
    levels.sort((a, b) => numeric(a, "level", file) - numeric(b, "level", file));
    if (index !== position + 1 || levels.length !== 6
      || levels.some((row, i) => numeric(row, "level", file) !== i + 1))
      throw new Error(`${file}: tower ${index} must have exactly levels 1 to 6; tower numbers must be consecutive`);
    const first = levels[0];
    if (levels.slice(1).some((row) => TOWER_METADATA.some((key) => row[key] !== "")))
      throw new Error(`${file}: tower ${index}: shared metadata belongs on level 1 only`);
    const ground = flag(first, "earth", file), air = flag(first, "fly", file);
    if (levels.some((row) => flag(row, "earth", file) !== ground || flag(row, "fly", file) !== air))
      throw new Error(`${file}: tower ${index}: target flags must match across levels`);
    const values = (key) => levels.map((row) => numeric(row, key, file));
    return {
      id: first.id, name: first.name, key: first.key, color: first.color, accent: first.accent, desc: first.desc,
      cost: numeric(first, "cost", file), ups: values("cost").slice(1),
      dmg: values("damage"), range: values("range"), rate: values("speed").map((n) => 1 / n),
      sell: values("sell"), special: levels.map((row) => row.special || "none"),
      factor: levels.map((row) => row["special factor"] === "" ? 0 : numeric(row, "special factor", file)),
      ground, air, shot: first.shot, speed: numeric(first, "projectile speed", file),
      multi: numeric(first, "multi", file), sprite: sprite(first, file, "drawSize"),
    };
  });
}

export function loadEnemies(root) {
  const file = "ennemies.csv";
  const enemies = Object.create(null);
  for (const row of readRows(root, file, ENEMY_COLUMNS)) {
    if (Object.hasOwn(enemies, row.id)) throw new Error(`${file}: duplicate enemy ${row.id}`);
    const splitValues = ["split type", "split count", "split hp", "split bounty"].map((key) => row[key]);
    if (splitValues.some(Boolean) && !splitValues.every(Boolean))
      throw new Error(`${file}: incomplete split definition: ${row.id}`);
    enemies[row.id] = {
      name: row.name, color: row.color, desc: row.desc,
      ...Object.fromEntries(["hp", "speed", "count", "gap", "bounty", "r", "lifeLoss"]
        .map((key) => [key, numeric(row, key, file)])),
      flying: flag(row, "flying", file), immune: flag(row, "immune", file),
      split: row["split type"] ? {
        type: row["split type"], count: numeric(row, "split count", file),
        hp: numeric(row, "split hp", file), bounty: numeric(row, "split bounty", file),
      } : null,
      sprite: sprite(row, file, "drawScale"),
    };
  }
  return { ...enemies };
}
