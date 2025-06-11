{
	description = "Char Cloud - 文字云生成工具";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				overlays = [ (import rust-overlay) ];
				pkgs = import nixpkgs {
					inherit system overlays;
				};

				rustToolchain = pkgs.rust-bin.stable.latest.default.override {
					extensions = [ "rust-src" "rust-analysis" ];
				};

				nativeBuildInputs = with pkgs; [
					rustToolchain
					pkg-config
				];

				buildInputs = with pkgs; [
					# 添加其他系统依赖
				];

			in
			{
				devShells.default = pkgs.mkShell {
					inherit buildInputs nativeBuildInputs;

					RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
				};

				# 构建包
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "char-cloud";
					version = "0.1.0";
					src = ./.;

					cargoLock.lockFile = ./Cargo.lock;


					nativeBuildInputs = [ pkgs.pkg-config ];
					buildInputs = buildInputs;

					meta = with pkgs.lib; {
						description = "A CLI tool for generating custom shape word clouds";
						homepage = "https://github.com/yourusername/char-cloud";
						license = licenses.agpl3Only;
						maintainers = [ ];
						mainProgram = "char-cloud";
					};
				};
			}
		);
}