# Migration Guide: v0.1.x -> v0.2

## Breaking CLI Renames

| v0.1.x | v0.2 |
|---|---|
| `--canva-size` | `--canvas-size` |
| `--canva-margin` | `--canvas-margin` |
| `--word_size_range` | `--word-size-range` |

## New CLI Options

- `--algorithm` (`fast-grid`, `spiral-greedy`, `random-baseline`)
  - plus `mcts`, `simulated-annealing`
- `--seed`
- `--word-file`
- `--weights-file`
- `--rotations`
- `--palette`
- `--palette-base`
- `--palette-size`
- `--config`
- `--debug-mask-out`
- `--no-progress`
- `--choose-system-font`

## Behavior Changes

- No intermediate mask file is written by default.
- Library API is now first-class (`src/lib.rs`, `generate`).
- Exit codes are now typed by error category.
- Added layered config loading (`~/.config/char-cloud/config.toml` -> `.char-cloud.toml` -> `--config` -> CLI args).
- Embedded Noto Sans SC moved behind `embedded_fonts` feature gate (no longer enabled by default).
- CLI font resolution order is now: `--font` > embedded font (`embedded_fonts`) > auto-discovered system font.

## Library Migration

v0.1.x had no stable public library API. In v0.2 use:

- `CloudRequest`
- `generate`
- `CloudResult`

for programmatic integration.
