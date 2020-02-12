import <nixpkgs> {
  overlays = [
    (import (builtins.fetchGit {
      url = https://github.com/mozilla/nixpkgs-mozilla.git;
      ref = "master";
      rev = "36455d54de0b40d9432bba6d8207a5582210b3eb";
    }))
    (self: super: rec {
      cratesIO = super.pkgs.callPackage ./crates-io.nix {};
      rustChannel = super.rustChannelOf {
        rustToolchain = ./rust-toolchain;
      };
      rust = rustChannel.rust;
      cargo = super.pkgs.callPackage ./Cargo.nix {
        inherit cratesIO;
      };
    })
  ];
}
