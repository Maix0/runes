{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
    in {
      devShell = with pkgs;
        mkShell {
          nativeBuildInputs = [pkgs.bashInteractive pkg-config];
          buildInputs = [
            # Rust
            (rust-bin.stable.latest.default.override {
              targets = ["x86_64-unknown-linux-gnu"];
            })
            mold
			clang
			cmake
			pkgconfig 
			fontconfig
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath [wayland wayland-protocols libxkbcommon vulkan-loader libGL];
          packages = [pkg-config libxkbcommon wayland-utils vulkan-headers vulkan-loader vulkan-validation-layers vulkan-tools];
        };
    });
}
