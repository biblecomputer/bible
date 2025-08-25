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

        commonArgs = {
          strictDeps = true;
          buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };

        nativeArgs = commonArgs // {
          src = pkgs.lib.fileset.toSource {
            root = ./site;
            fileset = pkgs.lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./site)
              (pkgs.lib.fileset.fileFilter (
                file: pkgs.lib.any file.hasExt [ "html" "scss" "css" "js" "json" "txt" "png" ]
              ) ./site)
              (pkgs.lib.fileset.maybeMissing ./assets)
            ];
          };
          pname = "trunk-workspace-native";
        };

        cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

        site = import ./site/site.nix {
          inherit pkgs craneLib rustToolchain;
        };

        bibleVerify = import ./bible-verify/bible-verify.nix {
          inherit pkgs craneLib;
        };

      in {
        packages = {
          default = site;
          bible-verify = bibleVerify;
        };

        checks = {
          inherit site;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            src = ./site;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          fmt = craneLib.cargoFmt (commonArgs // { src = ./site; });
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
            drv = site;
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
