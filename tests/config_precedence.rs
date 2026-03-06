use std::process::Command;
use tempfile::tempdir;

fn test_font_path() -> std::path::PathBuf {
	std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fonts/NotoSansSC-Regular.ttf")
}

#[test]
fn explicit_config_overrides_user_and_project() {
	let temp = tempdir().expect("tempdir should be created");
	let workspace = temp.path().join("workspace");
	let xdg = temp.path().join("xdg");
	std::fs::create_dir_all(&workspace).expect("workspace dir should be created");
	std::fs::create_dir_all(xdg.join("glyphweave")).expect("xdg config dir should be created");

	std::fs::write(
		xdg.join("glyphweave").join("config.toml"),
		"colors=[\"#111111\"]\nratio=0.1\nmax_tries=60\nalgorithm=\"fast-grid\"\n",
	)
	.expect("user config should be written");

	std::fs::write(
		workspace.join(".glyphweave.toml"),
		"colors=[\"#222222\"]\nratio=0.1\nmax_tries=60\nalgorithm=\"fast-grid\"\n",
	)
	.expect("project config should be written");

	std::fs::write(
		workspace.join("explicit.toml"),
		"colors=[\"#333333\"]\nratio=0.1\nmax_tries=60\nalgorithm=\"fast-grid\"\n",
	)
	.expect("explicit config should be written");

	let output = workspace.join("out.svg");
	let font = test_font_path();

	let status = Command::new(env!("CARGO_BIN_EXE_glyphweave"))
		.current_dir(&workspace)
		.env("XDG_CONFIG_HOME", &xdg)
		.args([
			"--config",
			"explicit.toml",
			"--text",
			"RUST",
			"--words",
			"rust,cloud,layout,mask",
			"--font",
		])
		.arg(&font)
		.args(["--seed", "42", "--no-progress", "--output"])
		.arg(&output)
		.status()
		.expect("process should run");

	assert!(status.success());

	let svg = std::fs::read_to_string(&output).expect("output svg should be readable");
	assert!(svg.contains("#333333"));
	assert!(!svg.contains("#222222"));
	assert!(!svg.contains("#111111"));
}

#[test]
fn cli_overrides_all_config_layers() {
	let temp = tempdir().expect("tempdir should be created");
	let workspace = temp.path().join("workspace");
	let xdg = temp.path().join("xdg");
	std::fs::create_dir_all(&workspace).expect("workspace dir should be created");
	std::fs::create_dir_all(xdg.join("glyphweave")).expect("xdg config dir should be created");

	std::fs::write(
		xdg.join("glyphweave").join("config.toml"),
		"colors=[\"#111111\"]\nratio=0.1\nmax_tries=60\nalgorithm=\"fast-grid\"\n",
	)
	.expect("user config should be written");

	std::fs::write(
		workspace.join(".glyphweave.toml"),
		"colors=[\"#222222\"]\nratio=0.1\nmax_tries=60\nalgorithm=\"fast-grid\"\n",
	)
	.expect("project config should be written");

	std::fs::write(
		workspace.join("explicit.toml"),
		"colors=[\"#333333\"]\nratio=0.1\nmax_tries=60\nalgorithm=\"fast-grid\"\n",
	)
	.expect("explicit config should be written");

	let output = workspace.join("out.svg");
	let font = test_font_path();

	let status = Command::new(env!("CARGO_BIN_EXE_glyphweave"))
		.current_dir(&workspace)
		.env("XDG_CONFIG_HOME", &xdg)
		.args([
			"--config",
			"explicit.toml",
			"--text",
			"RUST",
			"--words",
			"rust,cloud,layout,mask",
			"--colors",
			"#444444",
			"--font",
		])
		.arg(&font)
		.args(["--seed", "42", "--no-progress", "--output"])
		.arg(&output)
		.status()
		.expect("process should run");

	assert!(status.success());

	let svg = std::fs::read_to_string(&output).expect("output svg should be readable");
	assert!(svg.contains("#444444"));
	assert!(!svg.contains("#333333"));
	assert!(!svg.contains("#222222"));
	assert!(!svg.contains("#111111"));
}
