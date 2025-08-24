{ pkgs, craneLib }:

craneLib.buildPackage {
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
}