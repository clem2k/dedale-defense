// Dédale Défense — a mazing tower defense.
// Enemies cross the board from the left to the right and from the top to the bottom.
// Towers block their way, so the player builds a maze; a tower may never close the last path.

const CELL = 20;
const COLS = 28;
const ROWS = 22;
const W = COLS * CELL;
const H = ROWS * CELL;
const OPEN_R = [9, 10, 11, 12]; // rows open on the left and right edges
const OPEN_C = [12, 13, 14, 15]; // columns open on the top and bottom edges
const WAVE_GAP = 24; // seconds between waves

const MODES = {
  easy: { name: "Facile", hp: 0.75, lives: 30, gold: 70, gap: 28, waves: 30 },
  normal: { name: "Normal", hp: 1, lives: 20, gold: 60, gap: 24, waves: 50 },
  hard: { name: "Difficile", hp: 1.35, lives: 10, gold: 50, gap: 20, waves: 60 },
  uncle_scrooge: { name: "Oncle Picsou", hp: 1.3, lives: 20, gold: 10000, gap: 24, waves: 40 },
  rush: { name: "Ruée", hp: 1.1, lives: 15, gold: 60, gap: 10, waves: 20, desc: "Les vagues s'enchaînent rapidement." },
  marathon: { name: "Marathon", hp: 0.85, lives: 30, gold: 80, gap: 30, waves: 70, hpGrowth: 1.06, desc: "Longue partie, avec une progression des ennemis plus lente." },
};

// ---------- Towers ----------
const TOWERS = [
  {
    id: "stone", name: "Caillou", key: "1", color: "#9aa4b2", accent: "#e4e7ec",
    desc: "Tir rapide et bon marché. Touche le sol et les airs.",
    cost: 5, ups: [5, 10, 20, 40], dmg: [5, 10, 20, 40, 80], range: 70, rate: 0.5,
    ground: true, air: true, shot: "pellet", speed: 340,
  },
  {
    id: "burst", name: "Éclat", key: "2", color: "#4ea1ff", accent: "#b9dcff",
    desc: "Obus qui explose et blesse les ennemis proches. Sol uniquement.",
    cost: 15, ups: [15, 30, 60, 120], dmg: [8, 16, 32, 64, 128], range: 62, rate: 0.9,
    ground: true, air: false, shot: "shell", speed: 240, splash: 30,
  },
  {
    id: "needle", name: "Aiguille", key: "3", color: "#c084fc", accent: "#ead5ff",
    desc: "Longue portée et gros dégâts, mais lente. Sol et airs.",
    cost: 25, ups: [25, 50, 100, 200], dmg: [25, 50, 100, 200, 400], range: 130, rate: 1.4,
    ground: true, air: true, shot: "dart", speed: 520,
  },
  {
    id: "volley", name: "Volée", key: "4", color: "#f97066", accent: "#ffd1cc",
    desc: "Lance jusqu'à quatre missiles à la fois. Airs uniquement.",
    cost: 30, ups: [30, 60, 120, 240], dmg: [12, 24, 48, 96, 192], range: 105, rate: 0.85,
    ground: false, air: true, shot: "missile", speed: 300, multi: 4,
  },
  {
    id: "frost", name: "Givre", key: "5", color: "#5fe0e0", accent: "#d6fbfb",
    desc: "Ralentit sa cible pendant deux secondes. Sol et airs.",
    cost: 20, ups: [20, 40, 80, 160], dmg: [2, 4, 8, 16, 32], range: 72, rate: 0.6,
    ground: true, air: true, shot: "ice", speed: 320, slow: [0.35, 0.4, 0.45, 0.5, 0.6],
  },
  {
    id: "pulse", name: "Onde", key: "6", color: "#fdb022", accent: "#fff0c2",
    desc: "Onde de choc autour d'elle, peut étourdir. Sol uniquement.",
    cost: 40, ups: [40, 80, 160, 320], dmg: [20, 40, 80, 160, 320], range: 46, rate: 1.2,
    ground: true, air: false, shot: "pulse", stun: 0.15,
  },
];

// ---------- Enemies ----------
const CREEPS = {
  walker: { name: "Marcheurs", hp: 1, speed: 40, count: 10, gap: 0.65, bounty: 1, color: "#a6ef67", r: 6, shape: "circle", desc: "Ennemis ordinaires." },
  runner: { name: "Coureurs", hp: 0.7, speed: 78, count: 10, gap: 0.55, bounty: 1, color: "#fef08a", r: 5, shape: "tri", desc: "Très rapides." },
  swarm: { name: "Essaim", hp: 0.4, speed: 46, count: 22, gap: 0.3, bounty: 0.5, color: "#fda29b", r: 4, shape: "circle", desc: "Nombreux mais fragiles." },
  flyer: { name: "Volants", hp: 0.85, speed: 46, count: 8, gap: 0.9, bounty: 1.5, color: "#7cd4fd", r: 6, shape: "wing", flying: true, desc: "Survolent les tours." },
  armored: { name: "Blindés", hp: 1.5, speed: 34, count: 8, gap: 0.9, bounty: 1.5, color: "#bdb4fe", r: 7, shape: "square", immune: true, desc: "Insensibles au givre et au choc." },
  splitter: { name: "Scindeurs", hp: 1.05, speed: 40, count: 8, gap: 0.85, bounty: 1, color: "#f9a8d4", r: 7, shape: "diamond", splits: true, desc: "Se divisent en deux en mourant." },
  mini: { name: "Éclats", hp: 0.3, speed: 52, count: 0, gap: 0, bounty: 0.5, color: "#f9a8d4", r: 4, shape: "diamond" },
  boss: { name: "Colosse", hp: 14, speed: 28, count: 1, gap: 1, bounty: 25, color: "#f04438", r: 11, shape: "boss", desc: "Un seul, mais énorme." },
};
const CYCLE = ["walker", "runner", "walker", "flyer", "swarm", "armored", "walker", "splitter", "runner", "boss"];

function waveDef(n, modeKey = mode) {
  // n is 1-based
  const type = n === 1 ? "walker" : CYCLE[(n - 1) % CYCLE.length];
  const base = 12 * Math.pow(MODES[modeKey].hpGrowth || 1.115, n - 1) + 4 * n;
  return { n, type, hp: base, count: CREEPS[type].count, bounty: CREEPS[type].bounty * (2 + Math.floor(n / 6)) };
}

// ---------- Game state ----------
let S; // current game
let mode = "normal";
let buildType = -1;
let selected = null;
let hover = null; // {cx, cy, x, y}
let hoverTowerBtn = -1;
let shiftHeld = false;
let speed = 1;

const wall = new Uint8Array(COLS * ROWS);
for (let y = 0; y < ROWS; y++)
  for (let x = 0; x < COLS; x++) {
    const edge = x === 0 || y === 0 || x === COLS - 1 || y === ROWS - 1;
    const openLR = (x === 0 || x === COLS - 1) && OPEN_R.includes(y);
    const openTB = (y === 0 || y === ROWS - 1) && OPEN_C.includes(x);
    if (edge && !openLR && !openTB) wall[y * COLS + x] = 1;
  }
