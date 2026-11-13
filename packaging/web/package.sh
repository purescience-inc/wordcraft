#!/usr/bin/env bash
# Build the browser version and zip it:  $DIST/wordcraft-web-<version>.zip
#
# Usage: packaging/web/package.sh [--skip-build]
#
# Needs: trunk (brew install trunk / cargo install trunk --locked) and the wasm32-unknown-unknown
# target. The zip holds a self-contained static site in wordcraft-web-<version>/ that works
# from any URL path and inside an <iframe>. Hosting notes: packaging/web/README.md.
set -euo pipefail
# shellcheck source=../env.sh
. "$(dirname "${BASH_SOURCE[0]}")/../env.sh"
HERE="$ROOT/packaging/web"

if [[ "${1:-}" != "--skip-build" ]]; then
  command -v trunk >/dev/null || { echo "error: trunk not found (cargo install trunk --locked)" >&2; exit 1; }
  # CRAFT_WEB_FEATURES (e.g. "pure") turns on cargo features of the web shell; unset or empty
  # builds the default app.
  FEATURES=()
  [[ -n "${CRAFT_WEB_FEATURES:-}" ]] && FEATURES=(--features "$CRAFT_WEB_FEATURES")
  (cd "$ROOT/apps/wordcraft-web" && trunk build --release ${FEATURES[@]+"${FEATURES[@]}"})
fi

SITE="$ROOT/dist/web"
[[ -f "$SITE/index.html" ]] || { echo "error: $SITE/index.html missing; run without --skip-build" >&2; exit 1; }
# Paths must be relative so the site works under any prefix (public_url = "./" in Trunk.toml).
if grep -Eq '(src|href)="/[^/]' "$SITE/index.html"; then
  echo "error: $SITE/index.html has root-absolute URLs; it would break when served from a sub-path" >&2
  exit 1
fi

NAME="wordcraft-web-$VERSION"
WORK="$CARGO_TARGET_DIR/web-package"
rm -rf "$WORK"
mkdir -p "$WORK/$NAME"
cp -R "$SITE/." "$WORK/$NAME/"
# Sample server configs (MIME type, caching, compression); harmless where unused.
cp "$HERE/_headers" "$HERE/.htaccess" "$WORK/$NAME/"
cp "$HERE/README.md" "$WORK/$NAME/HOSTING.md"
copy_docs "$WORK/$NAME"
# The Pure edition's page names itself while the app loads (the packaged copy only; the source
# page is upstream's).
if [[ " ${CRAFT_WEB_FEATURES:-} " == *" pure "* ]]; then
  sed -i.bak -e 's|<title>WordCraft</title>|<title>PureWriter</title>|' -e 's|Loading WordCraft|Loading PureWriter|' "$WORK/$NAME/index.html"
  rm -f "$WORK/$NAME/index.html.bak"
fi
rm -f "$DIST/$NAME.zip"
(cd "$WORK" && zip -qr9 "$DIST/$NAME.zip" "$NAME")
echo "wrote $DIST/$NAME.zip"
ls -lh "$DIST/$NAME.zip"
