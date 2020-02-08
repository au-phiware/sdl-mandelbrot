import <nixpkgs> {
  overlays = [
    (import (builtins.fetchGit {
      url = https://github.com/mozilla/nixpkgs-mozilla.git;
      ref = "master";
      rev = "5300241b41243cb8962fad284f0004afad187dad";
    }))
    (self: super: rec {
      cratesIO = super.pkgs.callPackage ./crates-io.nix {};
      rustChannel = super.rustChannelOf {
        date = "2020-02-01";
        channel = "nightly";
      };
      rust = rustChannel.rust;
      cargo = super.pkgs.callPackage ./Cargo.nix {
        inherit cratesIO;
      };
    })
  ];
}
