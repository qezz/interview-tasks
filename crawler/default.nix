let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  # unstable = import (fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz) { };
  rust-channel = (nixpkgs.rustChannelOf { date = "2021-11-02"; channel = "nightly"; });
  rust-nightly = rust-channel.rust.override {
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
    name = "moz_overlay_shell";
    runtimeBuildInputs = [
      pkgconfig
      rust-analyzer
      tdlib
    ];
    buildInputs = [
      # to use the latest nightly:
      # nixpkgs.latest.rustChannels.nightly.rust
      # to use a specific nighly:
      # (nixpkgs.rustChannelOf { date = "2021-01-07"; channel = "nightly"; }).rust
      rust-nightly
      # to use the project's rust-toolchain file:
      # (nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust
      rust-analyzer
      pkgconfig
      openssl
      tdlib
    ];

    LD_LIBRARY_PATH = lib.makeLibraryPath [
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      libglvnd
      tdlib
    ];
  }
