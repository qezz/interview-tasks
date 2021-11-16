let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust-nightly = nixpkgs.latest.rustChannels.nightly.rust.override {
     extensions = [
       "rust-src"
       "rls-preview"
       "rust-analysis"
       "rustfmt-preview"
     ];
   };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust-nightly-env";
    buildInputs = [
      rust-nightly
      pkgconfig
      openssl
      perf-tools
      linuxPackages.perf
    ];
  }
