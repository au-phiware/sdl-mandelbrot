{ lib, buildRustCrate, buildRustCrateHelpers }:
with buildRustCrateHelpers;
let inherit (lib.lists) fold;
    inherit (lib.attrsets) recursiveUpdate;
in
rec {

# autocfg-1.0.0

  crates.autocfg."1.0.0" = deps: { features?(features_."autocfg"."1.0.0" deps {}) }: buildRustCrate {
    crateName = "autocfg";
    version = "1.0.0";
    description = "Automatic cfg for Rust compiler features";
    authors = [ "Josh Stone <cuviper@gmail.com>" ];
    sha256 = "1hhgqh551gmws22z9rxbnsvlppwxvlj0nszj7n1x56pqa3j3swy7";
  };
  features_."autocfg"."1.0.0" = deps: f: updateFeatures f (rec {
    autocfg."1.0.0".default = (f.autocfg."1.0.0".default or true);
  }) [];


# end

# bitflags-1.2.1

  crates.bitflags."1.2.1" = deps: { features?(features_."bitflags"."1.2.1" deps {}) }: buildRustCrate {
    crateName = "bitflags";
    version = "1.2.1";
    description = "A macro to generate structures which behave like bitflags.
";
    homepage = "https://github.com/bitflags/bitflags";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0b77awhpn7yaqjjibm69ginfn996azx5vkzfjj39g3wbsqs7mkxg";
    build = "build.rs";
    features = mkFeatures (features."bitflags"."1.2.1" or {});
  };
  features_."bitflags"."1.2.1" = deps: f: updateFeatures f (rec {
    bitflags."1.2.1".default = (f.bitflags."1.2.1".default or true);
  }) [];


# end

# cfg-if-0.1.10

  crates.cfg_if."0.1.10" = deps: { features?(features_."cfg_if"."0.1.10" deps {}) }: buildRustCrate {
    crateName = "cfg-if";
    version = "0.1.10";
    description = "A macro to ergonomically define an item depending on a large number of #[cfg]
parameters. Structured like an if-else chain, the first matching branch is the
item that gets emitted.
";
    homepage = "https://github.com/alexcrichton/cfg-if";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    edition = "2018";
    sha256 = "0x52qzpbyl2f2jqs7kkqzgfki2cpq99gpfjjigdp8pwwfqk01007";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."cfg_if"."0.1.10" or {});
  };
  features_."cfg_if"."0.1.10" = deps: f: updateFeatures f (rec {
    cfg_if = fold recursiveUpdate {} [
      { "0.1.10"."compiler_builtins" =
        (f.cfg_if."0.1.10"."compiler_builtins" or false) ||
        (f.cfg_if."0.1.10"."rustc-dep-of-std" or false) ||
        (cfg_if."0.1.10"."rustc-dep-of-std" or false); }
      { "0.1.10"."core" =
        (f.cfg_if."0.1.10"."core" or false) ||
        (f.cfg_if."0.1.10"."rustc-dep-of-std" or false) ||
        (cfg_if."0.1.10"."rustc-dep-of-std" or false); }
      { "0.1.10".default = (f.cfg_if."0.1.10".default or true); }
    ];
  }) [];


# end

# lazy_static-1.4.0

  crates.lazy_static."1.4.0" = deps: { features?(features_."lazy_static"."1.4.0" deps {}) }: buildRustCrate {
    crateName = "lazy_static";
    version = "1.4.0";
    description = "A macro for declaring lazily evaluated statics in Rust.";
    authors = [ "Marvin Löbel <loebel.marvin@gmail.com>" ];
    sha256 = "13h6sdghdcy7vcqsm2gasfw3qg7ssa0fl3sw7lq6pdkbk52wbyfr";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."lazy_static"."1.4.0" or {});
  };
  features_."lazy_static"."1.4.0" = deps: f: updateFeatures f (rec {
    lazy_static = fold recursiveUpdate {} [
      { "1.4.0"."spin" =
        (f.lazy_static."1.4.0"."spin" or false) ||
        (f.lazy_static."1.4.0"."spin_no_std" or false) ||
        (lazy_static."1.4.0"."spin_no_std" or false); }
      { "1.4.0".default = (f.lazy_static."1.4.0".default or true); }
    ];
  }) [];


# end