const EXIT_R = OPEN_R.map((y) => y * COLS + COLS - 1);
const EXIT_B = OPEN_C.map((x) => (ROWS - 1) * COLS + x);
const ENTRY_L = OPEN_R.map((y) => y * COLS);
const ENTRY_T = OPEN_C.map((x) => x);

function newGame(m) {
  mode = m;
  const M = MODES[m];
  S = {
    mode: m,
    lives: M.lives,
    gold: M.gold,
    score: 0,
    kills: 0,
    wave: 0, // waves sent
    started: false,
    paused: false,
    over: false,
    won: false,
    timer: 0,
    towers: [],
    towerAt: new Array(COLS * ROWS).fill(null),
    creeps: [],
    spawners: [],
    shots: [],
    fx: [],
    floats: [],
    fieldR: null,
    fieldB: null,
    path: [],
    nextId: 1,
    time: 0,
  };
  buildType = -1;
  selected = null;
  refreshFields();
  ui.showStart(false);
  ui.update(true);
}

// ---------- Path finding (8 directions, no corner cutting) ----------
const NB = [
  [1, 0, 10], [-1, 0, 10], [0, 1, 10], [0, -1, 10],
  [1, 1, 14], [1, -1, 14], [-1, 1, 14], [-1, -1, 14],
];

function computeField(targets, blocked) {
  const dist = new Float32Array(COLS * ROWS).fill(Infinity);
  const heap = [];
  const push = (d, i) => {
    heap.push([d, i]);
    let k = heap.length - 1;
    while (k > 0) {
      const p = (k - 1) >> 1;
      if (heap[p][0] <= heap[k][0]) break;
      [heap[p], heap[k]] = [heap[k], heap[p]];
      k = p;
    }
  };
  const pop = () => {
    const top = heap[0];
    const last = heap.pop();
    if (heap.length) {
      heap[0] = last;
      let k = 0;
      for (;;) {
        const l = 2 * k + 1, r = l + 1;
        let m = k;
        if (l < heap.length && heap[l][0] < heap[m][0]) m = l;
        if (r < heap.length && heap[r][0] < heap[m][0]) m = r;
        if (m === k) break;
        [heap[m], heap[k]] = [heap[k], heap[m]];
        k = m;
      }
    }
    return top;
  };
  for (const t of targets) { dist[t] = 0; push(0, t); }
  while (heap.length) {
    const [d, i] = pop();
    if (d > dist[i]) continue;
    const x = i % COLS, y = (i / COLS) | 0;
    for (const [dx, dy, c] of NB) {
      const nx = x + dx, ny = y + dy;
      if (!free(nx, ny, blocked)) continue;
      if (dx && dy && (!free(x + dx, y, blocked) || !free(x, y + dy, blocked))) continue;
      const ni = ny * COLS + nx;
      const nd = d + c;
      if (nd < dist[ni]) { dist[ni] = nd; push(nd, ni); }
    }
  }
  return dist;
}

function free(x, y, blocked) {
  if (x < 0 || y < 0 || x >= COLS || y >= ROWS) return false;
  const i = y * COLS + x;
  if (wall[i] || S.towerAt[i]) return false;
  if (blocked && blocked.has(i)) return false;
  return true;
}

function nextStep(field, x, y) {
  let best = -1, bestCost = Infinity;
  for (const [dx, dy, c] of NB) {
    const nx = x + dx, ny = y + dy;
    if (!free(nx, ny)) continue;
    if (dx && dy && (!free(x + dx, y) || !free(x, y + dy))) continue;
    const ni = ny * COLS + nx;
    const cost = field[ni] + c;
    if (cost < bestCost) { bestCost = cost; best = ni; }
  }
  return best;
}

function refreshFields() {
  S.fieldR = computeField(EXIT_R);
  S.fieldB = computeField(EXIT_B);
  // Preview of the two routes, from the middle of each entrance.
  S.path = [];
  for (const [field, start] of [[S.fieldR, ENTRY_L[1]], [S.fieldB, ENTRY_T[1]]]) {
    const pts = [];
    let i = start, guard = 0;
    while (i >= 0 && field[i] > 0 && guard++ < 800) {
      pts.push(i);
      i = nextStep(field, i % COLS, (i / COLS) | 0);
    }
    if (i >= 0) pts.push(i);
    S.path.push(pts);
  }
}

// ---------- Placement ----------
function cellsOf(cx, cy) {
  return [cy * COLS + cx, cy * COLS + cx + 1, (cy + 1) * COLS + cx, (cy + 1) * COLS + cx + 1];
}

function placeCheck(cx, cy) {
  if (cx < 1 || cy < 1 || cx + 1 > COLS - 2 || cy + 1 > ROWS - 2) return "out";
  const cells = cellsOf(cx, cy);
  for (const c of cells) if (S.towerAt[c] || wall[c]) return "taken";
  const set = new Set(cells);
  for (const k of S.creeps) {
    if (k.flying) continue;
    if (set.has(k.cell) || set.has(k.wcell)) return "creep";
  }
  const fr = computeField(EXIT_R, set);
  const fb = computeField(EXIT_B, set);
  if (!ENTRY_L.some((i) => fr[i] < Infinity) || !ENTRY_T.some((i) => fb[i] < Infinity)) return "block";
  if (ENTRY_L.some((i) => fr[i] === Infinity) || ENTRY_T.some((i) => fb[i] === Infinity)) return "block";
  for (const k of S.creeps) {
    if (k.flying || k.wcell < 0) continue;
    const f = k.target === "R" ? fr : fb;
    if (f[k.wcell] === Infinity) return "block";
  }
  return "ok";
}

function tryPlace(cx, cy) {
  if (S.over) return;
  const def = TOWERS[buildType];
  if (S.gold < def.cost) { flash("Pas assez d'or", cx, cy); return; }
  const r = placeCheck(cx, cy);
  if (r !== "ok") {
    const msg = { block: "Ne bloquez pas le passage !", creep: "Un ennemi est là", taken: "Place occupée", out: "Hors du plateau" }[r];
    flash(msg, cx, cy);
    return;
  }
  const t = {
    id: S.nextId++, type: buildType, cx, cy, level: 0,
    x: (cx + 1) * CELL, y: (cy + 1) * CELL,
    cool: 0, angle: -Math.PI / 2, spent: def.cost, builtWave: S.wave, builtStarted: S.started,
  };
  for (const c of cellsOf(cx, cy)) S.towerAt[c] = t;
  S.towers.push(t);
  S.gold -= def.cost;
  refreshFields();
  if (!shiftHeld) setBuild(-1);
  ui.update();
}

