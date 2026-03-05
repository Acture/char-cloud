# Char Cloud

[![Crates.io](https://img.shields.io/crates/v/char-cloud)](https://crates.io/crates/char-cloud)
[![Release Build](https://github.com/Acture/char-cloud/actions/workflows/release.yml/badge.svg)](https://github.com/Acture/char-cloud/actions/workflows/release.yml)
[![License](https://img.shields.io/crates/l/char-cloud)](LICENSE)

Char Cloud 是一个 **Rust CLI + library**，用于把词云约束在目标文字轮廓内部并输出 SVG。  
Char Cloud is a **Rust CLI + reusable library** for generating shape-constrained SVG word clouds.

## 核心能力 | Highlights

- 文本轮廓遮罩（自动计算形状字号）
- 5 种布局算法：`fast-grid` / `mcts` / `simulated-annealing` / `spiral-greedy` / `random-baseline`
- 支持随机种子复现结果（`--seed`）
- 支持词权重（`--word-file` / `--weights-file`）
- 支持旋转集合（`--rotations 0,90`）
- 支持自动调色板（互补/三色组/类比/单色/预设）
- 支持全局与项目级配置文件（TOML）
- 可选输出调试遮罩图（`--debug-mask-out`）

## 安装 | Installation

```bash
cargo install char-cloud
```

启用内置 Noto Sans SC（可选）：

```bash
cargo install char-cloud --features embedded_fonts
```

## CLI 快速开始 | Quick Start

```bash
char-cloud \
  --text "RUST" \
  --words "cloud,speed,layout,mask,svg,grid" \
  --canvas-size 1400,800 \
  --algorithm fast-grid \
  --seed 42 \
  --output output.svg
```

通过文本文件输入词与权重：

```text
# words.txt
rust,3
cloud,2
layout,2
mask
svg
```

```bash
char-cloud \
  --text "AI" \
  --word-file words.txt \
  --palette complementary \
  --palette-base "#4F46E5" \
  --palette-size 7 \
  --rotations 0,90 \
  --algorithm spiral-greedy \
  --output ai.svg
```

查看完整参数：

```bash
char-cloud --help
```

## 算法选择建议 | Algorithm Guide

| Algorithm | Speed | Fill quality | Recommended for |
|---|---:|---:|---|
| `fast-grid` | High | High | 默认生产使用，平衡速度和质量 |
| `mcts` | Medium-Low | High | 想用搜索策略换更高质量结果 |
| `simulated-annealing` | Medium-Low | Medium-High | 想要随机探索和局部最优跳出能力 |
| `spiral-greedy` | Medium | Medium-High | 希望视觉更集中、结构更稳定 |
| `random-baseline` | Low | Medium | 对照基线、回归比较 |

## 调参建议 | Tuning

- `--ratio`：目标填充率（0.0~1.0），常用 `0.75~0.92`
- `--max-tries`：尝试次数上限，图越大建议越高
- `--word-size-range`：大跨度会增强层次，但更难填满边缘
- `--seed`：固定后可复现实验和回归
- `--algorithm`：优先 `fast-grid`；高质量搜索可试 `mcts`；随机优化可试 `simulated-annealing`
- `--palette`：可选 `auto/complementary/triadic/analogous/monochrome/pastel/earth/vibrant`
- `--colors` 与 `--palette` 同时存在时，优先使用 `--colors`
- `--font`：显式指定 `.ttf/.otf` 字体文件
- `--choose-system-font`：当内置字体不可用时，交互选择系统字体

## 字体与许可 | Fonts & License

- 内置字体 feature：`embedded_fonts`（默认关闭）
- 内置字体：Noto Sans SC
- 字体许可：SIL Open Font License 1.1
- 许可文本：`fonts/OFL-NotoSansSC.txt`

## 全局配置 | Global Config

支持以下配置加载顺序（后者覆盖前者）：

1. `~/.config/char-cloud/config.toml`（或 `$XDG_CONFIG_HOME/char-cloud/config.toml`）
2. 当前目录 `.char-cloud.toml`
3. `--config <path>` 指定文件
4. CLI 参数（最高优先级）

示例：

```toml
canvas_size = [1600, 900]
canvas_margin = 12
word_size_range = [10, 30]
algorithm = "fast-grid"
palette = "analogous"
palette_base = "#0EA5E9"
palette_size = 6
ratio = 0.85
max_tries = 12000
rotations = [0, 90]
```

更多详见：

- [Architecture](docs/architecture.md)
- [Library API](docs/library-api.md)
- [Algorithms](docs/algorithms.md)
- [Tuning](docs/tuning.md)
- [Migration v0.2](docs/migration-v0.2.md)

## Library 使用示例 | Library Example

```rust
use char_cloud::{
    generate, AlgorithmKind, CanvasConfig, CloudRequest, FontSizeSpec, RenderOptions,
    ShapeConfig, StyleConfig, WordEntry, load_font_from_file,
};
use std::{path::Path, sync::Arc};

let font = load_font_from_file(Path::new("fonts/NotoSansSC-Regular.ttf"))?;
let request = CloudRequest {
    canvas: CanvasConfig { width: 1200, height: 700, margin: 12 },
    shape: ShapeConfig { text: "DATA".to_string(), font_size: FontSizeSpec::AutoFit },
    words: vec![WordEntry::new("rust", 2.0), WordEntry::new("svg", 1.0)],
    style: StyleConfig::default(),
    algorithm: AlgorithmKind::FastGrid,
    ratio_threshold: 0.85,
    max_try_count: 10_000,
    seed: Some(7),
    font: Arc::new(font),
    render: RenderOptions::default(),
};

let result = generate(request)?;
std::fs::write("cloud.svg", result.svg)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

启用 `--features embedded_fonts` 构建后，可以直接使用 `load_default_embedded_font()`。

## License

AGPL-3.0. See [LICENSE](LICENSE).