# libc-0.2.66

  crates.libc."0.2.66" = deps: { features?(features_."libc"."0.2.66" deps {}) }: buildRustCrate {
    crateName = "libc";
    version = "0.2.66";
    description = "Raw FFI bindings to platform libraries like libc.
";
    homepage = "https://github.com/rust-lang/libc";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0wz5fdpjpj8qp7wx7gq9rqckd2bdv7hcm5631hq03amxy5ikhi3l";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."libc"."0.2.66" or {});
  };
  features_."libc"."0.2.66" = deps: f: updateFeatures f (rec {
    libc = fold recursiveUpdate {} [
      { "0.2.66"."align" =
        (f.libc."0.2.66"."align" or false) ||
        (f.libc."0.2.66"."rustc-dep-of-std" or false) ||
        (libc."0.2.66"."rustc-dep-of-std" or false); }
      { "0.2.66"."rustc-std-workspace-core" =
        (f.libc."0.2.66"."rustc-std-workspace-core" or false) ||
        (f.libc."0.2.66"."rustc-dep-of-std" or false) ||
        (libc."0.2.66"."rustc-dep-of-std" or false); }
      { "0.2.66"."std" =
        (f.libc."0.2.66"."std" or false) ||
        (f.libc."0.2.66"."default" or false) ||
        (libc."0.2.66"."default" or false) ||
        (f.libc."0.2.66"."use_std" or false) ||
        (libc."0.2.66"."use_std" or false); }
      { "0.2.66".default = (f.libc."0.2.66".default or true); }
    ];
  }) [];


# end

# num-complex-0.2.4

  crates.num_complex."0.2.4" = deps: { features?(features_."num_complex"."0.2.4" deps {}) }: buildRustCrate {
    crateName = "num-complex";
    version = "0.2.4";
    description = "Complex numbers implementation for Rust";
    homepage = "https://github.com/rust-num/num-complex";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1zsrdcl2yigvg0zbrpgq4j4d9v74a15bkkdpr24wn0xvjz3bi2s1";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."num_traits"."${deps."num_complex"."0.2.4"."num_traits"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."autocfg"."${deps."num_complex"."0.2.4"."autocfg"}" deps)
    ]);
    features = mkFeatures (features."num_complex"."0.2.4" or {});
  };
  features_."num_complex"."0.2.4" = deps: f: updateFeatures f (rec {
    autocfg."${deps.num_complex."0.2.4".autocfg}".default = true;
    num_complex = fold recursiveUpdate {} [
      { "0.2.4"."std" =
        (f.num_complex."0.2.4"."std" or false) ||
        (f.num_complex."0.2.4"."default" or false) ||
        (num_complex."0.2.4"."default" or false); }
      { "0.2.4".default = (f.num_complex."0.2.4".default or true); }
    ];
    num_traits = fold recursiveUpdate {} [
      { "${deps.num_complex."0.2.4".num_traits}"."i128" =
        (f.num_traits."${deps.num_complex."0.2.4".num_traits}"."i128" or false) ||
        (num_complex."0.2.4"."i128" or false) ||
        (f."num_complex"."0.2.4"."i128" or false); }
      { "${deps.num_complex."0.2.4".num_traits}"."std" =
        (f.num_traits."${deps.num_complex."0.2.4".num_traits}"."std" or false) ||
        (num_complex."0.2.4"."std" or false) ||
        (f."num_complex"."0.2.4"."std" or false); }
      { "${deps.num_complex."0.2.4".num_traits}".default = (f.num_traits."${deps.num_complex."0.2.4".num_traits}".default or false); }
    ];
  }) [
    (f: if deps."num_complex"."0.2.4" ? "num_traits" then features_.num_traits."${deps."num_complex"."0.2.4"."num_traits" or ""}" deps f else f)
    (f: if deps."num_complex"."0.2.4" ? "autocfg" then features_.autocfg."${deps."num_complex"."0.2.4"."autocfg" or ""}" deps f else f)
  ];


# end

# num-traits-0.2.11

  crates.num_traits."0.2.11" = deps: { features?(features_."num_traits"."0.2.11" deps {}) }: buildRustCrate {
    crateName = "num-traits";
    version = "0.2.11";
    description = "Numeric traits for generic mathematics";
    homepage = "https://github.com/rust-num/num-traits";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0vr6ca475yglxdz7qriasdziq0cqn54z0z1xkmiqlnjx30plwmbl";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);

    buildDependencies = mapFeatures features ([
      (crates."autocfg"."${deps."num_traits"."0.2.11"."autocfg"}" deps)
    ]);
    features = mkFeatures (features."num_traits"."0.2.11" or {});
  };
  features_."num_traits"."0.2.11" = deps: f: updateFeatures f (rec {
    autocfg."${deps.num_traits."0.2.11".autocfg}".default = true;
    num_traits = fold recursiveUpdate {} [
      { "0.2.11"."std" =
        (f.num_traits."0.2.11"."std" or false) ||
        (f.num_traits."0.2.11"."default" or false) ||
        (num_traits."0.2.11"."default" or false); }
      { "0.2.11".default = (f.num_traits."0.2.11".default or true); }
    ];
  }) [
    (f: if deps."num_traits"."0.2.11" ? "autocfg" then features_.autocfg."${deps."num_traits"."0.2.11"."autocfg" or ""}" deps f else f)
  ];