function flash(text, cx, cy) {
  S.floats.push({ x: (cx + 1) * CELL, y: (cy + 1) * CELL - 6, text, color: "#fda29b", life: 1.2, max: 1.2 });
}

function towerStats(t) {
  const d = TOWERS[t.type];
  const L = t.level;
  return {
    dmg: d.dmg[L],
    range: d.range * (1 + 0.08 * L),
    rate: d.rate * (1 - 0.04 * L),
    slow: d.slow ? d.slow[L] : 0,
  };
}

function sellValue(t) {
  // Full refund if it was placed before the first wave, otherwise 75 %.
  if (!t.builtStarted) return t.spent;
  return Math.floor(t.spent * 0.75);
}

function upgrade(t) {
  if (!t || S.over) return;
  const d = TOWERS[t.type];
  if (t.level >= d.ups.length) return;
  const cost = d.ups[t.level];
  if (S.gold < cost) { flash("Pas assez d'or", t.cx, t.cy); return; }
  S.gold -= cost;
  t.spent += cost;
  t.level++;
  S.fx.push({ kind: "ring", x: t.x, y: t.y, r: 4, max: 22, color: d.accent, life: 0.4, maxLife: 0.4 });
  ui.update();
}

function sell(t) {
  if (!t || S.over) return;
  const v = sellValue(t);
  S.gold += v;
  for (const c of cellsOf(t.cx, t.cy)) S.towerAt[c] = null;
  S.towers = S.towers.filter((o) => o !== t);
  S.floats.push({ x: t.x, y: t.y, text: `+${v}`, color: "#fac515", life: 1, max: 1 });
  if (selected === t) selected = null;
  refreshFields();
  ui.update();
}

// ---------- Waves ----------
function sendWave() {
  const totalWaves = MODES[S.mode].waves;
  if (S.over || S.wave >= totalWaves) return;
  if (S.started && S.timer > 0) {
    const bonus = Math.ceil(S.timer) * (S.wave + 1);
    S.score += bonus;
    S.floats.push({ x: W / 2, y: 24, text: `Envoi anticipé +${bonus} pts`, color: "#fac515", life: 1.6, max: 1.6 });
  }
  S.started = true;
  S.wave++;
  const def = waveDef(S.wave);
  S.spawners.push({ def, left: def.count, t: 0, k: 0 });
  S.timer = MODES[S.mode].gap;
  ui.update(true);
}

function spawn(def, k) {
  const c = CREEPS[def.type];
  let target;
  if (def.n >= 15) target = k % 2 ? "B" : "R";
  else target = def.n % 2 ? "R" : "B";
  makeCreep(def.type, def.hp * c.hp * MODES[S.mode].hp, def.bounty, target, null);
}

function makeCreep(type, hp, bounty, target, from) {
  const c = CREEPS[type];
  const lane = (Math.random() * 4) | 0;
  let x, y, wcell;
  if (from) {
    x = from.x + (Math.random() - 0.5) * 6; y = from.y + (Math.random() - 0.5) * 6;
    wcell = from.wcell;
  } else if (target === "R") {
    x = -CELL * 0.6; y = (OPEN_R[lane] + 0.5) * CELL; wcell = ENTRY_L[lane];
  } else {
    x = (OPEN_C[lane] + 0.5) * CELL; y = -CELL * 0.6; wcell = ENTRY_T[lane];
  }
  const k = {
    id: S.nextId++, type, x, y, hp, maxHp: hp, bounty, target,
    speed: c.speed, flying: !!c.flying, r: c.r,
    slowUntil: 0, slowF: 0, stunUntil: 0,
    cell: -1, wcell, exiting: false, dead: false,
    wob: Math.random() * Math.PI * 2,
  };
  if (k.flying) {
    k.wcell = -1;
    k.wx = target === "R" ? W + CELL : x;
    k.wy = target === "R" ? y : H + CELL;
  } else if (from && from.exiting) {
    k.exiting = true; k.wx = from.wx; k.wy = from.wy; k.wcell = -1;
  } else {
    k.wx = (wcell % COLS + 0.5) * CELL;
    k.wy = (((wcell / COLS) | 0) + 0.5) * CELL;
  }
  S.creeps.push(k);
  return k;
}

function remaining(k) {
  if (k.flying) return k.target === "R" ? W - k.x : H - k.y;
  if (k.exiting) return 0;
  const f = k.target === "R" ? S.fieldR : S.fieldB;
  return (f[k.wcell] / 10) * CELL + Math.hypot(k.wx - k.x, k.wy - k.y);
}

function moveCreep(k, dt) {
  let sp = k.speed;
  if (S.time < k.stunUntil) return;
  if (S.time < k.slowUntil) sp *= 1 - k.slowF;
  let step = sp * dt;
  while (step > 0) {
    const dx = k.wx - k.x, dy = k.wy - k.y;
    const d = Math.hypot(dx, dy);
    if (d > step) {
      k.x += (dx / d) * step; k.y += (dy / d) * step;
      break;
    }
    k.x = k.wx; k.y = k.wy; step -= d;
    if (k.flying || k.exiting) { leak(k); return; }
    // Arrived at the centre of a cell: choose the next one.
    k.cell = k.wcell;
    const f = k.target === "R" ? S.fieldR : S.fieldB;
    if (f[k.cell] === 0) {
      k.exiting = true; k.wcell = -1;
      if (k.target === "R") { k.wx = W + CELL; } else { k.wy = H + CELL; }
      continue;
    }
    const n = nextStep(f, k.cell % COLS, (k.cell / COLS) | 0);
    if (n < 0) break; // should never happen: placement keeps a path open
    k.wcell = n;
    k.wx = (n % COLS + 0.5) * CELL;
    k.wy = (((n / COLS) | 0) + 0.5) * CELL;
  }
}

function leak(k) {
  k.dead = true;
  const loss = k.type === "boss" ? 2 : 1;
  S.lives = Math.max(0, S.lives - loss);
  S.floats.push({ x: Math.min(W - 20, Math.max(20, k.x)), y: Math.min(H - 10, Math.max(14, k.y)), text: `-${loss} vie`, color: "#eb3751", life: 1.2, max: 1.2 });
  if (S.lives <= 0) endGame(false);
}

function hurt(k, dmg) {
  if (k.dead) return;
  k.hp -= dmg;
  if (k.hp <= 0) {
    k.dead = true;
    const g = Math.max(1, Math.round(k.bounty));
    S.gold += g;
    S.score += g * 10;
    S.kills++;
    S.floats.push({ x: k.x, y: k.y - 6, text: `+${g}`, color: "#fac515", life: 0.8, max: 0.8 });
    S.fx.push({ kind: "pop", x: k.x, y: k.y, color: CREEPS[k.type].color, life: 0.35, maxLife: 0.35, r: k.r });
    if (CREEPS[k.type].splits) {
      for (let i = 0; i < 2; i++) makeCreep("mini", k.maxHp * 0.3, k.bounty * 0.5, k.target, k);
    }
  }
}

