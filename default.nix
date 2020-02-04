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
  rustChannel = rustChannelOf {
    date = "2020-02-01";
    channel = "nightly";
  };
  twey = builtins.fetchGit {
    url = https://github.com/au-phiware/mkRustCrate.git;
    ref = "master";
    rev = "73b101527e6396cfa558860c572965e64cfb6ccd";
  };
  mkRustCrate = callPackage "${twey}/mkRustCrate/lib/mkRustCrate" {
    cargo = rustChannel.cargo;
    rust = rustChannel.rust;
  };
  fetchFromCratesIo = callPackage "${twey}/mkRustCrate/lib/fetchFromCratesIo" { };
in
mkRustCrate rec {
  name = "sdl-mandelbrot";
  version = "0.1.0";
  src = ./.;
  doCheck = true;
  buildInputs = [ SDL2 ];
  dependencies = [ rust_sdl2 num_complex num_traits exit ];
  buildDependencies = [];

  rust_sdl2 = mkRustCrate rec {
    name = "sdl2";
    version = "0.33.0";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "1r6s5qsyn2fb57nkp9d0y9p91c5h970qnc5ak18jibrbapv7y7pl";
    };
    dependencies = [ lazy_static sdl2_sys libc bitflags ];
    buildDependencies = [ ];
  };

  num_complex = mkRustCrate rec {
    name = "num-complex";
    version = "0.2.4";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "1zsrdcl2yigvg0zbrpgq4j4d9v74a15bkkdpr24wn0xvjz3bi2s1";
    };
    features = [ "std" ];
    dependencies = [ num_traits ];
    buildDependencies = [ autocfg ];
  };

  exit = mkRustCrate rec {
    name = "exit";
    version = "0.1.0";
    src = fetchgit {
      url = https://github.com/au-phiware/exit.git;
      rev = "a625f26bd55b27f27ddf4c7859b5ca826ec06720";
      sha256 = "06hsg98qnn87x1yv8k21nswlf744vbj0ng51jfdx9svhjb09s6sy";
    };
  };

  sdl2_sys = mkRustCrate rec {
    name = "sdl2-sys";
    version = "0.33.0";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "0znn3j9jl5n00psgdnna4lgc3xpnaix7azmg1ssqljixd3020h9k";
    };
    dependencies = [ libc ];
    buildDependencies = [ cfg_if ];
  };

  lazy_static = mkRustCrate rec {
    name = "lazy_static";
    version = "1.4.0";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "13h6sdghdcy7vcqsm2gasfw3qg7ssa0fl3sw7lq6pdkbk52wbyfr";
    };
  };

  libc = mkRustCrate rec {
    name = "libc";
    version = "0.2.66";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "0wz5fdpjpj8qp7wx7gq9rqckd2bdv7hcm5631hq03amxy5ikhi3l";
    };
  };

  cfg_if = mkRustCrate rec {
    name = "cfg-if";
    version = "0.1.10";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "0x52qzpbyl2f2jqs7kkqzgfki2cpq99gpfjjigdp8pwwfqk01007";
    };
  };

  bitflags = mkRustCrate rec {
    name = "bitflags";
    version = "1.2.1";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "0b77awhpn7yaqjjibm69ginfn996azx5vkzfjj39g3wbsqs7mkxg";
    };
  };

  num_traits = mkRustCrate rec {
    name = "num-traits";
    version = "0.2.11";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "0vr6ca475yglxdz7qriasdziq0cqn54z0z1xkmiqlnjx30plwmbl";
    };
    features = [ "libm" ];
    dependencies = [ libm ];
    buildDependencies = [ autocfg ];
  };

  libm = mkRustCrate rec {
    name = "libm";
    version = "0.2.1";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "1c1jz9pkbv71icgi3cvzhh9g3mzjmj0bc8xv4l1w2b0wqcxxjnlc";
    };
  };

  autocfg = mkRustCrate rec {
    name = "autocfg";
    version = "1.0.0";
    src = fetchFromCratesIo {
      inherit name version;
      sha256 = "1hhgqh551gmws22z9rxbnsvlppwxvlj0nszj7n1x56pqa3j3swy7";
    };
  };
}