# end

# sdl2-0.33.0

  crates.sdl2."0.33.0" = deps: { features?(features_."sdl2"."0.33.0" deps {}) }: buildRustCrate {
    crateName = "sdl2";
    version = "0.33.0";
    description = "SDL2 bindings for Rust";
    authors = [ "Tony Aldridge <tony@angry-lawyer.com>" "Cobrand <cobrandw@gmail.com>" ];
    sha256 = "1r6s5qsyn2fb57nkp9d0y9p91c5h970qnc5ak18jibrbapv7y7pl";
    libPath = "src/sdl2/lib.rs";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."sdl2"."0.33.0"."bitflags"}" deps)
      (crates."lazy_static"."${deps."sdl2"."0.33.0"."lazy_static"}" deps)
      (crates."libc"."${deps."sdl2"."0.33.0"."libc"}" deps)
      (crates."sdl2_sys"."${deps."sdl2"."0.33.0"."sdl2_sys"}" deps)
    ]);
    features = mkFeatures (features."sdl2"."0.33.0" or {});
  };
  features_."sdl2"."0.33.0" = deps: f: updateFeatures f (rec {
    bitflags."${deps.sdl2."0.33.0".bitflags}".default = true;
    lazy_static."${deps.sdl2."0.33.0".lazy_static}".default = true;
    libc."${deps.sdl2."0.33.0".libc}".default = true;
    sdl2 = fold recursiveUpdate {} [
      { "0.33.0"."c_vec" =
        (f.sdl2."0.33.0"."c_vec" or false) ||
        (f.sdl2."0.33.0"."gfx" or false) ||
        (sdl2."0.33.0"."gfx" or false); }
      { "0.33.0".default = (f.sdl2."0.33.0".default or true); }
    ];
    sdl2_sys = fold recursiveUpdate {} [
      { "${deps.sdl2."0.33.0".sdl2_sys}"."bundled" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."bundled" or false) ||
        (sdl2."0.33.0"."bundled" or false) ||
        (f."sdl2"."0.33.0"."bundled" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."gfx" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."gfx" or false) ||
        (sdl2."0.33.0"."gfx" or false) ||
        (f."sdl2"."0.33.0"."gfx" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."image" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."image" or false) ||
        (sdl2."0.33.0"."image" or false) ||
        (f."sdl2"."0.33.0"."image" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."mixer" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."mixer" or false) ||
        (sdl2."0.33.0"."mixer" or false) ||
        (f."sdl2"."0.33.0"."mixer" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."static-link" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."static-link" or false) ||
        (sdl2."0.33.0"."static-link" or false) ||
        (f."sdl2"."0.33.0"."static-link" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."ttf" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."ttf" or false) ||
        (sdl2."0.33.0"."ttf" or false) ||
        (f."sdl2"."0.33.0"."ttf" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."use-bindgen" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."use-bindgen" or false) ||
        (sdl2."0.33.0"."use-bindgen" or false) ||
        (f."sdl2"."0.33.0"."use-bindgen" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."use-pkgconfig" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."use-pkgconfig" or false) ||
        (sdl2."0.33.0"."use-pkgconfig" or false) ||
        (f."sdl2"."0.33.0"."use-pkgconfig" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}"."use_mac_framework" =
        (f.sdl2_sys."${deps.sdl2."0.33.0".sdl2_sys}"."use_mac_framework" or false) ||
        (sdl2."0.33.0"."use_mac_framework" or false) ||
        (f."sdl2"."0.33.0"."use_mac_framework" or false); }
      { "${deps.sdl2."0.33.0".sdl2_sys}".default = true; }
    ];
  }) [
    (f: if deps."sdl2"."0.33.0" ? "bitflags" then features_.bitflags."${deps."sdl2"."0.33.0"."bitflags" or ""}" deps f else f)
    (f: if deps."sdl2"."0.33.0" ? "lazy_static" then features_.lazy_static."${deps."sdl2"."0.33.0"."lazy_static" or ""}" deps f else f)
    (f: if deps."sdl2"."0.33.0" ? "libc" then features_.libc."${deps."sdl2"."0.33.0"."libc" or ""}" deps f else f)
    (f: if deps."sdl2"."0.33.0" ? "sdl2_sys" then features_.sdl2_sys."${deps."sdl2"."0.33.0"."sdl2_sys" or ""}" deps f else f)
  ];


# end

