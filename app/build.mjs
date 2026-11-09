// Copies the game into dist/ and serves the same game page at every path the original had.
import { mkdirSync, rmSync, readFileSync, writeFileSync, copyFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(import.meta.url));
const src = join(root, "src");
const dist = join(root, "dist");

rmSync(dist, { recursive: true, force: true });
mkdirSync(dist, { recursive: true });

for (const f of ["style.css", "game.js"]) copyFileSync(join(src, f), join(dist, f));

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
    html.replaceAll("{{BASE}}", prefix).replace('lang="fr"', `lang="${lang}"`)
  );
}
console.log(`Built ${pages.length} pages into dist/`);