// ---------- Towers firing ----------
function canHit(d, k) {
  return k.flying ? d.air : d.ground;
}

function fire(t) {
  const d = TOWERS[t.type];
  const st = towerStats(t);
  const inRange = S.creeps.filter((k) => !k.dead && canHit(d, k) && Math.hypot(k.x - t.x, k.y - t.y) <= st.range + k.r);
  if (!inRange.length) return false;
  if (d.shot === "pulse") {
    S.fx.push({ kind: "ring", x: t.x, y: t.y, r: 8, max: st.range, color: d.color, life: 0.35, maxLife: 0.35 });
    for (const k of inRange) {
      hurt(k, st.dmg);
      if (!CREEPS[k.type].immune && Math.random() < d.stun) k.stunUntil = S.time + 0.6;
    }
    return true;
  }
  inRange.sort((a, b) => remaining(a) - remaining(b));
  const targets = d.multi ? inRange.slice(0, d.multi) : [inRange[0]];
  t.angle = Math.atan2(targets[0].y - t.y, targets[0].x - t.x);
  for (const k of targets) {
    S.shots.push({
      kind: d.shot, x: t.x + Math.cos(t.angle) * 8, y: t.y + Math.sin(t.angle) * 8,
      target: k, tx: k.x, ty: k.y, speed: d.speed, dmg: st.dmg, splash: d.splash || 0,
      slow: st.slow, color: d.accent, wob: Math.random() * 6,
    });
  }
  return true;
}

function updateShots(dt) {
  for (const s of S.shots) {
    if (s.target && !s.target.dead) { s.tx = s.target.x; s.ty = s.target.y; }
    const dx = s.tx - s.x, dy = s.ty - s.y;
    const d = Math.hypot(dx, dy);
    const step = s.speed * dt;
    if (d <= step + 2) {
      s.done = true;
      if (s.splash) {
        S.fx.push({ kind: "ring", x: s.tx, y: s.ty, r: 4, max: s.splash, color: "#4ea1ff", life: 0.3, maxLife: 0.3 });
        for (const k of S.creeps) if (!k.dead && !k.flying && Math.hypot(k.x - s.tx, k.y - s.ty) <= s.splash + k.r) hurt(k, s.dmg);
      } else if (s.target && !s.target.dead) {
        const k = s.target;
        if (s.slow && !CREEPS[k.type].immune) {
          k.slowF = Math.max(s.slow, S.time < k.slowUntil ? k.slowF : 0);
          k.slowUntil = S.time + 2;
        }
        hurt(k, s.dmg);
      }
      continue;
    }
    s.x += (dx / d) * step; s.y += (dy / d) * step;
  }
  S.shots = S.shots.filter((s) => !s.done);
}

// ---------- Main update ----------
function update(dt) {
  if (!S || S.over || S.paused || ui.overlayOpen) return;
  S.time += dt;

  if (S.started && S.wave < MODES[S.mode].waves) {
    S.timer -= dt;
    if (S.timer <= 0) { S.timer = 0; sendWave(); }
  }

  for (const sp of S.spawners) {
    sp.t -= dt;
    while (sp.left > 0 && sp.t <= 0) {
      spawn(sp.def, sp.k++);
      sp.left--;
      sp.t += CREEPS[sp.def.type].gap;
    }
  }
  S.spawners = S.spawners.filter((sp) => sp.left > 0);

  for (const k of S.creeps) if (!k.dead) moveCreep(k, dt);
  if (S.over) return;

  for (const t of S.towers) {
    t.cool -= dt;
    if (t.cool <= 0) {
      if (fire(t)) t.cool = towerStats(t).rate;
      else t.cool = 0;
    }
  }
  updateShots(dt);
  S.creeps = S.creeps.filter((k) => !k.dead);

  for (const f of S.fx) f.life -= dt;
  S.fx = S.fx.filter((f) => f.life > 0);
  for (const f of S.floats) { f.life -= dt; f.y -= 18 * dt; }
  S.floats = S.floats.filter((f) => f.life > 0);

  if (S.wave >= MODES[S.mode].waves && !S.spawners.length && !S.creeps.length) endGame(true);
}

function endGame(won) {
  if (S.over) return;
  S.over = true;
  S.won = won;
  if (won) S.score += S.lives * 100;
  const key = `dd-best-${S.mode}`;
  const best = +(localStorage.getItem(key) || 0);
  S.best = Math.max(best, S.score);
  S.newBest = S.score > best;
  try { localStorage.setItem(key, String(S.best)); } catch (e) { /* storage may be off */ }
  ui.showEnd();
  ui.update(true);
}

// ---------- Drawing ----------
const canvas = document.getElementById("board");
const ctx = canvas.getContext("2d");
let pixelScale = 1;

function setupCanvas(scale) {
  const dpr = (window.devicePixelRatio || 1) * scale;
  pixelScale = Math.max(1, Math.min(4, dpr));
  canvas.width = Math.round(W * pixelScale);
  canvas.height = Math.round(H * pixelScale);
  drawBackground();
}

let bgCanvas = null;
function drawBackground() {
  bgCanvas = document.createElement("canvas");
  bgCanvas.width = canvas.width;
  bgCanvas.height = canvas.height;
  const g = bgCanvas.getContext("2d");
  g.setTransform(pixelScale, 0, 0, pixelScale, 0, 0);
  const grad = g.createLinearGradient(0, 0, W, H);
  grad.addColorStop(0, "#16243a");
  grad.addColorStop(1, "#122033");
  g.fillStyle = grad;
  g.fillRect(0, 0, W, H);
  // grid
  for (let x = 0; x <= COLS; x++) {
    g.strokeStyle = x % 2 ? "rgba(120,160,220,0.06)" : "rgba(120,160,220,0.13)";
    g.beginPath(); g.moveTo(x * CELL + 0.5, 0); g.lineTo(x * CELL + 0.5, H); g.stroke();
  }
  for (let y = 0; y <= ROWS; y++) {
    g.strokeStyle = y % 2 ? "rgba(120,160,220,0.06)" : "rgba(120,160,220,0.13)";
    g.beginPath(); g.moveTo(0, y * CELL + 0.5); g.lineTo(W, y * CELL + 0.5); g.stroke();
  }
  // walls
  for (let i = 0; i < wall.length; i++) {
    if (!wall[i]) continue;
    const x = (i % COLS) * CELL, y = ((i / COLS) | 0) * CELL;
    g.fillStyle = "#0b1422";
    g.fillRect(x, y, CELL, CELL);
    g.strokeStyle = "rgba(255,255,255,0.05)";
    g.beginPath();
    for (let o = -CELL; o < CELL; o += 6) { g.moveTo(x + o, y + CELL); g.lineTo(x + o + CELL, y); }
    g.save(); g.beginPath(); g.rect(x, y, CELL, CELL); g.clip();
    g.beginPath();
    for (let o = -CELL; o < CELL; o += 6) { g.moveTo(x + o, y + CELL); g.lineTo(x + o + CELL, y); }
    g.stroke(); g.restore();
  }
  // openings
  const arrow = (x, y, dir, color) => {
    g.save(); g.translate(x, y); g.rotate(dir);
    g.fillStyle = color;
    g.beginPath(); g.moveTo(7, 0); g.lineTo(-5, -7); g.lineTo(-2, 0); g.lineTo(-5, 7); g.closePath(); g.fill();
    g.restore();
  };
  g.fillStyle = "rgba(60,203,127,0.16)";
  g.fillRect(0, OPEN_R[0] * CELL, CELL, OPEN_R.length * CELL);
  g.fillRect(OPEN_C[0] * CELL, 0, OPEN_C.length * CELL, CELL);
  g.fillStyle = "rgba(235,55,81,0.16)";
  g.fillRect(W - CELL, OPEN_R[0] * CELL, CELL, OPEN_R.length * CELL);
  g.fillRect(OPEN_C[0] * CELL, H - CELL, OPEN_C.length * CELL, CELL);
  arrow(CELL / 2, (OPEN_R[0] + 2) * CELL, 0, "rgba(60,203,127,0.7)");
  arrow((OPEN_C[0] + 2) * CELL, CELL / 2, Math.PI / 2, "rgba(60,203,127,0.7)");
  arrow(W - CELL / 2, (OPEN_R[0] + 2) * CELL, 0, "rgba(235,55,81,0.7)");
  arrow((OPEN_C[0] + 2) * CELL, H - CELL / 2, Math.PI / 2, "rgba(235,55,81,0.7)");
}