# sdl2-sys-0.33.0

  crates.sdl2_sys."0.33.0" = deps: { features?(features_."sdl2_sys"."0.33.0" deps {}) }: buildRustCrate {
    crateName = "sdl2-sys";
    version = "0.33.0";
    description = "Raw SDL2 bindings for Rust, used internally rust-sdl2";
    authors = [ "Tony Aldridge <tony@angry-lawyer.com>" ];
    edition = "2018";
    sha256 = "0znn3j9jl5n00psgdnna4lgc3xpnaix7azmg1ssqljixd3020h9k";
    libPath = "src/lib.rs";
    libName = "sdl2_sys";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."sdl2_sys"."0.33.0"."libc"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."sdl2_sys"."0.33.0"."cfg_if"}" deps)
    ]);
    features = mkFeatures (features."sdl2_sys"."0.33.0" or {});
  };
  features_."sdl2_sys"."0.33.0" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.sdl2_sys."0.33.0".cfg_if}".default = true;
    libc."${deps.sdl2_sys."0.33.0".libc}".default = true;
    sdl2_sys = fold recursiveUpdate {} [
      { "0.33.0"."bindgen" =
        (f.sdl2_sys."0.33.0"."bindgen" or false) ||
        (f.sdl2_sys."0.33.0"."use-bindgen" or false) ||
        (sdl2_sys."0.33.0"."use-bindgen" or false); }
      { "0.33.0"."cmake" =
        (f.sdl2_sys."0.33.0"."cmake" or false) ||
        (f.sdl2_sys."0.33.0"."bundled" or false) ||
        (sdl2_sys."0.33.0"."bundled" or false); }
      { "0.33.0"."flate2" =
        (f.sdl2_sys."0.33.0"."flate2" or false) ||
        (f.sdl2_sys."0.33.0"."bundled" or false) ||
        (sdl2_sys."0.33.0"."bundled" or false); }
      { "0.33.0"."pkg-config" =
        (f.sdl2_sys."0.33.0"."pkg-config" or false) ||
        (f.sdl2_sys."0.33.0"."use-pkgconfig" or false) ||
        (sdl2_sys."0.33.0"."use-pkgconfig" or false); }
      { "0.33.0"."tar" =
        (f.sdl2_sys."0.33.0"."tar" or false) ||
        (f.sdl2_sys."0.33.0"."bundled" or false) ||
        (sdl2_sys."0.33.0"."bundled" or false); }
      { "0.33.0"."unidiff" =
        (f.sdl2_sys."0.33.0"."unidiff" or false) ||
        (f.sdl2_sys."0.33.0"."bundled" or false) ||
        (sdl2_sys."0.33.0"."bundled" or false); }
      { "0.33.0".default = (f.sdl2_sys."0.33.0".default or true); }
    ];
  }) [
    (f: if deps."sdl2_sys"."0.33.0" ? "libc" then features_.libc."${deps."sdl2_sys"."0.33.0"."libc" or ""}" deps f else f)
    (f: if deps."sdl2_sys"."0.33.0" ? "cfg_if" then features_.cfg_if."${deps."sdl2_sys"."0.33.0"."cfg_if" or ""}" deps f else f)
  ];


# end

  crates.autocfg.default = crates.autocfg."1.0.0";
  features_.autocfg.default = features_.autocfg."1.0.0";

  crates.bitflags.default = crates.bitflags."1.2.1";
  features_.bitflags.default = features_.bitflags."1.2.1";

  crates.cfg_if.default = crates.cfg_if."0.1.10";
  features_.cfg_if.default = features_.cfg_if."0.1.10";

  crates.exit.default = crates.exit."0.1.0";
  features_.exit.default = features_.exit."0.1.0";

  crates.lazy_static.default = crates.lazy_static."1.4.0";
  features_.lazy_static.default = features_.lazy_static."1.4.0";

  crates.libc.default = crates.libc."0.2.66";
  features_.libc.default = features_.libc."0.2.66";

  crates.num_complex.default = crates.num_complex."0.2.4";
  features_.num_complex.default = features_.num_complex."0.2.4";

  crates.num_traits.default = crates.num_traits."0.2.11";
  features_.num_traits.default = features_.num_traits."0.2.11";

  crates.sdl_mandelbrot.default = crates.sdl_mandelbrot."0.1.0";
  features_.sdl_mandelbrot.default = features_.sdl_mandelbrot."0.1.0";

  crates.sdl2.default = crates.sdl2."0.33.0";
  features_.sdl2.default = features_.sdl2."0.33.0";

  crates.sdl2_sys.default = crates.sdl2_sys."0.33.0";
  features_.sdl2_sys.default = features_.sdl2_sys."0.33.0";

}
