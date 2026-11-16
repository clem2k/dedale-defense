#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
profile="${1:-debug}"
case "$profile" in
    debug) flags=() ;;
    release) flags=(--release) ;;
    *) echo "Usage: $0 [debug|release]" >&2; exit 2 ;;
esac

command -v cargo-quad-apk >/dev/null || {
    echo "Missing cargo-quad-apk; install the pinned revision in apps/android/README.md." >&2
    exit 1
}
: "${ANDROID_HOME:?Set ANDROID_HOME to the Android SDK directory}"
: "${NDK_HOME:?Set NDK_HOME to the Android NDK directory}"
export PATH="$ANDROID_HOME/build-tools/35.0.0:$PATH"
# The packager sets CC for Android, but MP3 decoding also builds C code for the host.
export HOST_CC="${HOST_CC:-cc}" HOST_CXX="${HOST_CXX:-c++}" HOST_AR="${HOST_AR:-ar}"

cargo quad-apk build -p dedale-android --locked "${flags[@]}"
apk="target/android-artifacts/$profile/apk/dedale-android.apk"
test -s "$apk"
apksigner verify "$apk"
printf 'APK: %s/%s\n' "$PWD" "$apk"
