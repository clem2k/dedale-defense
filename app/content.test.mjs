import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { copyFileSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import vm from "node:vm";
import { pathToFileURL } from "node:url";
import { loadContent, validateContent } from "./content.mjs";
import { parseCsv } from "./csv.mjs";
import { FX_NAMES, TOWER_FX } from "./src/audio.js";

const content = loadContent();
const source = readFileSync(new URL("./src/game.js", import.meta.url), "utf8");
const towerCsv = readFileSync(new URL("../towers.csv", import.meta.url), "utf8");
const enemyCsv = readFileSync(new URL("../ennemies.csv", import.meta.url), "utf8");
const csv = parseCsv(towerCsv, "towers.csv").map(Object.values);

test("construction hint expires at five real seconds and dismisses immediately", () => {
  let now = 1000;
  const game = simulation(content, () => now);
  game.newGame("normal");
  assert.equal(game.showStartHint(), true);
  now = 5999;
  assert.equal(game.showStartHint(), true);
  now = 6000;
  assert.equal(game.showStartHint(), false);
  game.newGame("normal");
  assert.equal(game.showStartHint(), true);
  game.dismissStartHint();
  assert.equal(game.showStartHint(), false);
  game.setBuild(0);
  game.tryPlace(5, 5);
  assert.equal(game.state.towers.length, 1);
  assert.equal(game.showStartHint(), false);
});

test("all tower sprites select the correct row and nearest of eight directions", () => {
  const game = simulation();
  for (let type = 0; type < content.towers.length; type++) {
    const sprite = content.towers[type].sprite;
    for (let direction = 0; direction < 8; direction++) {
      for (const turns of [-2, -1, 0, 1, 2]) {
        for (const offset of [-0.1, 0, 0.1]) {
          const frame = game.towerSpriteFrame(type, direction * Math.PI / 4 + turns * Math.PI * 2 + offset);
          assert.equal(frame.x, (sprite.column + direction) * sprite.frameSize);
          assert.equal(frame.y, sprite.row * sprite.frameSize);
        }
      }
    }
  }
});

test("tower rendering uses the atlas without rotating it and preserves level indicators", () => {
  const game = simulation();
  const renderer = source.slice(source.indexOf("function drawTowerShape("), source.indexOf("function roundRect("));
  const image = {};
  const draw = vm.runInNewContext(`${renderer}; drawTowerShape`, {
    TOWERS: content.towers,
    spriteImages: new Map(content.towers.map((t) => [t.sprite.image, image])),
    towerSpriteFrame: game.towerSpriteFrame,
  });
  for (let type = 0; type < 6; type++) {
    for (let direction = 0; direction < 8; direction++) {
      const calls = [];
      let pips = 0;
      const g = {
        save() {}, restore() {}, translate() {}, scale() {}, beginPath() {}, fill() {},
        arc() { pips++; }, drawImage(...args) { calls.push(args); },
      };
      draw(g, type, 5, 100, 120, direction * Math.PI / 4);
      assert.equal(calls.length, 1);
      const sprite = content.towers[type].sprite;
      assert.deepEqual(calls[0], [image, (sprite.column + direction) * sprite.frameSize, sprite.row * sprite.frameSize,
        sprite.frameSize, sprite.frameSize, -sprite.drawSize / 2, -sprite.drawSize / 2, sprite.drawSize, sprite.drawSize]);
      assert.equal(g.imageSmoothingEnabled, false);
      assert.equal(pips, 5);
    }
  }
});

test("enemy rendering selects every sprite and preserves flight, health and status effects", () => {
  const game = simulation();
  game.newGame("normal");
  game.state.time = 1;
  const renderer = source.slice(source.indexOf("function drawCreep("), source.indexOf("function draw()"));
  const image = {};
  for (const [type, definition] of Object.entries(content.enemies)) {
    const calls = [];
    const rings = [];
    let shadows = 0, healthBars = 0, stunMarkers = 0;
    const context = {
      save() {}, restore() {}, translate() {}, beginPath() {}, fill() {}, stroke() {},
      ellipse() { shadows++; }, arc(...args) { rings.push(args); },
      fillRect() { healthBars++; }, fillText() { stunMarkers++; },
      drawImage(...args) { calls.push(args); },
    };
    const draw = vm.runInNewContext(`${renderer}; drawCreep`, {
      S: game.state, ctx: context, CREEPS: content.enemies,
      spriteImages: new Map(Object.values(content.enemies).map((e) => [e.sprite.image, image])),
      enemySpriteFrame: game.enemySpriteFrame,
    });
    const creep = game.makeCreep(type, 100, 1, "R", null);
    creep.hp = creep.maxHp / 2;
    creep.slowUntil = 2;
    creep.stunUntil = 2;
    draw(creep);
    const sprite = definition.sprite;
    const size = definition.r * sprite.drawScale;
    assert.deepEqual(calls, [[image, sprite.column * sprite.frameSize, sprite.row * sprite.frameSize,
      sprite.frameSize, sprite.frameSize, -size / 2, -size / 2, size, size]]);
    assert.equal(context.imageSmoothingEnabled, false);
    assert.equal(shadows, definition.flying ? 1 : 0);
    assert.equal(rings.length, 1);
    assert.equal(rings[0][2], definition.r + 2);
    assert.equal(healthBars, 2);
    assert.equal(stunMarkers, 1);
    calls.length = 0;
    rings.length = 0;
    shadows = healthBars = stunMarkers = 0;
    creep.hp = creep.maxHp;
    creep.slowUntil = creep.stunUntil = 0;
    draw(creep);
    assert.equal(calls.length, 1);
    assert.equal(rings.length, 0);
    assert.equal(healthBars, 0);
    assert.equal(stunMarkers, 0);
  }
});

test("all 36 CSV rows match the shared tower definitions exactly", () => {
  assert.equal(csv.length, 36);
  for (const r of csv) {
    const t = content.towers[Number(r[0]) - 1], i = Number(r[1]) - 1;
    assert.equal(i === 0 ? t.cost : t.ups[i - 1], Number(r[2]));
    assert.equal(t.dmg[i], Number(r[3]));
    assert.equal(t.range[i], Number(r[4]));
    assert.equal(1 / t.rate[i], Number(r[5]));
    assert.equal(t.sell[i], Number(r[6]));
    assert.equal(t.special[i], r[7] || "none");
    assert.equal(t.factor[i], Number(r[8] || 0));
    assert.equal(t.ground, r[9] === "1");
    assert.equal(t.air, r[10] === "1");
  }
});

test("grid fits exactly 13 by 11 towers and range 180 spans its width", () => {
  assert.equal((content.board.cols - 2) / 2, 13);
  assert.equal((content.board.rows - 2) / 2, 11);
  assert.equal(180 * content.board.rangeScale * 2, (content.board.cols - 2) * content.board.cell);
  const game = simulation();
  game.newGame("uncle_scrooge");
  game.setBuild(0);
  game.tryPlace(25, 21);
  assert.equal(game.state.towers.length, 1);
  game.setBuild(0);
  game.tryPlace(26, 21);
  game.tryPlace(24, 20);
  assert.equal(game.state.towers.length, 1);
});

test("half-tower placement permits offset towers and narrow corridors", () => {
  const game = simulation();
  game.newGame("uncle_scrooge");
  game.setBuild(0);
  game.tryPlace(5, 5);
  assert.equal(game.placeCheck(8, 6), "ok");
  game.setBuild(0);
  game.tryPlace(8, 6);
  assert.equal(game.state.towers.length, 2);
  assert.equal(game.placeCheck(6, 5), "taken");
  assert.equal(game.placeCheck(4.5, 5), "out");
  const cell = content.board.cell, cols = content.board.cols;
  const enemy = game.makeCreep("walker", 100, 0, "R", null);
  enemy.x = 7.5 * cell;
  enemy.y = 5.5 * cell;
  enemy.wcell = 6 * cols + 7;
  enemy.wx = enemy.x;
  enemy.wy = 6.5 * cell;
  game.moveCreep(enemy, cell / enemy.speed);
  assert.ok(Math.abs(enemy.y - 6.5 * cell) < 0.0001);
  assert.equal(enemy.dead, false);
  assert.equal(enemy.cell, 6 * cols + 7);
});

test("centered entrances match reference widths and every enemy fits a half-tower corridor", () => {
  const { cell, cols, rows, openRows, openCols } = content.board;
  assert.equal(openRows.length * cell, 3 * 2 * cell);
  assert.equal(openCols.length * cell, 4 * 2 * cell);
  assert.equal(openRows[0] + openRows.at(-1) + 1, rows);
  assert.equal(openCols[0] + openCols.at(-1) + 1, cols);
  for (const enemy of Object.values(content.enemies)) {
    assert.ok(enemy.r * 2 < cell, `${enemy.name} must fit in a half-tower corridor`);
  }
});

test("mouse coordinates snap to half-tower centers at any display scale", () => {
  const game = simulation();
  for (const [cx, cy] of [[1, 1], [6, 5], [8, 6], [24, 20], [25, 21]]) {
    for (const offset of [-9, 0, 9]) {
      const pos = game.boardPos({
        clientX: 10 + ((cx + 1) * content.board.cell + offset) / 2,
        clientY: 20 + ((cy + 1) * content.board.cell + offset) / 2,
      });
      assert.equal(pos.cx, cx);
      assert.equal(pos.cy, cy);
    }
  }
});

test("offset towers still prevent diagonal corner cutting", () => {
  const game = simulation();
  game.newGame("normal");
  game.setBuild(0);
  game.tryPlace(5, 5);
  const cols = content.board.cols;
  const field = new Float32Array(cols * content.board.rows).fill(200);
  field[4 * cols + 5] = 0;
  field[4 * cols + 4] = 100;
  assert.equal(game.nextStep(field, 4, 5), 4 * cols + 4);
});

function withFixture(run) {
  const directory = mkdtempSync(join(tmpdir(), "dedale-csv-"));
  try {
    const app = join(directory, "app");
    mkdirSync(app);
    for (const name of ["content.mjs", "csv.mjs", "build.mjs"]) {
      copyFileSync(new URL(`./${name}`, import.meta.url), join(app, name));
    }
    cpSync(new URL("./src", import.meta.url), join(app, "src"), { recursive: true });
    cpSync(new URL("../assets/fx", import.meta.url), join(directory, "assets/fx"), { recursive: true });
    for (const file of ["towers.csv", "ennemies.csv", "game-content.json"]) {
      copyFileSync(new URL(`../${file}`, import.meta.url), join(directory, file));
    }
    for (const image of new Set([...content.towers, ...Object.values(content.enemies)].map((d) => d.sprite.image))) {
      const path = join(directory, image);
      mkdirSync(join(path, ".."), { recursive: true });
      copyFileSync(new URL(`../${image}`, import.meta.url), path);
    }
    run(directory, pathToFileURL(directory + "/"));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

function editCsv(directory, file, edit) {
  const path = join(directory, file);
  const rows = parseCsv(readFileSync(path, "utf8"), file);
  edit(rows);
  const headers = Object.keys(rows[0]);
  const quote = (value) => /[,"\r\n]/.test(value) ? `"${value.replaceAll('"', '""')}"` : value;
  writeFileSync(path, [headers, ...rows.map((row) => headers.map((key) => row[key]))]
    .map((row) => row.map(quote).join(",")).join("\r\n") + "\r\n");
}

test("editing only CSV stats, metadata, sprite file and coordinates changes the built game", () => {
  withFixture((directory, root) => {
    const config = readFileSync(join(directory, "game-content.json"), "utf8");
    const alternate = "assets/towers/alternate.png";
    copyFileSync(join(directory, content.towers[0].sprite.image), join(directory, alternate));
    editCsv(directory, "towers.csv", (rows) => {
      rows[0].damage = "123";
      rows[0].name = 'Tour "test", modifiée';
      rows[0].desc = "Première ligne\nDeuxième ligne";
      rows[0]["sprite file"] = alternate;
      rows[0]["sprite row"] = "2";
      rows[0]["draw size"] = "36";
    });
    editCsv(directory, "ennemies.csv", (rows) => {
      rows[0].hp = "2.5";
      rows[0]["sprite column"] = "10";
      rows[0]["draw scale"] = "3";
    });
    execFileSync(process.execPath, [join(directory, "app/build.mjs")]);
    const module = readFileSync(join(directory, "app/dist/content.js"), "utf8");
    const built = JSON.parse(module.slice("export const CONTENT = ".length).trim().replace(/;$/, ""));
    assert.equal(built.towers[0].dmg[0], 123);
    assert.equal(built.towers[0].name, 'Tour "test", modifiée');
    assert.equal(built.towers[0].desc, "Première ligne\nDeuxième ligne");
    assert.equal(built.towers[0].sprite.image, alternate);
    assert.equal(built.towers[0].sprite.row, 1);
    assert.equal(built.towers[0].sprite.drawSize, 36);
    assert.equal(built.enemies.walker.hp, 2.5);
    assert.equal(built.enemies.walker.sprite.column, 9);
    assert.equal(built.enemies.walker.sprite.drawScale, 3);
    assert.deepEqual(readFileSync(join(directory, "app/dist", alternate)), readFileSync(join(directory, alternate)));
    assert.deepEqual(built.board, content.board);
    assert.deepEqual(built.waveSets, content.waveSets);
    assert.equal(readFileSync(join(directory, "game-content.json"), "utf8"), config);
    assert.deepEqual(loadContent(root), built);
    const game = simulation(built);
    game.newGame("normal");
    game.setBuild(0);
    game.tryPlace(5, 5);
    assert.equal(game.towerStats(game.state.towers[0]).dmg, 123);
    assert.equal(game.enemySpriteFrame("walker").x, 9 * 208);
    assert.equal(game.towerSpriteFrame(0, 0).y, 208);
    game.sendWave();
    game.update(0);
    assert.equal(game.state.creeps[0].hp, built.waveSets.standard[0].hp * 2.5);
  });
});

test("CSV parser accepts BOM, CRLF, escaped quotes and embedded newlines", () => {
  assert.deepEqual(parseCsv('\uFEFFid,desc\r\na,"Un ""nom"",\nune suite"\r\n', "test.csv"),
    [{ id: "a", desc: 'Un "nom",\nune suite' }]);
  for (const invalid of ['id,id\na,b', 'id,desc\na', 'id\n"unfinished', 'id\n"x"oops']) {
    assert.throws(() => parseCsv(invalid, "test.csv"), /test.csv:/);
  }
});

test("CSV validation rejects invalid numbers, inconsistent levels, splits and sprite references", () => {
  withFixture((directory, root) => {
    for (const [file, edit, expected] of [
      ["towers.csv", (r) => { r[0].damage = ""; }, /invalid number/],
      ["towers.csv", (r) => { r[1].level = "1"; }, /levels 1 to 6/],
      ["towers.csv", (r) => { r[1].name = "ignored?"; }, /metadata belongs/],
      ["towers.csv", (r) => { r[0].speed = "0"; }, /invalid effects/],
      ["towers.csv", (r) => { r[1].earth = "0"; }, /target flags/],
      ["towers.csv", (r) => { r[0]["sprite row"] = "7"; }, /frame outside/],
      ["towers.csv", (r) => { r[0]["sprite column"] = "2"; }, /frame outside/],
      ["ennemies.csv", (r) => { r[0].hp = "NaN"; }, /invalid number/],
      ["ennemies.csv", (r) => { r[1].id = r[0].id; }, /duplicate enemy/],
      ["ennemies.csv", (r) => { r[0]["split count"] = "2"; }, /incomplete split/],
      ["ennemies.csv", (r) => { r[0]["sprite column"] = "0"; }, /invalid enemy sprite/],
      ["ennemies.csv", (r) => { r[0]["sprite file"] = "../outside.png"; }, /invalid enemy sprite/],
      ["ennemies.csv", (r) => { r[0]["sprite file"] = "assets/missing.png"; }, /ENOENT/],
    ]) {
      editCsv(directory, file, edit);
      assert.throws(() => loadContent(root), expected);
      writeFileSync(join(directory, file), file === "towers.csv" ? towerCsv : enemyCsv);
    }
    const config = JSON.parse(readFileSync(join(directory, "game-content.json"), "utf8"));
    config.towers = [];
    writeFileSync(join(directory, "game-content.json"), JSON.stringify(config));
    assert.throws(() => loadContent(root), /must be defined only/);
  });
});

for (const special of ["spread", "slow", "stun"]) {
  test(`${special} affects only compatible enemies inside both circles`, () => {
    const edited = structuredClone(content);
    const type = special === "spread" ? 2 : special === "slow" ? 4 : 5;
    edited.towers[type].factor[0] = special === "spread" ? 4 : 1;
    const game = simulation(edited);
    game.newGame("normal");
    const make = (kind, x, y) => {
      const c = game.makeCreep(kind, 1000, 0, "R", null);
      c.x = x; c.y = y;
      return c;
    };
    const target = make("walker", 30, 0);
    const nearby = make("walker", 40, 0);
    const outsideTower = make("walker", 60, 0);
    const outsideTarget = make("walker", -30, 0);
    const immune = make("armored", 40, 0);
    const flyer = make("flyer", 40, 0);
    game.impact({ tower: { type, x: 0, y: 0 }, tx: 30, ty: 0, target, range: 50, dmg: 8, special, factor: edited.towers[type].factor[0] });
    assert.equal(target.hp, 992);
    assert.equal(nearby.hp, special === "spread" ? 996 : 1000);
    assert.equal(outsideTower.hp, 1000);
    assert.equal(outsideTower.slowUntil, 0);
    assert.equal(outsideTower.stunUntil, 0);
    assert.equal(outsideTarget.hp, 1000);
    assert.equal(outsideTarget.slowUntil, 0);
    assert.equal(outsideTarget.stunUntil, 0);
    if (special === "slow") {
      assert.equal(nearby.slowF, 0.5);
      assert.equal(nearby.slowUntil, 2);
      assert.equal(flyer.slowUntil, 2);
      assert.equal(immune.slowUntil, 0);
    } else if (special === "stun") {
      assert.equal(nearby.stunUntil, 2);
      assert.equal(flyer.stunUntil, 0);
      assert.equal(immune.stunUntil, 0);
    } else assert.equal(flyer.hp, 1000);
  });
}

function simulation(data = content, now = () => 0) {
  const logic = source.slice(0, source.indexOf("// ---------- Drawing ----------"))
    .replace('import { CONTENT } from "./content.js";', "")
    .replace('import { FxAudio, TOWER_FX } from "./audio.js";', "");
  const audio = { played: [], play(name) { this.played.push(name); } };
  const setBuild = source.slice(source.indexOf("function setBuild("), source.indexOf("function togglePause("));
  const boardPos = source.slice(source.indexOf("function boardPos("), source.indexOf('canvas.addEventListener("pointermove"'));
  return vm.runInNewContext(`${logic}\n${setBuild}\n${boardPos}
    ({ get state() { return S; }, audio, leak, newGame, waveDef, sendWave, update,
       setBuild, tryPlace, placeCheck, moveCreep, boardPos, nextStep, towerStats, sellValue, upgrade, sell, hurt, spawn, makeCreep, impact, fire, updateShots, showStartHint, dismissStartHint, towerSpriteFrame, enemySpriteFrame })`, {
    CONTENT: data,
    TOWER_FX, audio,
    performance: { now },
    canvas: { style: {}, getBoundingClientRect: () => ({
      left: 10, top: 20, width: content.board.cols * content.board.cell / 2, height: content.board.rows * content.board.cell / 2,
    }) },
    ui: { overlayOpen: false, showStart() {}, update() {}, showEnd() {}, info() {} },
    localStorage: { getItem() { return null; }, setItem() {} },
  });
}

test("every level and wave comes from the shared file", () => {
  const game = simulation();
  for (const level of content.levels) {
    game.newGame(level.id);
    assert.equal(game.state.gold, level.gold);
    assert.equal(game.state.lives, level.lives);
    for (let n = 1; n <= level.waves; n++) {
      const wave = game.waveDef(n, level.id);
      const expected = content.waveSets[level.waveSet][n - 1];
      const enemy = content.enemies[expected.type];
      assert.equal(wave.type, expected.type);
      assert.equal(wave.hp, expected.hp);
      assert.equal(wave.count, enemy.count);
      assert.equal(wave.bounty, enemy.bounty * expected.bountyMultiplier);
      assert.deepEqual(wave.targets, expected.targets);
    }
    assert.throws(() => game.waveDef(0, level.id), /Invalid wave/);
    assert.throws(() => game.waveDef(level.waves + 1, level.id), /Invalid wave/);
    for (let n = 0; n < level.waves + 1; n++) game.sendWave();
    assert.equal(game.state.wave, level.waves);
    game.state.creeps = [];
    game.state.spawners = [];
    const score = game.state.score;
    game.update(0);
    assert.equal(game.state.won, true);
    assert.equal(game.state.score, score + level.lives * content.rules.winScore);
  }
});

test("migration preserves all existing wave health and rewards", () => {
  const cycle = ["walker", "runner", "walker", "flyer", "swarm", "armored", "walker", "splitter", "runner", "boss"];
  for (const [id, growth] of [["standard", 1.115], ["marathon", 1.06]]) {
    for (const [i, wave] of content.waveSets[id].entries()) {
      assert.equal(wave.type, cycle[i % cycle.length]);
      assert.equal(wave.hp, 12 * Math.pow(growth, i) + 4 * (i + 1));
      assert.equal(wave.bountyMultiplier, 2 + Math.floor((i + 1) / 6));
    }
  }
});

test("wave target lists control individual spawns", () => {
  const game = simulation();
  game.newGame("normal");
  for (const n of [1, 2, 15, 16]) {
    const wave = game.waveDef(n);
    game.state.creeps = [];
    for (let i = 0; i < wave.count; i++) game.spawn(wave, i);
    game.state.creeps.forEach((creep, i) => {
      assert.equal(creep.target, wave.targets[i % wave.targets.length]);
      assert.equal(creep.hp, wave.hp * content.enemies[wave.type].hp);
    });
  }
});

test("tower stats, upgrades and sales use shared content", () => {
  const game = simulation();
  for (const [type, def] of content.towers.entries()) {
    game.newGame("uncle_scrooge");
    game.setBuild(type);
    game.tryPlace(5, 5);
    const tower = game.state.towers[0];
    assert.ok(tower);
    for (let level = 0; level < def.dmg.length; level++) {
      const stats = game.towerStats(tower);
      assert.equal(stats.dmg, def.dmg[level]);
      assert.equal(stats.range, def.range[level] * content.board.rangeScale);
      assert.equal(stats.rate, def.rate[level]);
      assert.equal(stats.slow, def.special[level] === "slow" ? 0.5 : 0);
      if (level < def.ups.length) game.upgrade(tower);
    }
    assert.equal(game.sellValue(tower), def.cost + def.ups.reduce((a, b) => a + b, 0));
    game.sell(tower);
    assert.equal(game.state.gold, content.levels.find((l) => l.id === "uncle_scrooge").gold);
    game.sendWave();
    game.setBuild(type);
    game.tryPlace(5, 5);
    const built = game.state.towers[0];
    for (let level = 0; level < def.dmg.length; level++) {
      assert.equal(game.sellValue(built), def.sell[level]);
      if (level < def.ups.length) game.upgrade(built);
    }
  }
});

test("prebuilt towers use CSV refunds once the first wave starts", () => {
  const game = simulation();
  game.newGame("normal");
  game.setBuild(0);
  game.tryPlace(5, 5);
  const tower = game.state.towers[0];
  assert.equal(game.sellValue(tower), 5);
  game.sendWave();
  assert.equal(game.sellValue(tower), 3);
});

test("enemy speed scales with the enlarged vertical grid", () => {
  const game = simulation();
  game.newGame("normal");
  const right = game.makeCreep("flyer", 100, 0, "R", null);
  const bottom = game.makeCreep("flyer", 100, 0, "B", null);
  assert.equal(right.speed, content.enemies.flyer.speed);
  assert.equal(bottom.speed, content.enemies.flyer.speed * 1.1);
  const x = right.x, y = bottom.y;
  game.update(1);
  assert.ok(Math.abs(right.x - x - right.speed) < 0.00001);
  assert.ok(Math.abs(bottom.y - y - bottom.speed) < 0.00001);
});

test("all six towers fire moving projectiles at their compatible targets", () => {
  const game = simulation();
  for (let type = 0; type < content.towers.length; type++) {
    game.newGame("uncle_scrooge");
    game.setBuild(type);
    game.tryPlace(5, 5);
    const tower = game.state.towers[0];
    const def = content.towers[type];
    const creep = game.makeCreep(def.ground ? "walker" : "flyer", 10000, 0, "R", null);
    creep.x = tower.x + 20; creep.y = tower.y;
    assert.equal(game.fire(tower), true);
    assert.ok(game.state.shots[0].speed > 0);
    game.updateShots(1);
    assert.equal(creep.hp, 10000 - def.dmg[0]);
    assert.equal(game.state.shots.length, 0);
  }
});

test("split behavior and rewards come from the enemy definition", () => {
  const game = simulation();
  game.newGame("normal");
  const parent = game.makeCreep("splitter", 100, 8, "R", null);
  const gold = game.state.gold;
  game.hurt(parent, 100);
  const split = content.enemies.splitter.split;
  const children = game.state.creeps.filter((creep) => !creep.dead);
  assert.equal(children.length, split.count);
  for (const child of children) {
    assert.equal(child.type, split.type);
    assert.equal(child.hp, parent.maxHp * split.hp);
    assert.equal(child.bounty, parent.bounty * split.bounty);
    assert.equal(child.target, parent.target);
  }
  assert.equal(game.state.gold, gold + 8);
  assert.equal(game.state.score, 8 * content.rules.killScore);
});

test("new levels and edited waves require no simulation changes", () => {
  const edited = structuredClone(content);
  edited.levels.push({ id: "custom", name: "Custom", hp: 2, lives: 5, gold: 150, gap: 3, waves: 1, waveSet: "custom" });
  edited.enemies.tiny = { ...edited.enemies.runner, name: "Tiny", hp: 0.25, count: 2 };
  edited.waveSets.custom = [{ type: "tiny", hp: 123, bountyMultiplier: 7, targets: ["B", "R", "B"] }];
  const game = simulation(validateContent(edited));
  game.newGame("custom");
  game.sendWave();
  game.update(0);
  assert.equal(game.state.timer, 3);
  assert.equal(game.state.creeps[0].hp, 123 * edited.enemies.tiny.hp * 2);
  assert.equal(game.state.creeps[0].target, "B");
  assert.equal(game.state.creeps[0].bounty, edited.enemies.tiny.bounty * 7);
});

test("new towers can reuse existing projectile behavior", () => {
  const edited = structuredClone(content);
  edited.towers.push({ ...edited.towers[0], id: "training", name: "Training", key: "7", cost: 12, dmg: [7, 14, 28, 56, 112, 224] });
  const game = simulation(validateContent(edited));
  game.newGame("normal");
  game.setBuild(edited.towers.length - 1);
  game.tryPlace(5, 5);
  const tower = game.state.towers[0];
  assert.equal(game.state.gold, 60 - 12);
  assert.equal(game.towerStats(tower).dmg, 7);
});

for (const [name, mutate] of [
  ["version", (d) => { d.version = 2; }],
  ["unknown enemy", (d) => { d.waveSets.standard[0].type = "missing"; }],
  ["unknown wave set", (d) => { d.levels[0].waveSet = "missing"; }],
  ["too many waves", (d) => { d.levels[0].waves = 1000; }],
  ["empty targets", (d) => { d.waveSets.standard[0].targets = []; }],
  ["unknown target", (d) => { d.waveSets.standard[0].targets = ["left"]; }],
  ["invalid health", (d) => { d.waveSets.standard[0].hp = -1; }],
  ["invalid split", (d) => { d.enemies.splitter.split.type = "missing"; }],
  ["duplicate level", (d) => { d.levels.push(d.levels[0]); }],
  ["duplicate tower", (d) => { d.towers.push(d.towers[0]); }],
  ["invalid upgrade array", (d) => { d.towers[0].dmg.pop(); }],
  ["invalid shortcut", (d) => { d.towers[0].key = "10"; }],
  ["invalid firing rate", (d) => { d.towers[0].rate[0] = 0; }],
  ["invalid board lane", (d) => { d.board.openRows.push(d.board.rows); }],
]) {
  test(`invalid content fails explicitly: ${name}`, () => {
    const edited = structuredClone(content);
    mutate(edited);
    assert.throws(() => validateContent(edited), /game content/);
  });
}

test("browser build embeds exact shared content and wires every localized page", async () => {
  execFileSync(process.execPath, ["build.mjs"], { cwd: new URL(".", import.meta.url) });
  const built = await import("./dist/content.js");
  assert.deepEqual(built.CONTENT, content);
  assert.equal(existsSync(new URL("./dist/tower-atlas.js", import.meta.url)), false);
  assert.equal(existsSync(new URL("./dist/enemy-atlas.js", import.meta.url)), false);
  for (const name of FX_NAMES) {
    assert.deepEqual(readFileSync(new URL(`./dist/assets/fx/${name}.mp3`, import.meta.url)),
      readFileSync(new URL(`../assets/fx/${name}.mp3`, import.meta.url)));
  }
  assert.equal(readFileSync(new URL("./dist/audio.js", import.meta.url), "utf8"),
    readFileSync(new URL("./src/audio.js", import.meta.url), "utf8"));
  for (const image of new Set([...content.towers, ...Object.values(content.enemies)].map((d) => d.sprite.image))) {
    assert.deepEqual(readFileSync(new URL(`./dist/${image}`, import.meta.url)),
      readFileSync(new URL(`../${image}`, import.meta.url)));
  }
  for (const prefix of ["", ...["en", "es", "de", "fr", "id"].map((lang) => `${lang}/games/preecep/desktop-tower-defense/`)]) {
    const html = readFileSync(new URL(`./dist/${prefix}index.html`, import.meta.url), "utf8");
    assert.ok(!html.includes("{{"));
    assert.ok(html.includes(prefix ? '../../../../game.js' : './game.js'));
    assert.ok(html.includes(`width="${content.board.cell * content.board.cols}" height="${content.board.cell * content.board.rows}"`));
    for (const id of ["b-mute", "b-options", "options", "fx-volume"]) assert.ok(html.includes(`id="${id}"`));
  }
});

test("each tower plays its FX once per salvo, never without a target", () => {
  for (const [type, tower] of content.towers.entries()) {
    const game = simulation();
    game.newGame("normal");
    const placed = { type, level: 0, x: 100, y: 100 };
    assert.equal(game.fire(placed), false);
    assert.deepEqual(game.audio.played, []);
    for (let i = 0; i < 3; i++) {
      const creep = game.makeCreep(tower.ground ? "walker" : "flyer", 100, 1, "R", null);
      creep.x = 100; creep.y = 100;
    }
    assert.equal(game.fire(placed), true);
    assert.deepEqual(game.audio.played, [FX_NAMES[type]]);
    game.newGame("normal");
    assert.deepEqual(game.audio.played, [FX_NAMES[type]]);
  }
});

test("losing lives plays HIT, including a boss and the final life", () => {
  const game = simulation();
  game.newGame("normal");
  const boss = game.makeCreep("boss", 100, 1, "R", null);
  game.leak(boss);
  assert.equal(game.state.lives, content.levels.find((l) => l.id === "normal").lives - 2);
  game.state.lives = 1;
  game.leak(game.makeCreep("walker", 100, 1, "R", null));
  assert.equal(game.state.over, true);
  assert.deepEqual(game.audio.played, ["HIT", "HIT"]);
});

test("a multi-target salvo plays one FX for all projectiles", () => {
  const edited = structuredClone(content);
  edited.towers[0].multi = 3;
  const game = simulation(edited);
  game.newGame("normal");
  for (let i = 0; i < 3; i++) {
    const creep = game.makeCreep("walker", 100, 1, "R", null);
    creep.x = 100; creep.y = 100;
  }
  assert.equal(game.fire({ type: 0, level: 0, x: 100, y: 100 }), true);
  assert.equal(game.state.shots.length, 3);
  assert.deepEqual(game.audio.played, ["01"]);
});
