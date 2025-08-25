{ pkgs, craneLib, rustToolchain }:

let
  unfilteredRoot = ./.;
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

  wasmArgs = commonArgs // {
    pname = "trunk-workspace-wasm";
    cargoExtraArgs = "--package=bible";
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
  };

  cargoArtifactsWasm = craneLib.buildDepsOnly (wasmArgs // {
    doCheck = false;
  });

in craneLib.buildTrunkPackage (
  wasmArgs // {
    pname = "trunk-workspace-client";
    cargoArtifacts = cargoArtifactsWasm;

    nativeBuildInputs = (wasmArgs.nativeBuildInputs or []) ++ [ 
      pkgs.tailwindcss
    ];

    preBuild = ''
      mkdir -p target/trunk-cache
      
      echo "Building Tailwind CSS..."
      ${pkgs.tailwindcss}/bin/tailwindcss \
        -i ./style/tailwind.css \
        -o ./style/output.css \
        --config ./tailwind.config.js
    '';
    TRUNK_CACHE_DIR = "./target/trunk-cache";
    HOME = "./target/home";

    postBuild = ''
      cp ./dist/index.html ./dist/404.html
      
      mv ./dist ..
      cd ..
    '';

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
)