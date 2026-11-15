// Copies the game into dist/ and serves the same game page at every path the original had.
import { mkdirSync, rmSync, readFileSync, writeFileSync, copyFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { loadContent } from "./content.mjs";

const root = dirname(fileURLToPath(import.meta.url));
const src = join(root, "src");
const dist = join(root, "dist");
const content = loadContent();
const keys = content.towers.map((tower) => tower.key);
const towerKeys = keys.length > 1 && keys.every((key, i) => Number(key) === i + 1)
  ? `1-${keys.length}` : keys.join(", ");

rmSync(dist, { recursive: true, force: true });
mkdirSync(dist, { recursive: true });

for (const f of ["style.css", "game.js"]) copyFileSync(join(src, f), join(dist, f));
writeFileSync(join(dist, "content.js"), `export const CONTENT = ${JSON.stringify(content)};\n`);

const html = readFileSync(join(src, "index.html"), "utf8");
const pages = [
  "",
  "en/games/preecep/desktop-tower-defense",
  "es/games/preecep/desktop-tower-defense",
  "de/games/preecep/desktop-tower-defense",
  "fr/games/preecep/desktop-tower-defense",
  "id/games/preecep/desktop-tower-defense",
];
for (const p of pages) {
  const depth = p ? p.split("/").length : 0;
  const prefix = depth ? "../".repeat(depth) : "./";
  const dir = join(dist, p);
  mkdirSync(dir, { recursive: true });
  const lang = p ? p.split("/")[0] : "fr";
  writeFileSync(
    join(dir, "index.html"),
    html.replaceAll("{{BASE}}", prefix)
      .replaceAll("{{BOARD_WIDTH}}", String(content.board.cell * content.board.cols))
      .replaceAll("{{BOARD_HEIGHT}}", String(content.board.cell * content.board.rows))
      .replaceAll("{{TOWER_KEYS}}", towerKeys)
      .replace('lang="fr"', `lang="${lang}"`)
  );
}
console.log(`Built ${pages.length} pages into dist/`);
