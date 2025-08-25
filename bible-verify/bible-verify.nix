{ pkgs, craneLib }:

craneLib.buildPackage {
  src = pkgs.lib.fileset.toSource {
    root = ./.;
    fileset = pkgs.lib.fileset.unions [
      (craneLib.fileset.commonCargoSources ./.)
      (pkgs.lib.fileset.maybeMissing ./kjv.json)
    ];
  };
  pname = "bible-verify";
  version = "0.1.0";
  strictDeps = true;
  buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.libiconv
  ];
}