function drawTowerShape(g, type, level, x, y, angle, size = 1) {
  const d = TOWERS[type];
  g.save();
  g.translate(x, y);
  g.scale(size, size);
  // base
  g.fillStyle = "#26354d";
  roundRect(g, -18, -18, 36, 36, 6); g.fill();
  g.strokeStyle = "rgba(255,255,255,0.12)"; g.lineWidth = 1; g.stroke();
  g.fillStyle = "#1a2638";
  roundRect(g, -14, -14, 28, 28, 5); g.fill();
  // body
  g.fillStyle = d.color;
  switch (d.id) {
    case "stone":
      g.beginPath(); g.arc(0, 0, 9, 0, Math.PI * 2); g.fill();
      break;
    case "burst":
      g.beginPath();
      for (let i = 0; i < 6; i++) { const a = (i / 6) * Math.PI * 2; g.lineTo(Math.cos(a) * 11, Math.sin(a) * 11); }
      g.closePath(); g.fill();
      break;
    case "needle":
      g.beginPath(); g.moveTo(0, -12); g.lineTo(11, 0); g.lineTo(0, 12); g.lineTo(-11, 0); g.closePath(); g.fill();
      break;
    case "volley":
      roundRect(g, -10, -10, 20, 20, 4); g.fill();
      g.fillStyle = "#1a2638";
      for (const [a, b] of [[-5, -5], [5, -5], [-5, 5], [5, 5]]) { g.beginPath(); g.arc(a, b, 2.6, 0, Math.PI * 2); g.fill(); }
      break;
    case "frost":
      g.strokeStyle = d.color; g.lineWidth = 3; g.lineCap = "round";
      for (let i = 0; i < 3; i++) {
        const a = (i / 3) * Math.PI;
        g.beginPath(); g.moveTo(Math.cos(a) * 11, Math.sin(a) * 11); g.lineTo(-Math.cos(a) * 11, -Math.sin(a) * 11); g.stroke();
      }
      g.beginPath(); g.arc(0, 0, 4, 0, Math.PI * 2); g.fill();
      break;
    case "pulse":
      g.beginPath(); g.arc(0, 0, 10, 0, Math.PI * 2); g.fill();
      g.strokeStyle = "#1a2638"; g.lineWidth = 2;
      g.beginPath(); g.arc(0, 0, 6, 0, Math.PI * 2); g.stroke();
      g.beginPath(); g.arc(0, 0, 2, 0, Math.PI * 2); g.stroke();
      break;
  }
  // barrel
  if (d.id !== "pulse" && d.id !== "frost") {
    g.rotate(angle);
    g.fillStyle = d.accent;
    g.fillRect(2, -2, d.id === "needle" ? 14 : 11, 4);
    g.rotate(-angle);
  }
  // level pips
  for (let i = 0; i < level; i++) {
    g.fillStyle = "#fac515";
    g.beginPath(); g.arc(-12 + i * 6, 14.5, 1.8, 0, Math.PI * 2); g.fill();
  }
  g.restore();
}

function roundRect(g, x, y, w, h, r) {
  g.beginPath();
  g.moveTo(x + r, y);
  g.arcTo(x + w, y, x + w, y + h, r);
  g.arcTo(x + w, y + h, x, y + h, r);
  g.arcTo(x, y + h, x, y, r);
  g.arcTo(x, y, x + w, y, r);
  g.closePath();
}

function drawCreep(k) {
  const c = CREEPS[k.type];
  const t = S.time;
  ctx.save();
  ctx.translate(k.x, k.y + (k.flying ? Math.sin(t * 6 + k.wob) * 1.5 : 0));
  if (k.flying) {
    ctx.fillStyle = "rgba(0,0,0,0.25)";
    ctx.beginPath(); ctx.ellipse(3, 7, k.r, k.r * 0.45, 0, 0, Math.PI * 2); ctx.fill();
  }
  const slowed = t < k.slowUntil;
  ctx.fillStyle = c.color;
  ctx.strokeStyle = slowed ? "#5fe0e0" : "rgba(0,0,0,0.5)";
  ctx.lineWidth = slowed ? 2 : 1;
  const r = k.r;
  const dir = k.flying ? (k.target === "R" ? 0 : Math.PI / 2) : Math.atan2(k.wy - k.y, k.wx - k.x);
  ctx.beginPath();
  switch (c.shape) {
    case "tri":
      ctx.rotate(dir);
      ctx.moveTo(r + 2, 0); ctx.lineTo(-r, -r); ctx.lineTo(-r, r); ctx.closePath();
      break;
    case "square":
      ctx.rect(-r, -r, r * 2, r * 2);
      break;
    case "diamond":
      ctx.moveTo(0, -r - 1); ctx.lineTo(r + 1, 0); ctx.lineTo(0, r + 1); ctx.lineTo(-r - 1, 0); ctx.closePath();
      break;
    case "wing": {
      ctx.rotate(dir);
      const flap = Math.sin(t * 14 + k.wob) * 3;
      ctx.moveTo(r + 2, 0); ctx.lineTo(-r, -r - flap); ctx.lineTo(-r * 0.4, 0); ctx.lineTo(-r, r + flap); ctx.closePath();
      break;
    }
    case "boss":
      for (let i = 0; i < 10; i++) {
        const a = (i / 10) * Math.PI * 2 + t;
        const rr = i % 2 ? r * 0.75 : r;
        ctx.lineTo(Math.cos(a) * rr, Math.sin(a) * rr);
      }
      ctx.closePath();
      break;
    default:
      ctx.arc(0, 0, r, 0, Math.PI * 2);
  }
  ctx.fill();
  ctx.stroke();
  ctx.restore();
  if (t < k.stunUntil) {
    ctx.fillStyle = "#fff0c2";
    ctx.font = "bold 9px system-ui";
    ctx.fillText("✶", k.x - 3, k.y - r - 5);
  }
  if (k.hp < k.maxHp) {
    const w = Math.max(12, r * 2 + 2);
    ctx.fillStyle = "rgba(0,0,0,0.6)";
    ctx.fillRect(k.x - w / 2, k.y - r - 6, w, 3);
    const p = Math.max(0, k.hp / k.maxHp);
    ctx.fillStyle = p > 0.5 ? "#3ccb7f" : p > 0.25 ? "#fac515" : "#eb3751";
    ctx.fillRect(k.x - w / 2, k.y - r - 6, w * p, 3);
  }
}

