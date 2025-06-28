{
  description = "Rust flake using naersk and rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        naersk-lib = pkgs.callPackage naersk { inherit rust-toolchain; };
      in {
        packages.default = naersk-lib.buildPackage {
          pname = "leptos-tutorial";
          root = ./leptos-tutorial;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rust-toolchain
            pkgs.rust-analyzer
            pkgs.trunk
          ];
        };
      });
}
