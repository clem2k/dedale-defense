#!/usr/bin/env bash
# Build natif Linux x86_64 de Dédale Défense.
# Usage : UNITY_PATH=~/Unity/Hub/Editor/<version>/Editor/Unity ./unity/build-linux.sh [--il2cpp]
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT="$HERE/DedaleDefense"
OUT="$HERE/Build/Linux/DedaleDefense.x86_64"
LOG="$HERE/Build/build.log"

if [[ -z "${UNITY_PATH:-}" || ! -x "$UNITY_PATH" ]]; then
  echo "UNITY_PATH doit pointer vers l'exécutable Unity (voir dev.md §6)." >&2
  exit 2
fi

EXTRA=()
[[ "${1:-}" == "--il2cpp" ]] && EXTRA+=(-il2cpp)

mkdir -p "$(dirname "$LOG")"
"$UNITY_PATH" -batchmode -nographics -quit \
  -projectPath "$PROJECT" \
  -buildTarget Linux64 \
  -executeMethod BuildScript.BuildLinux \
  -output "$OUT" "${EXTRA[@]}" \
  -logFile "$LOG" || { echo "Échec du build, voir $LOG" >&2; tail -40 "$LOG" >&2; exit 1; }

echo "OK : $OUT"