function draw() {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  if (bgCanvas) ctx.drawImage(bgCanvas, 0, 0);
  ctx.setTransform(pixelScale, 0, 0, pixelScale, 0, 0);
  if (!S) return;

  // route preview
  ctx.fillStyle = "rgba(250,197,21,0.22)";
  for (const pts of S.path) {
    for (let j = 0; j < pts.length; j++) {
      const i = pts[j];
      ctx.beginPath();
      ctx.arc((i % COLS + 0.5) * CELL, (((i / COLS) | 0) + 0.5) * CELL, 1.8, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  // ranges
  const showRange = (x, y, range, color) => {
    ctx.fillStyle = color + "18";
    ctx.strokeStyle = color + "88";
    ctx.lineWidth = 1;
    ctx.beginPath(); ctx.arc(x, y, range, 0, Math.PI * 2); ctx.fill(); ctx.stroke();
  };
  if (selected) showRange(selected.x, selected.y, towerStats(selected).range, "#ffffff");

  for (const t of S.towers) {
    drawTowerShape(ctx, t.type, t.level, t.x, t.y, t.angle, 1.08);
    if (t === selected) {
      ctx.strokeStyle = "#fac515"; ctx.lineWidth = 2;
      ctx.strokeRect(t.x - CELL + 1, t.y - CELL + 1, CELL * 2 - 2, CELL * 2 - 2);
    }
  }

  // ghost
  if (buildType >= 0 && hover && !S.over) {
    const r = placeCheck(hover.cx, hover.cy);
    const ok = r === "ok" && S.gold >= TOWERS[buildType].cost;
    const x = (hover.cx + 1) * CELL, y = (hover.cy + 1) * CELL;
    showRange(x, y, TOWERS[buildType].range, ok ? "#3ccb7f" : "#eb3751");
    ctx.globalAlpha = 0.65;
    drawTowerShape(ctx, buildType, 0, x, y, -Math.PI / 2, 1.08);
    ctx.globalAlpha = 1;
    ctx.fillStyle = ok ? "rgba(60,203,127,0.25)" : "rgba(235,55,81,0.35)";
    ctx.fillRect(hover.cx * CELL, hover.cy * CELL, CELL * 2, CELL * 2);
  }

  for (const k of S.creeps) if (!k.flying) drawCreep(k);
  for (const k of S.creeps) if (k.flying) drawCreep(k);

  // shots
  for (const s of S.shots) {
    ctx.fillStyle = s.color;
    ctx.strokeStyle = s.color;
    if (s.kind === "dart") {
      const a = Math.atan2(s.ty - s.y, s.tx - s.x);
      ctx.lineWidth = 2;
      ctx.beginPath(); ctx.moveTo(s.x, s.y); ctx.lineTo(s.x - Math.cos(a) * 9, s.y - Math.sin(a) * 9); ctx.stroke();
    } else if (s.kind === "missile") {
      ctx.beginPath(); ctx.arc(s.x, s.y, 2.5, 0, Math.PI * 2); ctx.fill();
      ctx.fillStyle = "rgba(255,180,120,0.4)";
      ctx.beginPath(); ctx.arc(s.x - 2, s.y + 1, 2, 0, Math.PI * 2); ctx.fill();
    } else if (s.kind === "shell") {
      ctx.beginPath(); ctx.arc(s.x, s.y, 3.5, 0, Math.PI * 2); ctx.fill();
    } else {
      ctx.beginPath(); ctx.arc(s.x, s.y, 2.2, 0, Math.PI * 2); ctx.fill();
    }
  }

  // effects
  for (const f of S.fx) {
    const p = 1 - f.life / f.maxLife;
    ctx.globalAlpha = 1 - p;
    if (f.kind === "ring") {
      ctx.strokeStyle = f.color; ctx.lineWidth = 2;
      ctx.beginPath(); ctx.arc(f.x, f.y, f.r + (f.max - f.r) * p, 0, Math.PI * 2); ctx.stroke();
    } else {
      ctx.fillStyle = f.color;
      for (let i = 0; i < 6; i++) {
        const a = (i / 6) * Math.PI * 2;
        ctx.beginPath(); ctx.arc(f.x + Math.cos(a) * (f.r + 10 * p), f.y + Math.sin(a) * (f.r + 10 * p), 2, 0, Math.PI * 2); ctx.fill();
      }
    }
  }
  ctx.globalAlpha = 1;

  ctx.font = "bold 11px ui-sans-serif, system-ui, sans-serif";
  ctx.textAlign = "center";
  for (const f of S.floats) {
    ctx.globalAlpha = Math.min(1, f.life / f.max * 2);
    ctx.fillStyle = "rgba(0,0,0,0.6)";
    ctx.fillText(f.text, f.x + 1, f.y + 1);
    ctx.fillStyle = f.color;
    ctx.fillText(f.text, f.x, f.y);
  }
  ctx.globalAlpha = 1;
  ctx.textAlign = "start";

  if (S.paused && !S.over) {
    ctx.fillStyle = "rgba(10,16,28,0.55)";
    ctx.fillRect(0, 0, W, H);
    ctx.fillStyle = "#fff";
    ctx.font = "800 28px ui-sans-serif, system-ui, sans-serif";
    ctx.textAlign = "center";
    ctx.fillText("PAUSE", W / 2, H / 2);
    ctx.font = "13px ui-sans-serif, system-ui, sans-serif";
    ctx.fillStyle = "#d0d5dd";
    ctx.fillText("Appuyez sur Espace pour reprendre", W / 2, H / 2 + 24);
    ctx.textAlign = "start";
  } else if (!S.started && !S.over && !ui.overlayOpen) {
    ctx.fillStyle = "rgba(10,16,28,0.6)";
    roundRect(ctx, W / 2 - 170, 14, 340, 44, 8); ctx.fill();
    ctx.fillStyle = "#fff";
    ctx.font = "600 12px ui-sans-serif, system-ui, sans-serif";
    ctx.textAlign = "center";
    ctx.fillText("Construisez votre labyrinthe, puis lancez la vague", W / 2, 32);
    ctx.fillStyle = "#fac515";
    ctx.fillText("Bouton « Vague suivante » ou touche N", W / 2, 49);
    ctx.textAlign = "start";
  }
}

// ---------- Interface ----------
const $ = (id) => document.getElementById(id);
const overlay = $("overlay");

const ui = {
  overlayOpen: false,
  last: {},
  buildTowerButtons() {
    const box = $("towers");
    box.innerHTML = "";
    TOWERS.forEach((d, i) => {
      const b = document.createElement("button");
      b.className = "tbtn";
      b.title = `${d.name} (${d.key})`;
      b.innerHTML = `<kbd>${d.key}</kbd><canvas width="56" height="56"></canvas><span class="nm">${d.name}</span><span class="c">${d.cost} or</span>`;
      const g = b.querySelector("canvas").getContext("2d");
      g.scale(2, 2);
      drawTowerShape(g, i, 0, 14, 14, -Math.PI / 2, 0.75);
      b.addEventListener("click", () => setBuild(buildType === i ? -1 : i));
      b.addEventListener("mouseenter", () => { hoverTowerBtn = i; ui.info(); });
      b.addEventListener("mouseleave", () => { hoverTowerBtn = -1; ui.info(); });
      box.appendChild(b);
    });
  },
  update(force) {
    if (!S) return;
    const set = (id, v) => { if (force || ui.last[id] !== v) { $(id).textContent = v; ui.last[id] = v; } };
    const totalWaves = MODES[S.mode].waves;
    set("s-wave", `${S.wave}/${totalWaves}`);
    set("s-lives", String(S.lives));
    set("s-gold", String(S.gold));
    set("s-score", String(S.score));
    $("s-lives").classList.toggle("low", S.lives <= 5);
    [...$("towers").children].forEach((b, i) => {
      b.classList.toggle("sel", i === buildType);
      b.classList.toggle("poor", S.gold < TOWERS[i].cost);
    });
    $("b-next").disabled = S.over || S.wave >= totalWaves;
    $("b-pause").textContent = S.paused ? "Reprendre" : "Pause";
    $("b-speed").textContent = `Vitesse ×${speed}`;
    ui.waves(force);
    ui.info();
  },
  waves(force) {
    const box = $("waves");
    const key = `${S.wave}`;
    if (force || box.dataset.k !== key) {
      box.dataset.k = key;
      box.innerHTML = "";
      const totalWaves = MODES[S.mode].waves;
      for (let n = S.wave + 1; n <= Math.min(totalWaves, S.wave + 7); n++) {
        const w = waveDef(n, S.mode);
        const c = CREEPS[w.type];
        const el = document.createElement("div");
        el.className = "wave" + (w.type === "boss" ? " boss" : "");
        const from = n >= 15 ? "← ↑" : n % 2 ? "←" : "↑";
        el.innerHTML = `<span class="dot" style="background:${c.color}"></span><b>${n}. ${c.name}</b><i>${from} ${w.count} · ${Math.round(w.hp * c.hp * MODES[S.mode].hp)} PV</i>${n === S.wave + 1 ? '<span class="bar"></span>' : ""}`;
        el.title = c.desc || "";
        box.appendChild(el);
      }
      if (S.wave >= totalWaves) box.innerHTML = '<div class="wave"><b>Dernière vague envoyée !</b><i>Tenez bon jusqu\'au bout.</i></div>';
    }
    const bar = box.querySelector(".bar");
    if (bar) {
      const gap = MODES[S.mode].gap;
      bar.style.width = S.started ? `${(1 - S.timer / gap) * 100}%` : "0%";
    }
  },
  info() {
    if (!S) return;
    const box = $("info");
    let html = "";
    const showType = buildType >= 0 ? buildType : hoverTowerBtn;
    if (selected) {
      const t = selected;
      const d = TOWERS[t.type];
      const st = towerStats(t);
      const max = t.level >= d.ups.length;
      const nx = max ? null : towerStats({ ...t, level: t.level + 1 });
      const upCost = max ? 0 : d.ups[t.level];
      const row = (k, v, nv) => `<tr><td>${k}</td><td>${v}${nv !== undefined && nv !== v ? ` <span class="up">→ ${nv}</span>` : ""}</td></tr>`;
      html = `<h3>${d.name} <small>niveau ${t.level + 1}/5</small></h3>
        <div class="desc">${d.desc}</div>
        <table>
        ${row("Dégâts", st.dmg, nx && nx.dmg)}
        ${row("Portée", Math.round(st.range), nx && Math.round(nx.range))}
        ${row("Cadence", st.rate.toFixed(2) + " s", nx && nx.rate.toFixed(2) + " s")}
        ${d.slow ? row("Ralentit", Math.round(st.slow * 100) + " %", nx && Math.round(nx.slow * 100) + " %") : ""}
        ${row("Vise", [d.ground && "sol", d.air && "air"].filter(Boolean).join(" + "))}
        </table>
        <div class="acts">
          <button class="btn primary" id="b-up">${max ? "Niveau max" : `Améliorer ${upCost} <kbd>U</kbd>`}</button>
          <button class="btn danger" id="b-sell">Vendre ${sellValue(t)} <kbd>S</kbd></button>
        </div>`;
    } else if (showType >= 0) {
      const d = TOWERS[showType];
      html = `<h3>${d.name} <small>${d.cost} or</small></h3>
        <div class="desc">${d.desc}</div>
        <table>
        <tr><td>Dégâts</td><td>${d.dmg[0]} → ${d.dmg[4]}</td></tr>
        <tr><td>Portée</td><td>${d.range}</td></tr>
        <tr><td>Cadence</td><td>${d.rate.toFixed(2)} s</td></tr>
        <tr><td>Vise</td><td>${[d.ground && "sol", d.air && "air"].filter(Boolean).join(" + ")}</td></tr>
        </table>
        <div class="desc" style="margin-top:6px">${buildType >= 0 ? "Cliquez sur le plateau pour la poser. Maj : en poser plusieurs. 0 : annuler." : "Touche " + d.key + " pour la choisir."}</div>`;
    } else {
      html = `<h3>Comment jouer</h3>
        <div class="desc">Les ennemis traversent le plateau de gauche à droite et de haut en bas. Posez des tours pour leur barrer la route et allonger leur chemin : ils contournent toujours vos tours, mais vous ne pouvez jamais fermer complètement le passage. Les volants passent au-dessus de tout.</div>
        <div class="desc">Cliquez sur une tour posée pour l'améliorer ou la vendre.</div>`;
    }
    const key = html;
    if (box.dataset.k !== key) {
      box.dataset.k = key;
      box.innerHTML = html;
      const up = $("b-up"), sl = $("b-sell");
      if (up) up.addEventListener("click", () => upgrade(selected));
      if (sl) sl.addEventListener("click", () => sell(selected));
    }
    const up = $("b-up");
    if (up && selected) {
      const d = TOWERS[selected.type];
      up.disabled = selected.level >= d.ups.length || S.gold < d.ups[selected.level];
    }
  },
  showStart(open = true) {
    ui.overlayOpen = open;
    overlay.classList.toggle("show", open);
    if (!open) return;
    const best = (m) => +(localStorage.getItem(`dd-best-${m}`) || 0);
    overlay.innerHTML = `<div class="card">
      <h2>Dédale <span>Défense</span></h2>
      <p>Bâtissez un labyrinthe de tours et empêchez les vagues d'ennemis de traverser le plateau. Choisissez votre défi !</p>
      <div class="modes">
        ${Object.entries(MODES).map(([k, m]) => `<button class="btn ${k === "normal" ? "primary" : ""}" data-mode="${k}" title="${m.desc || ""}">${m.name} <span class="best">· ${m.waves} vagues · ${m.gap}s entre vagues · ${m.lives} vies · record ${best(k)}</span></button>`).join("")}
      </div>
    </div>`;
    overlay.querySelectorAll("[data-mode]").forEach((b) => b.addEventListener("click", () => newGame(b.dataset.mode)));
  },
  showEnd() {
    ui.overlayOpen = true;
    overlay.classList.add("show");
    overlay.innerHTML = `<div class="card">
      <h2>${S.won ? "Victoire !" : "Partie terminée"}</h2>
      <p>${S.won ? `Les ${MODES[S.mode].waves} vagues sont passées sans percer votre labyrinthe.` : `Vous avez tenu jusqu'à la vague ${S.wave}.`} ${S.kills} ennemis arrêtés.</p>
      <div class="big">${S.score}</div>
      <p class="best">${S.newBest ? "Nouveau record !" : `Record : ${S.best}`} · mode ${MODES[S.mode].name}</p>
      <div class="modes">
        <button class="btn primary" id="b-again">Rejouer</button>
        <button class="btn" id="b-modes">Changer de mode</button>
      </div>
    </div>`;
    $("b-again").addEventListener("click", () => newGame(S.mode));
    $("b-modes").addEventListener("click", () => ui.showStart(true));
  },
};

function setBuild(i) {
  buildType = i;
  if (i >= 0) selected = null;
  canvas.style.cursor = i >= 0 ? "none" : "crosshair";
  ui.update();
}

function togglePause() {
  if (!S || S.over || ui.overlayOpen) return;
  S.paused = !S.paused;
  ui.update();
}

// ---------- Input ----------
function boardPos(e) {
  const r = canvas.getBoundingClientRect();
  const x = ((e.clientX - r.left) / r.width) * W;
  const y = ((e.clientY - r.top) / r.height) * H;
  return { x, y, cx: Math.round(x / CELL - 1), cy: Math.round(y / CELL - 1) };
}

canvas.addEventListener("pointermove", (e) => { hover = boardPos(e); });
canvas.addEventListener("pointerleave", () => { hover = null; });
canvas.addEventListener("pointerdown", (e) => {
  if (!S || S.over || ui.overlayOpen) return;
  if (e.button === 2) { setBuild(-1); selected = null; ui.update(); return; }
  hover = boardPos(e);
  shiftHeld = e.shiftKey;
  if (buildType >= 0) { tryPlace(hover.cx, hover.cy); return; }
  const cx = Math.floor(hover.x / CELL), cy = Math.floor(hover.y / CELL);
  const t = cx >= 0 && cy >= 0 && cx < COLS && cy < ROWS ? S.towerAt[cy * COLS + cx] : null;
  selected = t || null;
  ui.update();
});
canvas.addEventListener("contextmenu", (e) => e.preventDefault());

window.addEventListener("keydown", (e) => {
  if (e.key === "Shift") shiftHeld = true;
  if (!S || ui.overlayOpen) return;
  const k = e.key.toLowerCase();
  if (k >= "1" && k <= "6") { setBuild(+k - 1); e.preventDefault(); }
  else if (k === "0" || k === "escape") { setBuild(-1); selected = null; ui.update(); }
  else if (k === "u") upgrade(selected);
  else if (k === "s") sell(selected);
  else if (k === " ") { togglePause(); e.preventDefault(); }
  else if (k === "n") { if (!S.paused) sendWave(); }
  else if (k === "f") toggleSpeed();
});
window.addEventListener("keyup", (e) => { if (e.key === "Shift") shiftHeld = false; });
window.addEventListener("blur", () => { shiftHeld = false; });

function toggleSpeed() {
  speed = speed === 1 ? 2 : speed === 2 ? 3 : 1;
  ui.update();
}

$("b-next").addEventListener("click", () => { if (S && !S.paused) sendWave(); });
$("b-pause").addEventListener("click", togglePause);
$("b-speed").addEventListener("click", toggleSpeed);
$("b-menu").addEventListener("click", () => { if (S) S.paused = false; ui.showStart(true); });

// Buttons lose focus after a click so Space keeps meaning "pause".
document.addEventListener("click", (e) => {
  const b = e.target.closest && e.target.closest("button");
  if (b) b.blur();
});

// ---------- Fitting the game to the window ----------
const game = $("game");
const stage = $("stage");
function fit() {
  const vw = document.documentElement.clientWidth;
  const vh = window.innerHeight;
  game.classList.toggle("stacked", vw < 880);
  game.style.transform = "none";
  const w = game.offsetWidth, h = game.offsetHeight;
  let scale = (vw - 16) / w;
  if (!game.classList.contains("stacked")) scale = Math.min(scale, (vh - stage.offsetTop - 20) / h);
  scale = Math.max(0.2, Math.min(scale, 1.6));
  game.style.transform = `scale(${scale})`;
  game.style.left = `${Math.max(8, (vw - w * scale) / 2)}px`;
  stage.style.height = `${h * scale + 16}px`;
  setupCanvas(scale);
}
window.addEventListener("resize", fit);

// ---------- Loop ----------
let lastT = performance.now();
let acc = 0;
const STEP = 1 / 60;
function frame(now) {
  const dt = Math.min(0.1, (now - lastT) / 1000);
  lastT = now;
  acc += dt * speed;
  while (acc >= STEP) { update(STEP); acc -= STEP; }
  draw();
  if (S) {
    ui.update();
  }
  requestAnimationFrame(frame);
}

ui.buildTowerButtons();
newGame("normal");
ui.showStart(true);
fit();
requestAnimationFrame(frame);

// Small hook so automated checks can drive the game.
window.__game = {
  get state() { return S; }, newGame, sendWave, setBuild, tryPlace, upgrade, sell, placeCheck,
  step(seconds) { for (let i = 0; i < seconds * 60 && !S.over; i++) update(STEP); },
};
