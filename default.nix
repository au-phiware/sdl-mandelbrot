with (import <nixpkgs> {
  overlays = [
    (import (builtins.fetchGit {
      url = https://github.com/mozilla/nixpkgs-mozilla.git;
      ref = "master";
      rev = "5300241b41243cb8962fad284f0004afad187dad";
    }))
  ];
});
let
  cratesIO = pkgs.callPackage ./crates-io.nix {};
  rustChannel = rustChannelOf {
    date = "2020-02-01";
    channel = "nightly";
  };
  rust = rustChannel.rust;
  cargo = pkgs.callPackage ./Cargo.nix {
    inherit cratesIO;
  };
in
  (cargo.sdl_mandelbrot {}).override {
    inherit rust;
    crateOverrides = defaultCrateOverrides // {
      "sdl-mandelbrot" = attrs: { buildInputs = [ SDL2 ]; };
    };
  }
