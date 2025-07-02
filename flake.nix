{
  description = "Build a cargo project with Trunk and Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (_: rustToolchain);

        unfilteredRoot = ./site;
        src = pkgs.lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = pkgs.lib.fileset.unions [
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (pkgs.lib.fileset.fileFilter (
              file: pkgs.lib.any file.hasExt [ "html" "scss" "css" "js" "json" ]
            ) unfilteredRoot)
            (pkgs.lib.fileset.maybeMissing ./assets)
          ];
        };

        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };

        nativeArgs = commonArgs // {
          pname = "trunk-workspace-native";
        };

        cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

        wasmArgs = commonArgs // {
          pname = "trunk-workspace-wasm";
          cargoExtraArgs = "--package=bible";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        cargoArtifactsWasm = craneLib.buildDepsOnly (wasmArgs // {
          doCheck = false;
        });

        myClient = craneLib.buildTrunkPackage (
          wasmArgs // {
            pname = "trunk-workspace-client";
            cargoArtifacts = cargoArtifactsWasm;

            # Fix trunk caching issues in sandbox
            preBuild = ''
              mkdir -p target/trunk-cache
            '';
            TRUNK_CACHE_DIR = "./target/trunk-cache";
            HOME = "./target/home";

            postBuild = ''
              mv ./dist ..
              cd ..
            '';

            # Required wasm-bindgen-cli setup
            wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
              src = pkgs.fetchCrate {
                pname = "wasm-bindgen-cli";
                version = "0.2.100";
                hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
              };

              cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
                inherit src;
                inherit (src) pname version;
                hash = "sha256-qsO12332HSjWCVKtf1cUePWWb9IdYUmT+8OPj/XP2WE=";
              };
            };
          }
        );
      in {
        packages.default = myClient;

        checks = {
          inherit myClient;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          fmt = craneLib.cargoFmt commonArgs;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = [
            pkgs.trunk
            pkgs.cargo-leptos
          ];
          shellHook = ''
            export CLIENT_DIST=$PWD/client/dist
          '';
        };

        apps.default = flake-utils.lib.mkApp {
          name = "bible";
          drv = myClient;
        };
      }
    );
}
