use std::process::Command;
use tempfile::tempdir;

#[test]
fn cli_generates_svg_file() {
    let dir = tempdir().expect("tempdir should be created");
    let output = dir.path().join("cloud.svg");

    let status = Command::new(env!("CARGO_BIN_EXE_char-cloud"))
        .args([
            "--text",
            "RUST",
            "--words",
            "rust,cloud,layout,svg,mask",
            "--seed",
            "42",
            "--algorithm",
            "fast-grid",
            "--no-progress",
            "--output",
        ])
        .arg(&output)
        .status()
        .expect("process should run");

    assert!(status.success());

    let content = std::fs::read_to_string(&output).expect("svg should be written");
    assert!(content.contains("<svg"));
}

#[test]
fn cli_returns_invalid_config_exit_code_when_words_missing() {
    let dir = tempdir().expect("tempdir should be created");
    let output = dir.path().join("missing.svg");

    let result = Command::new(env!("CARGO_BIN_EXE_char-cloud"))
        .args(["--text", "RUST", "--no-progress", "--output"])
        .arg(&output)
        .output()
        .expect("process should run");

    assert_eq!(result.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("no words provided"));
}

#[test]
fn cli_can_write_debug_mask() {
    let dir = tempdir().expect("tempdir should be created");
    let output = dir.path().join("cloud.svg");
    let mask = dir.path().join("mask.png");

    let status = Command::new(env!("CARGO_BIN_EXE_char-cloud"))
        .args([
            "--text",
            "RUST",
            "--words",
            "rust,cloud,layout",
            "--canvas-size",
            "420,240",
            "--algorithm",
            "random-baseline",
            "--seed",
            "123",
            "--ratio",
            "0.2",
            "--max-tries",
            "200",
            "--debug-mask-out",
        ])
        .arg(&mask)
        .args(["--no-progress", "--output"])
        .arg(&output)
        .status()
        .expect("process should run");

    assert!(status.success());
    assert!(mask.exists());
}
