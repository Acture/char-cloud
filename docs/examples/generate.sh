#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

BIN="target/debug/char-cloud"
if [[ ! -x "$BIN" ]]; then
	cargo build --bin char-cloud
fi

"$BIN" --text RUST --word-file docs/examples/words-tech.txt --canvas-size 1600,900 --canvas-margin 14 --word-size-range 10,38 --algorithm fast-grid --palette auto --palette-base '#0EA5E9' --palette-size 7 --rotations 0,90 --seed 42 --ratio 0.87 --max-tries 14000 --font fonts/Roboto-Regular.ttf --no-progress --output docs/examples/example-fast-grid.svg
"$BIN" --text AI --word-file docs/examples/words-tech.txt --canvas-size 1400,840 --canvas-margin 12 --word-size-range 10,36 --algorithm fast-grid --palette complementary --palette-base '#8B5CF6' --palette-size 7 --rotations 0,90 --seed 2026 --ratio 0.86 --max-tries 12000 --font fonts/Roboto-Regular.ttf --no-progress --output docs/examples/example-complementary.svg
"$BIN" --text DATA --word-file docs/examples/words-tech.txt --canvas-size 1400,840 --canvas-margin 12 --word-size-range 10,36 --algorithm fast-grid --palette analogous --palette-base '#10B981' --palette-size 7 --rotations 0,90 --seed 31415 --ratio 0.86 --max-tries 12000 --font fonts/Roboto-Regular.ttf --no-progress --output docs/examples/example-analogous.svg
"$BIN" --text CODE --word-file docs/examples/words-tech.txt --canvas-size 1400,840 --canvas-margin 12 --word-size-range 10,36 --algorithm fast-grid --palette vibrant --palette-size 8 --rotations 0,90 --seed 9001 --ratio 0.86 --max-tries 12000 --font fonts/Roboto-Regular.ttf --no-progress --output docs/examples/example-vibrant.svg
