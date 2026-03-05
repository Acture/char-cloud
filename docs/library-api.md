# Library API

## Public Types

- `CloudRequest`: input configuration
- `CloudResult`: output SVG + placements + stats
- `AlgorithmKind`: `FastGrid` / `SpiralGreedy` / `RandomBaseline`
- `CanvasConfig`, `ShapeConfig`, `StyleConfig`, `WordEntry`, `RenderOptions`

## Entry Point

```rust
pub fn generate(request: CloudRequest) -> Result<CloudResult, CharCloudError>
```

## Font Loading Helpers

```rust
pub fn load_font_from_file<P: AsRef<Path>>(path: P) -> Result<Font, CharCloudError>
pub fn load_default_embedded_font() -> Result<Font, CharCloudError>
```

## Minimal Example

```rust
use char_cloud::*;
use std::sync::Arc;

let font = load_default_embedded_font()?;
let request = CloudRequest {
    canvas: CanvasConfig::default(),
    shape: ShapeConfig { text: "HELLO".into(), font_size: FontSizeSpec::AutoFit },
    words: vec![WordEntry::new("rust", 2.0), WordEntry::new("svg", 1.0)],
    style: StyleConfig::default(),
    algorithm: AlgorithmKind::FastGrid,
    ratio_threshold: 0.85,
    max_try_count: 10_000,
    seed: Some(42),
    font: Arc::new(font),
    render: RenderOptions::default(),
};

let result = generate(request)?;
std::fs::write("output.svg", result.svg)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Determinism

Set `seed` to a fixed value to make layout output reproducible for snapshots and regression tests.
