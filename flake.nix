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

				rustToolchain = pkgs.rust-bin.stable.latest.default;

				buildCommon = targetName: {
					pname = "char-cloud";
					version = "0.1.0";
					src = ./.;
					cargoLock = {
						lockFile = ./Cargo.lock;
					};
					buildFeatures = [ "embedded_fonts" ];
					nativeBuildInputs = [ pkgs.pkg-config ];
					meta = with pkgs.lib; {
						description = "A CLI tool for generating custom shape word clouds";
						homepage = "https://github.com/yourusername/char-cloud";
						license = licenses.agpl3Only;
						maintainers = [ "Acture <acturea@gmail.com>" ];
						mainProgram = "char-cloud-${targetName}";
					};
					installPhase = ''
						mkdir -p $out/bin
						cp target/*/release/char-cloud $out/bin/char-cloud-${targetName}
					'';
				};

				mkFor = target: import nixpkgs {
					inherit overlays;
					system = system;
					crossSystem = target;
				};

				mkBuild = target: targetName:
					let
						crossPkgs = mkFor target;
						args = buildCommon targetName;
					in
					crossPkgs.rustPlatform.buildRustPackage args;

				nativeArgs = buildCommon system;
			in {
				packages = let
				  targetMap = {
						"x86_64-linux" = { config = "x86_64-unknown-linux-gnu"; name = "x86_64-linux"; };
						"x86_64-darwin" = { config = "x86_64-apple-darwin"; name = "x86_64-macos"; };
						"aarch64-darwin" = { config = "aarch64-apple-darwin"; name = "aarch64-macos"; };
						"x86_64-windows" = { config = "x86_64-pc-windows-gnu"; name = "x86_64-windows"; };
						"aarch64-linux" = { config = "aarch64-unknown-linux-gnu"; name = "aarch64-linux"; };
				  };
				in
				let
				  target = pkgs.lib.getAttr system targetMap;
				in
				{
					default = mkBuild { config = target.config; } target.name;

					aarch64-linux = mkBuild { config = "aarch64-unknown-linux-gnu"; } "aarch64-linux";
					wasm = mkBuild { config = "wasm32-wasi"; } "wasm";
					x86_64-windows = mkBuild { config = "x86_64-pc-windows-gnu"; } "x86_64-windows";
					x86_64-macos = mkBuild { config = "x86_64-apple-darwin"; } "x86_64-macos";
					aarch64-macos = mkBuild { config = "aarch64-apple-darwin"; } "aarch64-macos";
					all = pkgs.symlinkJoin {
						name = "char-cloud-all";
						paths = [
						  ./aarch64-linux
						  ./wasm
						  ./x86_64-windows
						  ./x86_64-macos
						  ./aarch64-macos
						];
					};
				};

				devShells.default = pkgs.mkShell {
					nativeBuildInputs = [ rustToolchain pkgs.pkg-config ];
					RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
				};
			}
		);
}