import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";
import { loadContent, validateContent } from "./content.mjs";

const content = loadContent();
const source = readFileSync(new URL("./src/game.js", import.meta.url), "utf8");

function simulation(data = content) {
  const logic = source.slice(0, source.indexOf("// ---------- Drawing ----------"))
    .replace('import { CONTENT } from "./content.js";', "");
  const setBuild = source.slice(source.indexOf("function setBuild("), source.indexOf("function togglePause("));
  return vm.runInNewContext(`${logic}\n${setBuild}
    ({ get state() { return S; }, newGame, waveDef, sendWave, update,
       setBuild, tryPlace, towerStats, sellValue, upgrade, sell, hurt, spawn, makeCreep })`, {
    CONTENT: data,
    canvas: { style: {} },
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
      assert.equal(stats.range, def.range * (1 + content.rules.rangeGrowth * level));
      assert.equal(stats.rate, def.rate * (1 - content.rules.rateReduction * level));
      assert.equal(stats.slow, def.slow[level]);
      if (level < def.ups.length) game.upgrade(tower);
    }
    assert.equal(game.sellValue(tower), def.cost + def.ups.reduce((a, b) => a + b, 0));
    game.sell(tower);
    assert.equal(game.state.gold, content.levels.find((l) => l.id === "uncle_scrooge").gold);
    game.sendWave();
    game.setBuild(type);
    game.tryPlace(5, 5);
    assert.equal(game.sellValue(game.state.towers[0]), Math.floor(def.cost * content.rules.sellRefund));
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
  edited.towers.push({ ...edited.towers[0], id: "training", name: "Training", key: "7", cost: 12, dmg: [7, 14, 28, 56, 112] });
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
  ["invalid firing rate", (d) => { d.rules.rateReduction = 1; }],
  ["invalid board lane", (d) => { d.board.openRows.push(d.board.rows); }],
]) {
  test(`invalid content fails explicitly: ${name}`, () => {
    const edited = structuredClone(content);
    mutate(edited);
    assert.throws(() => validateContent(edited), /game-content.json:/);
  });
}

test("browser build embeds exact shared content and wires every localized page", async () => {
  execFileSync(process.execPath, ["build.mjs"], { cwd: new URL(".", import.meta.url) });
  const built = await import("./dist/content.js");
  assert.deepEqual(built.CONTENT, content);
  for (const prefix of ["", ...["en", "es", "de", "fr", "id"].map((lang) => `${lang}/games/preecep/desktop-tower-defense/`)]) {
    const html = readFileSync(new URL(`./dist/${prefix}index.html`, import.meta.url), "utf8");
    assert.ok(!html.includes("{{"));
    assert.ok(html.includes(prefix ? '../../../../game.js' : './game.js'));
    assert.ok(html.includes(`width="${content.board.cell * content.board.cols}" height="${content.board.cell * content.board.rows}"`));
  }
});
