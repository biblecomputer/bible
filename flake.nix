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
              file: pkgs.lib.any file.hasExt [ "html" "scss" "css" "js" "json" "txt" "png" ]
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

            # Add tailwindcss as build input
            nativeBuildInputs = (wasmArgs.nativeBuildInputs or []) ++ [ 
              pkgs.tailwindcss
            ];

            # Fix trunk caching issues in sandbox and build tailwind
            preBuild = ''
              mkdir -p target/trunk-cache
              
              # Build tailwind CSS
              echo "Building Tailwind CSS..."
              ${pkgs.tailwindcss}/bin/tailwindcss \
                -i ./style/tailwind.css \
                -o ./style/output.css \
                --config ./tailwind.config.js
            '';
            TRUNK_CACHE_DIR = "./target/trunk-cache";
            HOME = "./target/home";

            postBuild = ''
              # Copy index.html as 404.html
              cp ./dist/index.html ./dist/404.html
              
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
        packages = {
          default = myClient;
          bible-verify = craneLib.buildPackage {
            src = pkgs.lib.fileset.toSource {
              root = ./bible-verify;
              fileset = pkgs.lib.fileset.unions [
                (craneLib.fileset.commonCargoSources ./bible-verify)
                (pkgs.lib.fileset.maybeMissing ./bible-verify/kjv.json)
              ];
            };
            pname = "bible-verify";
            version = "0.1.0";
            strictDeps = true;
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.libiconv
            ];
          };
        };

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
            pkgs.rust-analyzer
            pkgs.miniserve
          ];
          shellHook = ''
            export CLIENT_DIST=$PWD/client/dist
          '';
        };

        apps = {
          default = flake-utils.lib.mkApp {
            name = "bible";
            drv = myClient;
          };
          
          dev = flake-utils.lib.mkApp {
            name = "dev-server";
            drv = pkgs.writeShellScriptBin "dev-server" ''
              set -e
              
              # Work in the current directory (should be the project root)
              if [ ! -d "site" ]; then
                echo "Error: Please run this from the project root directory"
                exit 1
              fi
              
              cd site
              
              # Ensure output.css exists
              if [ ! -f "style/output.css" ]; then
                echo "Generating initial Tailwind CSS..."
                ${pkgs.tailwindcss}/bin/tailwindcss \
                  -i ./style/tailwind.css \
                  -o ./style/output.css \
                  --config ./tailwind.config.js
              fi
              
              # Start Tailwind CSS watch in background
              echo "Starting Tailwind CSS watcher..."
              ${pkgs.tailwindcss}/bin/tailwindcss \
                -i ./style/tailwind.css \
                -o ./style/output.css \
                --config ./tailwind.config.js \
                --watch &
              TAILWIND_PID=$!
              
              # Cleanup function
              cleanup() {
                echo "Stopping development server..."
                kill $TAILWIND_PID 2>/dev/null || true
                exit 0
              }
              
              # Trap cleanup
              trap cleanup INT TERM
              
              # Start trunk serve
              echo "Starting trunk development server..."
              ${pkgs.trunk}/bin/trunk serve --open --port 8080
            '';
          };
        };
      }
    );
}
