## [0.2.0] - 2026-03-06

### 🚀 Features

- Refactor project into a reusable library + CLI shell architecture
- Add pluggable layout strategies with `fast-grid`, `spiral-greedy`, `mcts`, and `simulated-annealing`
- Add weighted words, configurable rotations, and deterministic generation via seed
- Add palette strategies (`auto`, `complementary`, `triadic`, `analogous`, `monochrome`, presets)
- Add feature-gated embedded font support with system font fallback

### 🐛 Bug Fixes

- Correct release workflow tag trigger pattern for v-prefixed tags
- Improve release workflow compatibility with Git LFS font assets

### 📚 Documentation

- Rewrite README in concise English style and add generated SVG gallery examples
- Add architecture, API, algorithms, tuning, and migration docs for v0.2
- Add embedded font license documentation for Noto Sans SC

### 🧪 Testing

- Add snapshot regression tests, config precedence tests, and performance regression checks
- Expand API/CLI integration coverage for new configuration and algorithm options

### ⚙️ CI

- Add dedicated CI and performance regression workflows
- Harden release pipeline checks (`fmt`, `clippy`, `tests`) before publishing assets

## [0.1.2-test] - 2025-08-08

### 🚀 Features

- *(ci)* Add support for Git ref input in release workflow

### 🐛 Bug Fixes

- *(ci)* Correct draft flag logic in release workflow
- *(ci)* Update default Git ref in release workflow

### 🚜 Refactor

- *(ci)* Simplify and streamline release workflow
- *(ci)* Remove manual dispatch inputs from release workflow

### 📚 Documentation

- *(readme)* Update README with English translation and improved formatting
## [0.1.2] - 2025-06-11

### 🚀 Features

- *(draw)* Add progress bar for text filling process
## [0.1.1] - 2025-06-11

### 🚀 Features

- *(flakes)* Add Nix Flake support for project development and builds
- *(ci)* Add GitHub Actions workflow for release builds
- *(main, embedded_fonts)* Add conditional support for embedded fonts
- *(ci)* Add manual trigger to release workflow
- *(ci, mask)* Enhance release workflow and fix module import
- *(flakes, ci)* Improve cross-platform builds and dev environment
- *(ci)* Enhance release workflow with improved platform support and artifact handling
- *(ci)* Refine release workflow with manual draft option and enhanced artifact handling
- *(ci)* Update release workflow with formal release option

### 🐛 Bug Fixes

- *(main)* Improve logging levels and clean up formatting
- *(utils, args, draw, mask, main)* Clean up formatting and optimize code consistency
- *(ci)* Update release workflow for Nix build command
- *(mask, ci)* Resolve module import and simplify workflow steps
- *(ci)* Remove unsupported aarch64-windows target and redundant steps
- *(ci)* Add `contents: write` permission for release workflow
- *(ci)* Simplify checksum generation in release workflow
- *(ci)* Remove redundant `make_latest` option in release workflow
- *(cargo)* Bump version to 0.1.1
- *(ci)* Enable Git LFS support in release workflow

### 📚 Documentation

- *(readme)* Update badge to display release build status

### ⚙️ Miscellaneous Tasks

- *(ci)* Update GitHub Actions to latest versions
## [1.0.0] - 2025-06-10

### 🚀 Features

- Initialize project with basic setup
- *(mask)* Add text rendering and mask generation utilities
- *(draw)* Add text drawing utilities and integrate with mask generation
- *(draw)* Improve text placement logic and introduce dynamic font sizing
- *(fonts)* Add NotoSansSC-Regular font file
- *(repo)* Add .gitattributes for LFS font file management
- *(draw)* Add support for configurable text colors
- *(config)* Add default values for new text and canvas configurations
- *(cli)* Add command-line interface for canvas and drawing configuration
- *(embedded_fonts)* Centralize font data management in a new module
- *(cli)* Enhance CLI with new options and improve canvas configuration

### 🐛 Bug Fixes

- *(draw)* Update font loading path and adjust editorconfig
- *(draw, mask)* Update font handling and improve configuration consistency

### 📚 Documentation

- *(readme)* Add project description, features, usage, and license details

### ⚙️ Miscellaneous Tasks

- *(gitignore)* Update gitignore for SVG and PNG assets
- *(gitignore)* Remove `python` from ignore list
- *(cargo)* Update project metadata in Cargo.toml
