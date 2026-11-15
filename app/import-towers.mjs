import { readFileSync, writeFileSync } from "node:fs";
import { validateContent } from "./content.mjs";

const path = new URL("../game-content.json", import.meta.url);
const data = JSON.parse(readFileSync(path, "utf8"));
const lines = readFileSync(new URL("../towers.csv", import.meta.url), "utf8").trim().split(/\r?\n/);
if (lines[0] !== "tower,level,cost,damage,range,speed,sell,special,special factor,earth,fly")
  throw new Error("towers.csv: unexpected column headers");
const rows = lines.slice(1).map((line) => line.split(","));
if (rows.length !== 36) throw new Error("towers.csv: expected six towers with six levels");
if (rows.some((r) => r.length !== 11 || !["0", "1"].includes(r[9]) || !["0", "1"].includes(r[10])))
  throw new Error("towers.csv: invalid columns or target flags");
for (const [i, tower] of data.towers.entries()) {
  const levels = rows.filter((r) => Number(r[0]) === i + 1);
  if (levels.length !== 6 || levels.some((r, j) => Number(r[1]) !== j + 1))
    throw new Error(`towers.csv: invalid levels for tower ${i + 1}`);
  tower.cost = Number(levels[0][2]);
  tower.ups = levels.slice(1).map((r) => Number(r[2]));
  tower.dmg = levels.map((r) => Number(r[3]));
  tower.range = levels.map((r) => Number(r[4]));
  tower.rate = levels.map((r) => 1 / Number(r[5]));
  tower.sell = levels.map((r) => Number(r[6]));
  tower.special = levels.map((r) => r[7] || "none");
  tower.factor = levels.map((r) => Number(r[8] || 0));
  tower.ground = levels[0][9] === "1";
  tower.air = levels[0][10] === "1";
  if (levels.some((r) => (r[9] === "1") !== tower.ground || (r[10] === "1") !== tower.air))
    throw new Error("towers.csv: target flags must match across levels");
  tower.multi = 1;
  delete tower.splash;
  delete tower.slow;
  delete tower.stun;
  tower.shot = ["pellet", "dart", "shell", "missile", "ice", "pulse"][i];
  if (tower.speed === 0) tower.speed = 320;
  tower.desc = [
    "Touche le sol et les airs. Portée étendue au niveau 6.",
    "Tir très rapide. Touche le sol et les airs.",
    "Touche une cible et éclabousse les ennemis terrestres proches.",
    "Tir rapide contre les ennemis aériens.",
    "Peut diviser par deux la vitesse des ennemis proches pendant deux secondes.",
    "Peut immobiliser les ennemis terrestres proches pendant deux secondes.",
  ][i];
}
delete data.rules.rangeGrowth;
delete data.rules.rateReduction;
delete data.rules.sellRefund;
data.rules.stunDuration = 2;
validateContent(data);
writeFileSync(path, JSON.stringify(data, null, 2) + "\n");
