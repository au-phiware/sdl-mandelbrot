{ lib, buildRustCrate, buildRustCrateHelpers }:
with buildRustCrateHelpers;
let inherit (lib.lists) fold;
    inherit (lib.attrsets) recursiveUpdate;
in
rec {

# approx-0.3.2

  crates.approx."0.3.2" = deps: { features?(features_."approx"."0.3.2" deps {}) }: buildRustCrate {
    crateName = "approx";
    version = "0.3.2";
    description = "Approximate floating point equality comparisons and assertions.";
    homepage = "https://github.com/brendanzab/approx";
    authors = [ "Brendan Zabarauskas <bjzaba@yahoo.com.au>" ];
    sha256 = "1sziapqxz6qk1m3bv4pay1bfqv887pf6045qfg913ip4jmyv8fcb";
    dependencies = mapFeatures features ([
      (crates."num_traits"."${deps."approx"."0.3.2"."num_traits"}" deps)
    ]);
    features = mkFeatures (features."approx"."0.3.2" or {});
  };
  features_."approx"."0.3.2" = deps: f: updateFeatures f (rec {
    approx = fold recursiveUpdate {} [
      { "0.3.2"."std" =
        (f.approx."0.3.2"."std" or false) ||
        (f.approx."0.3.2"."default" or false) ||
        (approx."0.3.2"."default" or false); }
      { "0.3.2".default = (f.approx."0.3.2".default or true); }
    ];
    num_traits."${deps.approx."0.3.2".num_traits}".default = true;
  }) [
    (f: if deps."approx"."0.3.2" ? "num_traits" then features_.num_traits."${deps."approx"."0.3.2"."num_traits" or ""}" deps f else f)
  ];


# end

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

# c2-chacha-0.2.3

  crates.c2_chacha."0.2.3" = deps: { features?(features_."c2_chacha"."0.2.3" deps {}) }: buildRustCrate {
    crateName = "c2-chacha";
    version = "0.2.3";
    description = "The ChaCha family of stream ciphers";
    authors = [ "The CryptoCorrosion Contributors" ];
    edition = "2018";
    sha256 = "04vh0cc9g94cj6cq96sfv3lks7rx486jdn43rmqcvb2syh4y9dqj";
    dependencies = mapFeatures features ([
      (crates."ppv_lite86"."${deps."c2_chacha"."0.2.3"."ppv_lite86"}" deps)
    ]);
    features = mkFeatures (features."c2_chacha"."0.2.3" or {});
  };
  features_."c2_chacha"."0.2.3" = deps: f: updateFeatures f (rec {
    c2_chacha = fold recursiveUpdate {} [
      { "0.2.3"."byteorder" =
        (f.c2_chacha."0.2.3"."byteorder" or false) ||
        (f.c2_chacha."0.2.3"."rustcrypto_api" or false) ||
        (c2_chacha."0.2.3"."rustcrypto_api" or false); }
      { "0.2.3"."rustcrypto_api" =
        (f.c2_chacha."0.2.3"."rustcrypto_api" or false) ||
        (f.c2_chacha."0.2.3"."default" or false) ||
        (c2_chacha."0.2.3"."default" or false); }
      { "0.2.3"."simd" =
        (f.c2_chacha."0.2.3"."simd" or false) ||
        (f.c2_chacha."0.2.3"."default" or false) ||
        (c2_chacha."0.2.3"."default" or false); }
      { "0.2.3"."std" =
        (f.c2_chacha."0.2.3"."std" or false) ||
        (f.c2_chacha."0.2.3"."default" or false) ||
        (c2_chacha."0.2.3"."default" or false); }
      { "0.2.3"."stream-cipher" =
        (f.c2_chacha."0.2.3"."stream-cipher" or false) ||
        (f.c2_chacha."0.2.3"."rustcrypto_api" or false) ||
        (c2_chacha."0.2.3"."rustcrypto_api" or false); }
      { "0.2.3".default = (f.c2_chacha."0.2.3".default or true); }
    ];
    ppv_lite86 = fold recursiveUpdate {} [
      { "${deps.c2_chacha."0.2.3".ppv_lite86}"."simd" =
        (f.ppv_lite86."${deps.c2_chacha."0.2.3".ppv_lite86}"."simd" or false) ||
        (c2_chacha."0.2.3"."simd" or false) ||
        (f."c2_chacha"."0.2.3"."simd" or false); }
      { "${deps.c2_chacha."0.2.3".ppv_lite86}"."std" =
        (f.ppv_lite86."${deps.c2_chacha."0.2.3".ppv_lite86}"."std" or false) ||
        (c2_chacha."0.2.3"."std" or false) ||
        (f."c2_chacha"."0.2.3"."std" or false); }
      { "${deps.c2_chacha."0.2.3".ppv_lite86}".default = (f.ppv_lite86."${deps.c2_chacha."0.2.3".ppv_lite86}".default or false); }
    ];
  }) [
    (f: if deps."c2_chacha"."0.2.3" ? "ppv_lite86" then features_.ppv_lite86."${deps."c2_chacha"."0.2.3"."ppv_lite86" or ""}" deps f else f)
  ];


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

# getrandom-0.1.14

  crates.getrandom."0.1.14" = deps: { features?(features_."getrandom"."0.1.14" deps {}) }: buildRustCrate {
    crateName = "getrandom";
    version = "0.1.14";
    description = "A small cross-platform library for retrieving random data from system source";
    authors = [ "The Rand Project Developers" ];
    edition = "2018";
    sha256 = "1i6r4q7i24zdy6v5h3l966a1cf8a1aip2fi1pmdsi71sk1m3w7wr";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."getrandom"."0.1.14"."cfg_if"}" deps)
    ])
      ++ (if kernel == "wasi" then mapFeatures features ([
      (crates."wasi"."${deps."getrandom"."0.1.14"."wasi"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."getrandom"."0.1.14"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "wasm32-unknown-unknown" then mapFeatures features ([
]) else []);
    features = mkFeatures (features."getrandom"."0.1.14" or {});
  };
  features_."getrandom"."0.1.14" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.getrandom."0.1.14".cfg_if}".default = true;
    getrandom = fold recursiveUpdate {} [
      { "0.1.14"."compiler_builtins" =
        (f.getrandom."0.1.14"."compiler_builtins" or false) ||
        (f.getrandom."0.1.14"."rustc-dep-of-std" or false) ||
        (getrandom."0.1.14"."rustc-dep-of-std" or false); }
      { "0.1.14"."core" =
        (f.getrandom."0.1.14"."core" or false) ||
        (f.getrandom."0.1.14"."rustc-dep-of-std" or false) ||
        (getrandom."0.1.14"."rustc-dep-of-std" or false); }
      { "0.1.14"."wasm-bindgen" =
        (f.getrandom."0.1.14"."wasm-bindgen" or false) ||
        (f.getrandom."0.1.14"."test-in-browser" or false) ||
        (getrandom."0.1.14"."test-in-browser" or false); }
      { "0.1.14".default = (f.getrandom."0.1.14".default or true); }
    ];
    libc."${deps.getrandom."0.1.14".libc}".default = (f.libc."${deps.getrandom."0.1.14".libc}".default or false);
    wasi."${deps.getrandom."0.1.14".wasi}".default = true;
  }) [
    (f: if deps."getrandom"."0.1.14" ? "cfg_if" then features_.cfg_if."${deps."getrandom"."0.1.14"."cfg_if" or ""}" deps f else f)
    (f: if deps."getrandom"."0.1.14" ? "wasi" then features_.wasi."${deps."getrandom"."0.1.14"."wasi" or ""}" deps f else f)
    (f: if deps."getrandom"."0.1.14" ? "libc" then features_.libc."${deps."getrandom"."0.1.14"."libc" or ""}" deps f else f)
  ];


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

# palette-0.5.0

  crates.palette."0.5.0" = deps: { features?(features_."palette"."0.5.0" deps {}) }: buildRustCrate {
    crateName = "palette";
    version = "0.5.0";
    description = "Makes linear color calculations and conversion easy and accessible for anyone.";
    authors = [ "Erik Hedvall <hello@erikhedvall.nu>" ];
    sha256 = "0z67l2wh36b15axva91103m3k1fh8p0har0llg8rqpzmz42hlg9j";
    build = "build/main.rs";
    dependencies = mapFeatures features ([
      (crates."approx"."${deps."palette"."0.5.0"."approx"}" deps)
      (crates."num_traits"."${deps."palette"."0.5.0"."num_traits"}" deps)
      (crates."palette_derive"."${deps."palette"."0.5.0"."palette_derive"}" deps)
    ]
      ++ (if features.palette."0.5.0".phf or false then [ (crates.phf."${deps."palette"."0.5.0".phf}" deps) ] else []));

    buildDependencies = mapFeatures features ([
    ]
      ++ (if features.palette."0.5.0".phf_codegen or false then [ (crates.phf_codegen."${deps."palette"."0.5.0".phf_codegen}" deps) ] else []));
    features = mkFeatures (features."palette"."0.5.0" or {});
  };
  features_."palette"."0.5.0" = deps: f: updateFeatures f (rec {
    approx = fold recursiveUpdate {} [
      { "${deps.palette."0.5.0".approx}"."std" =
        (f.approx."${deps.palette."0.5.0".approx}"."std" or false) ||
        (palette."0.5.0"."std" or false) ||
        (f."palette"."0.5.0"."std" or false); }
      { "${deps.palette."0.5.0".approx}".default = (f.approx."${deps.palette."0.5.0".approx}".default or false); }
    ];
    num_traits = fold recursiveUpdate {} [
      { "${deps.palette."0.5.0".num_traits}"."libm" =
        (f.num_traits."${deps.palette."0.5.0".num_traits}"."libm" or false) ||
        (palette."0.5.0"."libm" or false) ||
        (f."palette"."0.5.0"."libm" or false); }
      { "${deps.palette."0.5.0".num_traits}"."std" =
        (f.num_traits."${deps.palette."0.5.0".num_traits}"."std" or false) ||
        (palette."0.5.0"."std" or false) ||
        (f."palette"."0.5.0"."std" or false); }
      { "${deps.palette."0.5.0".num_traits}".default = (f.num_traits."${deps.palette."0.5.0".num_traits}".default or false); }
    ];
    palette = fold recursiveUpdate {} [
      { "0.5.0"."named" =
        (f.palette."0.5.0"."named" or false) ||
        (f.palette."0.5.0"."named_from_str" or false) ||
        (palette."0.5.0"."named_from_str" or false); }
      { "0.5.0"."named_from_str" =
        (f.palette."0.5.0"."named_from_str" or false) ||
        (f.palette."0.5.0"."default" or false) ||
        (palette."0.5.0"."default" or false); }
      { "0.5.0"."phf" =
        (f.palette."0.5.0"."phf" or false) ||
        (f.palette."0.5.0"."named_from_str" or false) ||
        (palette."0.5.0"."named_from_str" or false); }
      { "0.5.0"."phf_codegen" =
        (f.palette."0.5.0"."phf_codegen" or false) ||
        (f.palette."0.5.0"."named_from_str" or false) ||
        (palette."0.5.0"."named_from_str" or false); }
      { "0.5.0"."serde" =
        (f.palette."0.5.0"."serde" or false) ||
        (f.palette."0.5.0"."serializing" or false) ||
        (palette."0.5.0"."serializing" or false); }
      { "0.5.0"."std" =
        (f.palette."0.5.0"."std" or false) ||
        (f.palette."0.5.0"."default" or false) ||
        (palette."0.5.0"."default" or false) ||
        (f.palette."0.5.0"."named_from_str" or false) ||
        (palette."0.5.0"."named_from_str" or false) ||
        (f.palette."0.5.0"."serializing" or false) ||
        (palette."0.5.0"."serializing" or false); }
      { "0.5.0".default = (f.palette."0.5.0".default or true); }
    ];
    palette_derive."${deps.palette."0.5.0".palette_derive}".default = true;
  }) [
    (f: if deps."palette"."0.5.0" ? "phf" then recursiveUpdate f { phf."${deps."palette"."0.5.0"."phf"}"."default" = true; } else f)
    (f: if deps."palette"."0.5.0" ? "phf_codegen" then recursiveUpdate f { phf_codegen."${deps."palette"."0.5.0"."phf_codegen"}"."default" = true; } else f)
    (f: if deps."palette"."0.5.0" ? "approx" then features_.approx."${deps."palette"."0.5.0"."approx" or ""}" deps f else f)
    (f: if deps."palette"."0.5.0" ? "num_traits" then features_.num_traits."${deps."palette"."0.5.0"."num_traits" or ""}" deps f else f)
    (f: if deps."palette"."0.5.0" ? "palette_derive" then features_.palette_derive."${deps."palette"."0.5.0"."palette_derive" or ""}" deps f else f)
    (f: if deps."palette"."0.5.0" ? "phf" then features_.phf."${deps."palette"."0.5.0"."phf" or ""}" deps f else f)
    (f: if deps."palette"."0.5.0" ? "phf_codegen" then features_.phf_codegen."${deps."palette"."0.5.0"."phf_codegen" or ""}" deps f else f)
  ];


# end

# palette_derive-0.5.0

  crates.palette_derive."0.5.0" = deps: { features?(features_."palette_derive"."0.5.0" deps {}) }: buildRustCrate {
    crateName = "palette_derive";
    version = "0.5.0";
    description = "Automatically implement traits from the palette crate.";
    authors = [ "Erik Hedvall <hello@erikhedvall.nu>" ];
    sha256 = "13m20w23022sngbip6r93gqbvja29593hnh3lmvgwcfxnpf1hcxi";
    procMacro = true;
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."palette_derive"."0.5.0"."proc_macro2"}" deps)
      (crates."quote"."${deps."palette_derive"."0.5.0"."quote"}" deps)
      (crates."syn"."${deps."palette_derive"."0.5.0"."syn"}" deps)
    ]);
    features = mkFeatures (features."palette_derive"."0.5.0" or {});
  };
  features_."palette_derive"."0.5.0" = deps: f: updateFeatures f (rec {
    palette_derive."0.5.0".default = (f.palette_derive."0.5.0".default or true);
    proc_macro2."${deps.palette_derive."0.5.0".proc_macro2}".default = true;
    quote."${deps.palette_derive."0.5.0".quote}".default = true;
    syn = fold recursiveUpdate {} [
      { "${deps.palette_derive."0.5.0".syn}"."extra-traits" = true; }
      { "${deps.palette_derive."0.5.0".syn}".default = true; }
    ];
  }) [
    (f: if deps."palette_derive"."0.5.0" ? "proc_macro2" then features_.proc_macro2."${deps."palette_derive"."0.5.0"."proc_macro2" or ""}" deps f else f)
    (f: if deps."palette_derive"."0.5.0" ? "quote" then features_.quote."${deps."palette_derive"."0.5.0"."quote" or ""}" deps f else f)
    (f: if deps."palette_derive"."0.5.0" ? "syn" then features_.syn."${deps."palette_derive"."0.5.0"."syn" or ""}" deps f else f)
  ];


# end

# phf-0.8.0

  crates.phf."0.8.0" = deps: { features?(features_."phf"."0.8.0" deps {}) }: buildRustCrate {
    crateName = "phf";
    version = "0.8.0";
    description = "Runtime support for perfect hash function data structures";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    edition = "2018";
    sha256 = "132y7n7rg85ynsdr3fr3za8nipkgilw1l3nfi301lcg5b90nzh83";
    libPath = "src/lib.rs";
    dependencies = mapFeatures features ([
      (crates."phf_shared"."${deps."phf"."0.8.0"."phf_shared"}" deps)
    ]);
    features = mkFeatures (features."phf"."0.8.0" or {});
  };
  features_."phf"."0.8.0" = deps: f: updateFeatures f (rec {
    phf = fold recursiveUpdate {} [
      { "0.8.0"."phf_macros" =
        (f.phf."0.8.0"."phf_macros" or false) ||
        (f.phf."0.8.0"."macros" or false) ||
        (phf."0.8.0"."macros" or false); }
      { "0.8.0"."proc-macro-hack" =
        (f.phf."0.8.0"."proc-macro-hack" or false) ||
        (f.phf."0.8.0"."macros" or false) ||
        (phf."0.8.0"."macros" or false); }
      { "0.8.0"."std" =
        (f.phf."0.8.0"."std" or false) ||
        (f.phf."0.8.0"."default" or false) ||
        (phf."0.8.0"."default" or false); }
      { "0.8.0".default = (f.phf."0.8.0".default or true); }
    ];
    phf_shared = fold recursiveUpdate {} [
      { "${deps.phf."0.8.0".phf_shared}"."std" =
        (f.phf_shared."${deps.phf."0.8.0".phf_shared}"."std" or false) ||
        (phf."0.8.0"."std" or false) ||
        (f."phf"."0.8.0"."std" or false); }
      { "${deps.phf."0.8.0".phf_shared}"."unicase" =
        (f.phf_shared."${deps.phf."0.8.0".phf_shared}"."unicase" or false) ||
        (phf."0.8.0"."unicase" or false) ||
        (f."phf"."0.8.0"."unicase" or false); }
      { "${deps.phf."0.8.0".phf_shared}".default = true; }
    ];
  }) [
    (f: if deps."phf"."0.8.0" ? "phf_shared" then features_.phf_shared."${deps."phf"."0.8.0"."phf_shared" or ""}" deps f else f)
  ];


# end

# phf_codegen-0.8.0

  crates.phf_codegen."0.8.0" = deps: { features?(features_."phf_codegen"."0.8.0" deps {}) }: buildRustCrate {
    crateName = "phf_codegen";
    version = "0.8.0";
    description = "Codegen library for PHF types";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    edition = "2018";
    sha256 = "1vz53m8sl7d8nxb8l62v2nlxay9dhc89qnv3v0scwqz7v9q9rxhm";
    dependencies = mapFeatures features ([
      (crates."phf_generator"."${deps."phf_codegen"."0.8.0"."phf_generator"}" deps)
      (crates."phf_shared"."${deps."phf_codegen"."0.8.0"."phf_shared"}" deps)
    ]);
  };
  features_."phf_codegen"."0.8.0" = deps: f: updateFeatures f (rec {
    phf_codegen."0.8.0".default = (f.phf_codegen."0.8.0".default or true);
    phf_generator."${deps.phf_codegen."0.8.0".phf_generator}".default = true;
    phf_shared."${deps.phf_codegen."0.8.0".phf_shared}".default = true;
  }) [
    (f: if deps."phf_codegen"."0.8.0" ? "phf_generator" then features_.phf_generator."${deps."phf_codegen"."0.8.0"."phf_generator" or ""}" deps f else f)
    (f: if deps."phf_codegen"."0.8.0" ? "phf_shared" then features_.phf_shared."${deps."phf_codegen"."0.8.0"."phf_shared" or ""}" deps f else f)
  ];


# end

# phf_generator-0.8.0

  crates.phf_generator."0.8.0" = deps: { features?(features_."phf_generator"."0.8.0" deps {}) }: buildRustCrate {
    crateName = "phf_generator";
    version = "0.8.0";
    description = "PHF generation logic";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    edition = "2018";
    sha256 = "0i1hwbyahljk1nm8r3jg8ddkvqp213lqiqy3vnwqy43ycyn9r93c";
    crateBin =
      (if (features."phf_generator"."0.8.0"."criterion" or false) then [{  name = "gen_hash_test"; } ] else []);
    dependencies = mapFeatures features ([
      (crates."phf_shared"."${deps."phf_generator"."0.8.0"."phf_shared"}" deps)
      (crates."rand"."${deps."phf_generator"."0.8.0"."rand"}" deps)
    ]);
  };
  features_."phf_generator"."0.8.0" = deps: f: updateFeatures f (rec {
    phf_generator."0.8.0".default = (f.phf_generator."0.8.0".default or true);
    phf_shared."${deps.phf_generator."0.8.0".phf_shared}".default = true;
    rand = fold recursiveUpdate {} [
      { "${deps.phf_generator."0.8.0".rand}"."small_rng" = true; }
      { "${deps.phf_generator."0.8.0".rand}".default = true; }
    ];
  }) [
    (f: if deps."phf_generator"."0.8.0" ? "phf_shared" then features_.phf_shared."${deps."phf_generator"."0.8.0"."phf_shared" or ""}" deps f else f)
    (f: if deps."phf_generator"."0.8.0" ? "rand" then features_.rand."${deps."phf_generator"."0.8.0"."rand" or ""}" deps f else f)
  ];


# end

# phf_shared-0.8.0

  crates.phf_shared."0.8.0" = deps: { features?(features_."phf_shared"."0.8.0" deps {}) }: buildRustCrate {
    crateName = "phf_shared";
    version = "0.8.0";
    description = "Support code shared by PHF libraries";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    edition = "2018";
    sha256 = "00l8xp1bf10kcssmm0hlllh766qsfx61s05jh87q20qjlinjyan1";
    libPath = "src/lib.rs";
    dependencies = mapFeatures features ([
      (crates."siphasher"."${deps."phf_shared"."0.8.0"."siphasher"}" deps)
    ]);
    features = mkFeatures (features."phf_shared"."0.8.0" or {});
  };
  features_."phf_shared"."0.8.0" = deps: f: updateFeatures f (rec {
    phf_shared = fold recursiveUpdate {} [
      { "0.8.0"."std" =
        (f.phf_shared."0.8.0"."std" or false) ||
        (f.phf_shared."0.8.0"."default" or false) ||
        (phf_shared."0.8.0"."default" or false); }
      { "0.8.0".default = (f.phf_shared."0.8.0".default or true); }
    ];
    siphasher."${deps.phf_shared."0.8.0".siphasher}".default = true;
  }) [
    (f: if deps."phf_shared"."0.8.0" ? "siphasher" then features_.siphasher."${deps."phf_shared"."0.8.0"."siphasher" or ""}" deps f else f)
  ];


# end

# ppv-lite86-0.2.6

  crates.ppv_lite86."0.2.6" = deps: { features?(features_."ppv_lite86"."0.2.6" deps {}) }: buildRustCrate {
    crateName = "ppv-lite86";
    version = "0.2.6";
    description = "Implementation of the crypto-simd API for x86";
    authors = [ "The CryptoCorrosion Contributors" ];
    edition = "2018";
    sha256 = "1mlbp0713frbyvcbjmc5vl062b0vr58agkv3ar2qqi5plgy9b7ib";
    features = mkFeatures (features."ppv_lite86"."0.2.6" or {});
  };
  features_."ppv_lite86"."0.2.6" = deps: f: updateFeatures f (rec {
    ppv_lite86 = fold recursiveUpdate {} [
      { "0.2.6"."simd" =
        (f.ppv_lite86."0.2.6"."simd" or false) ||
        (f.ppv_lite86."0.2.6"."default" or false) ||
        (ppv_lite86."0.2.6"."default" or false); }
      { "0.2.6"."std" =
        (f.ppv_lite86."0.2.6"."std" or false) ||
        (f.ppv_lite86."0.2.6"."default" or false) ||
        (ppv_lite86."0.2.6"."default" or false); }
      { "0.2.6".default = (f.ppv_lite86."0.2.6".default or true); }
    ];
  }) [];


# end

# proc-macro2-1.0.8

  crates.proc_macro2."1.0.8" = deps: { features?(features_."proc_macro2"."1.0.8" deps {}) }: buildRustCrate {
    crateName = "proc-macro2";
    version = "1.0.8";
    description = "A stable implementation of the upcoming new `proc_macro` API. Comes with an
option, off by default, to also reimplement itself in terms of the upstream
unstable API.
";
    homepage = "https://github.com/alexcrichton/proc-macro2";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    edition = "2018";
    sha256 = "1xly6h56wzyg4lazds659vzxfj8g3fd392jjpzh6fh8a7dp1w543";
    dependencies = mapFeatures features ([
      (crates."unicode_xid"."${deps."proc_macro2"."1.0.8"."unicode_xid"}" deps)
    ]);
    features = mkFeatures (features."proc_macro2"."1.0.8" or {});
  };
  features_."proc_macro2"."1.0.8" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "1.0.8"."proc-macro" =
        (f.proc_macro2."1.0.8"."proc-macro" or false) ||
        (f.proc_macro2."1.0.8"."default" or false) ||
        (proc_macro2."1.0.8"."default" or false); }
      { "1.0.8".default = (f.proc_macro2."1.0.8".default or true); }
    ];
    unicode_xid."${deps.proc_macro2."1.0.8".unicode_xid}".default = true;
  }) [
    (f: if deps."proc_macro2"."1.0.8" ? "unicode_xid" then features_.unicode_xid."${deps."proc_macro2"."1.0.8"."unicode_xid" or ""}" deps f else f)
  ];


# end

# quote-1.0.2

  crates.quote."1.0.2" = deps: { features?(features_."quote"."1.0.2" deps {}) }: buildRustCrate {
    crateName = "quote";
    version = "1.0.2";
    description = "Quasi-quoting macro quote!(...)";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    edition = "2018";
    sha256 = "0r7030w7dymarn92gjgm02hsm04fwsfs6f1l20wdqiyrm9z8rs5q";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."quote"."1.0.2"."proc_macro2"}" deps)
    ]);
    features = mkFeatures (features."quote"."1.0.2" or {});
  };
  features_."quote"."1.0.2" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.quote."1.0.2".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.quote."1.0.2".proc_macro2}"."proc-macro" or false) ||
        (quote."1.0.2"."proc-macro" or false) ||
        (f."quote"."1.0.2"."proc-macro" or false); }
      { "${deps.quote."1.0.2".proc_macro2}".default = (f.proc_macro2."${deps.quote."1.0.2".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "1.0.2"."proc-macro" =
        (f.quote."1.0.2"."proc-macro" or false) ||
        (f.quote."1.0.2"."default" or false) ||
        (quote."1.0.2"."default" or false); }
      { "1.0.2".default = (f.quote."1.0.2".default or true); }
    ];
  }) [
    (f: if deps."quote"."1.0.2" ? "proc_macro2" then features_.proc_macro2."${deps."quote"."1.0.2"."proc_macro2" or ""}" deps f else f)
  ];


# end

# rand-0.7.3

  crates.rand."0.7.3" = deps: { features?(features_."rand"."0.7.3" deps {}) }: buildRustCrate {
    crateName = "rand";
    version = "0.7.3";
    description = "Random number generators and other randomness functionality.
";
    homepage = "https://crates.io/crates/rand";
    authors = [ "The Rand Project Developers" "The Rust Project Developers" ];
    edition = "2018";
    sha256 = "1amg6qj53ylq3ix22n27kmj1gyj6i15my36mkayr30ndymny0b8r";
    dependencies = mapFeatures features ([
      (crates."rand_core"."${deps."rand"."0.7.3"."rand_core"}" deps)
    ]
      ++ (if features.rand."0.7.3".rand_pcg or false then [ (crates.rand_pcg."${deps."rand"."0.7.3".rand_pcg}" deps) ] else []))
      ++ (if !(kernel == "emscripten") then mapFeatures features ([
      (crates."rand_chacha"."${deps."rand"."0.7.3"."rand_chacha"}" deps)
    ]) else [])
      ++ (if kernel == "emscripten" then mapFeatures features ([
      (crates."rand_hc"."${deps."rand"."0.7.3"."rand_hc"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
    ]
      ++ (if features.rand."0.7.3".libc or false then [ (crates.libc."${deps."rand"."0.7.3".libc}" deps) ] else [])) else []);
    features = mkFeatures (features."rand"."0.7.3" or {});
  };
  features_."rand"."0.7.3" = deps: f: updateFeatures f (rec {
    rand = fold recursiveUpdate {} [
      { "0.7.3"."alloc" =
        (f.rand."0.7.3"."alloc" or false) ||
        (f.rand."0.7.3"."std" or false) ||
        (rand."0.7.3"."std" or false); }
      { "0.7.3"."getrandom" =
        (f.rand."0.7.3"."getrandom" or false) ||
        (f.rand."0.7.3"."std" or false) ||
        (rand."0.7.3"."std" or false); }
      { "0.7.3"."getrandom_package" =
        (f.rand."0.7.3"."getrandom_package" or false) ||
        (f.rand."0.7.3"."getrandom" or false) ||
        (rand."0.7.3"."getrandom" or false); }
      { "0.7.3"."libc" =
        (f.rand."0.7.3"."libc" or false) ||
        (f.rand."0.7.3"."std" or false) ||
        (rand."0.7.3"."std" or false); }
      { "0.7.3"."packed_simd" =
        (f.rand."0.7.3"."packed_simd" or false) ||
        (f.rand."0.7.3"."simd_support" or false) ||
        (rand."0.7.3"."simd_support" or false); }
      { "0.7.3"."rand_pcg" =
        (f.rand."0.7.3"."rand_pcg" or false) ||
        (f.rand."0.7.3"."small_rng" or false) ||
        (rand."0.7.3"."small_rng" or false); }
      { "0.7.3"."simd_support" =
        (f.rand."0.7.3"."simd_support" or false) ||
        (f.rand."0.7.3"."nightly" or false) ||
        (rand."0.7.3"."nightly" or false); }
      { "0.7.3"."std" =
        (f.rand."0.7.3"."std" or false) ||
        (f.rand."0.7.3"."default" or false) ||
        (rand."0.7.3"."default" or false); }
      { "0.7.3".default = (f.rand."0.7.3".default or true); }
    ];
    rand_chacha."${deps.rand."0.7.3".rand_chacha}".default = (f.rand_chacha."${deps.rand."0.7.3".rand_chacha}".default or false);
    rand_core = fold recursiveUpdate {} [
      { "${deps.rand."0.7.3".rand_core}"."alloc" =
        (f.rand_core."${deps.rand."0.7.3".rand_core}"."alloc" or false) ||
        (rand."0.7.3"."alloc" or false) ||
        (f."rand"."0.7.3"."alloc" or false); }
      { "${deps.rand."0.7.3".rand_core}"."getrandom" =
        (f.rand_core."${deps.rand."0.7.3".rand_core}"."getrandom" or false) ||
        (rand."0.7.3"."getrandom" or false) ||
        (f."rand"."0.7.3"."getrandom" or false); }
      { "${deps.rand."0.7.3".rand_core}"."std" =
        (f.rand_core."${deps.rand."0.7.3".rand_core}"."std" or false) ||
        (rand."0.7.3"."std" or false) ||
        (f."rand"."0.7.3"."std" or false); }
      { "${deps.rand."0.7.3".rand_core}".default = true; }
    ];
    rand_hc."${deps.rand."0.7.3".rand_hc}".default = true;
  }) [
    (f: if deps."rand"."0.7.3" ? "libc" then recursiveUpdate f { libc."${deps."rand"."0.7.3"."libc"}"."default" = false; } else f)
    (f: if deps."rand"."0.7.3" ? "rand_pcg" then recursiveUpdate f { rand_pcg."${deps."rand"."0.7.3"."rand_pcg"}"."default" = true; } else f)
    (f: if deps."rand"."0.7.3" ? "rand_core" then features_.rand_core."${deps."rand"."0.7.3"."rand_core" or ""}" deps f else f)
    (f: if deps."rand"."0.7.3" ? "rand_pcg" then features_.rand_pcg."${deps."rand"."0.7.3"."rand_pcg" or ""}" deps f else f)
    (f: if deps."rand"."0.7.3" ? "rand_chacha" then features_.rand_chacha."${deps."rand"."0.7.3"."rand_chacha" or ""}" deps f else f)
    (f: if deps."rand"."0.7.3" ? "rand_hc" then features_.rand_hc."${deps."rand"."0.7.3"."rand_hc" or ""}" deps f else f)
    (f: if deps."rand"."0.7.3" ? "libc" then features_.libc."${deps."rand"."0.7.3"."libc" or ""}" deps f else f)
  ];


# end

# rand_chacha-0.2.1

  crates.rand_chacha."0.2.1" = deps: { features?(features_."rand_chacha"."0.2.1" deps {}) }: buildRustCrate {
    crateName = "rand_chacha";
    version = "0.2.1";
    description = "ChaCha random number generator
";
    homepage = "https://crates.io/crates/rand_chacha";
    authors = [ "The Rand Project Developers" "The Rust Project Developers" "The CryptoCorrosion Contributors" ];
    edition = "2018";
    sha256 = "0zpp3wmxhhmripb6bywhzhx5rfwl4dfbny85hpalwdj0sncv0p0k";
    dependencies = mapFeatures features ([
      (crates."c2_chacha"."${deps."rand_chacha"."0.2.1"."c2_chacha"}" deps)
      (crates."rand_core"."${deps."rand_chacha"."0.2.1"."rand_core"}" deps)
    ]);
    features = mkFeatures (features."rand_chacha"."0.2.1" or {});
  };
  features_."rand_chacha"."0.2.1" = deps: f: updateFeatures f (rec {
    c2_chacha = fold recursiveUpdate {} [
      { "${deps.rand_chacha."0.2.1".c2_chacha}"."simd" = true; }
      { "${deps.rand_chacha."0.2.1".c2_chacha}"."std" =
        (f.c2_chacha."${deps.rand_chacha."0.2.1".c2_chacha}"."std" or false) ||
        (rand_chacha."0.2.1"."std" or false) ||
        (f."rand_chacha"."0.2.1"."std" or false); }
      { "${deps.rand_chacha."0.2.1".c2_chacha}".default = (f.c2_chacha."${deps.rand_chacha."0.2.1".c2_chacha}".default or false); }
    ];
    rand_chacha = fold recursiveUpdate {} [
      { "0.2.1"."simd" =
        (f.rand_chacha."0.2.1"."simd" or false) ||
        (f.rand_chacha."0.2.1"."default" or false) ||
        (rand_chacha."0.2.1"."default" or false); }
      { "0.2.1"."std" =
        (f.rand_chacha."0.2.1"."std" or false) ||
        (f.rand_chacha."0.2.1"."default" or false) ||
        (rand_chacha."0.2.1"."default" or false); }
      { "0.2.1".default = (f.rand_chacha."0.2.1".default or true); }
    ];
    rand_core."${deps.rand_chacha."0.2.1".rand_core}".default = true;
  }) [
    (f: if deps."rand_chacha"."0.2.1" ? "c2_chacha" then features_.c2_chacha."${deps."rand_chacha"."0.2.1"."c2_chacha" or ""}" deps f else f)
    (f: if deps."rand_chacha"."0.2.1" ? "rand_core" then features_.rand_core."${deps."rand_chacha"."0.2.1"."rand_core" or ""}" deps f else f)
  ];


# end

# rand_core-0.5.1

  crates.rand_core."0.5.1" = deps: { features?(features_."rand_core"."0.5.1" deps {}) }: buildRustCrate {
    crateName = "rand_core";
    version = "0.5.1";
    description = "Core random number generator traits and tools for implementation.
";
    homepage = "https://crates.io/crates/rand_core";
    authors = [ "The Rand Project Developers" "The Rust Project Developers" ];
    edition = "2018";
    sha256 = "19qfnh77bzz0x2gfsk91h0gygy0z1s5l3yyc2j91gmprq60d6s3r";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.rand_core."0.5.1".getrandom or false then [ (crates.getrandom."${deps."rand_core"."0.5.1".getrandom}" deps) ] else []));
    features = mkFeatures (features."rand_core"."0.5.1" or {});
  };
  features_."rand_core"."0.5.1" = deps: f: updateFeatures f (rec {
    getrandom."${deps.rand_core."0.5.1".getrandom}"."std" =
        (f.getrandom."${deps.rand_core."0.5.1".getrandom}"."std" or false) ||
        (rand_core."0.5.1"."std" or false) ||
        (f."rand_core"."0.5.1"."std" or false);
    rand_core = fold recursiveUpdate {} [
      { "0.5.1"."alloc" =
        (f.rand_core."0.5.1"."alloc" or false) ||
        (f.rand_core."0.5.1"."std" or false) ||
        (rand_core."0.5.1"."std" or false); }
      { "0.5.1"."getrandom" =
        (f.rand_core."0.5.1"."getrandom" or false) ||
        (f.rand_core."0.5.1"."std" or false) ||
        (rand_core."0.5.1"."std" or false); }
      { "0.5.1"."serde" =
        (f.rand_core."0.5.1"."serde" or false) ||
        (f.rand_core."0.5.1"."serde1" or false) ||
        (rand_core."0.5.1"."serde1" or false); }
      { "0.5.1".default = (f.rand_core."0.5.1".default or true); }
    ];
  }) [
    (f: if deps."rand_core"."0.5.1" ? "getrandom" then recursiveUpdate f { getrandom."${deps."rand_core"."0.5.1"."getrandom"}"."default" = true; } else f)
    (f: if deps."rand_core"."0.5.1" ? "getrandom" then features_.getrandom."${deps."rand_core"."0.5.1"."getrandom" or ""}" deps f else f)
  ];


# end

# rand_hc-0.2.0

  crates.rand_hc."0.2.0" = deps: { features?(features_."rand_hc"."0.2.0" deps {}) }: buildRustCrate {
    crateName = "rand_hc";
    version = "0.2.0";
    description = "HC128 random number generator
";
    homepage = "https://crates.io/crates/rand_hc";
    authors = [ "The Rand Project Developers" ];
    edition = "2018";
    sha256 = "0592q9kqcna9aiyzy6vp3fadxkkbpfkmi2cnkv48zhybr0v2yf01";
    dependencies = mapFeatures features ([
      (crates."rand_core"."${deps."rand_hc"."0.2.0"."rand_core"}" deps)
    ]);
  };
  features_."rand_hc"."0.2.0" = deps: f: updateFeatures f (rec {
    rand_core."${deps.rand_hc."0.2.0".rand_core}".default = true;
    rand_hc."0.2.0".default = (f.rand_hc."0.2.0".default or true);
  }) [
    (f: if deps."rand_hc"."0.2.0" ? "rand_core" then features_.rand_core."${deps."rand_hc"."0.2.0"."rand_core" or ""}" deps f else f)
  ];


# end

# rand_pcg-0.2.1

  crates.rand_pcg."0.2.1" = deps: { features?(features_."rand_pcg"."0.2.1" deps {}) }: buildRustCrate {
    crateName = "rand_pcg";
    version = "0.2.1";
    description = "Selected PCG random number generators
";
    homepage = "https://crates.io/crates/rand_pcg";
    authors = [ "The Rand Project Developers" ];
    edition = "2018";
    sha256 = "04yzci1dbsp2n404iyhzi4dk50cplfw8g9si4xibxdcz062nbmh0";
    dependencies = mapFeatures features ([
      (crates."rand_core"."${deps."rand_pcg"."0.2.1"."rand_core"}" deps)
    ]);
    features = mkFeatures (features."rand_pcg"."0.2.1" or {});
  };
  features_."rand_pcg"."0.2.1" = deps: f: updateFeatures f (rec {
    rand_core."${deps.rand_pcg."0.2.1".rand_core}".default = true;
    rand_pcg = fold recursiveUpdate {} [
      { "0.2.1"."serde" =
        (f.rand_pcg."0.2.1"."serde" or false) ||
        (f.rand_pcg."0.2.1"."serde1" or false) ||
        (rand_pcg."0.2.1"."serde1" or false); }
      { "0.2.1".default = (f.rand_pcg."0.2.1".default or true); }
    ];
  }) [
    (f: if deps."rand_pcg"."0.2.1" ? "rand_core" then features_.rand_core."${deps."rand_pcg"."0.2.1"."rand_core" or ""}" deps f else f)
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

# siphasher-0.3.1

  crates.siphasher."0.3.1" = deps: { features?(features_."siphasher"."0.3.1" deps {}) }: buildRustCrate {
    crateName = "siphasher";
    version = "0.3.1";
    description = "SipHash-2-4, SipHash-1-3 and 128-bit variants in pure Rust";
    homepage = "https://docs.rs/siphasher";
    authors = [ "Frank Denis <github@pureftpd.org>" ];
    sha256 = "1ivwckrsjw5rq4yzvpqp11jcw4awr3b51nrwjvd9g815dgyx9f69";
  };
  features_."siphasher"."0.3.1" = deps: f: updateFeatures f (rec {
    siphasher."0.3.1".default = (f.siphasher."0.3.1".default or true);
  }) [];


# end

# syn-1.0.14

  crates.syn."1.0.14" = deps: { features?(features_."syn"."1.0.14" deps {}) }: buildRustCrate {
    crateName = "syn";
    version = "1.0.14";
    description = "Parser for Rust source code";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    edition = "2018";
    sha256 = "1z8bbg1s2ixyw7q877yzcgdnzh80n6vk14c1k0dznwd7rff0m66w";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."syn"."1.0.14"."proc_macro2"}" deps)
      (crates."unicode_xid"."${deps."syn"."1.0.14"."unicode_xid"}" deps)
    ]
      ++ (if features.syn."1.0.14".quote or false then [ (crates.quote."${deps."syn"."1.0.14".quote}" deps) ] else []));
    features = mkFeatures (features."syn"."1.0.14" or {});
  };
  features_."syn"."1.0.14" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.syn."1.0.14".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.syn."1.0.14".proc_macro2}"."proc-macro" or false) ||
        (syn."1.0.14"."proc-macro" or false) ||
        (f."syn"."1.0.14"."proc-macro" or false); }
      { "${deps.syn."1.0.14".proc_macro2}".default = (f.proc_macro2."${deps.syn."1.0.14".proc_macro2}".default or false); }
    ];
    quote."${deps.syn."1.0.14".quote}"."proc-macro" =
        (f.quote."${deps.syn."1.0.14".quote}"."proc-macro" or false) ||
        (syn."1.0.14"."proc-macro" or false) ||
        (f."syn"."1.0.14"."proc-macro" or false);
    syn = fold recursiveUpdate {} [
      { "1.0.14"."clone-impls" =
        (f.syn."1.0.14"."clone-impls" or false) ||
        (f.syn."1.0.14"."default" or false) ||
        (syn."1.0.14"."default" or false); }
      { "1.0.14"."derive" =
        (f.syn."1.0.14"."derive" or false) ||
        (f.syn."1.0.14"."default" or false) ||
        (syn."1.0.14"."default" or false); }
      { "1.0.14"."parsing" =
        (f.syn."1.0.14"."parsing" or false) ||
        (f.syn."1.0.14"."default" or false) ||
        (syn."1.0.14"."default" or false); }
      { "1.0.14"."printing" =
        (f.syn."1.0.14"."printing" or false) ||
        (f.syn."1.0.14"."default" or false) ||
        (syn."1.0.14"."default" or false); }
      { "1.0.14"."proc-macro" =
        (f.syn."1.0.14"."proc-macro" or false) ||
        (f.syn."1.0.14"."default" or false) ||
        (syn."1.0.14"."default" or false); }
      { "1.0.14"."quote" =
        (f.syn."1.0.14"."quote" or false) ||
        (f.syn."1.0.14"."printing" or false) ||
        (syn."1.0.14"."printing" or false); }
      { "1.0.14".default = (f.syn."1.0.14".default or true); }
    ];
    unicode_xid."${deps.syn."1.0.14".unicode_xid}".default = true;
  }) [
    (f: if deps."syn"."1.0.14" ? "quote" then recursiveUpdate f { quote."${deps."syn"."1.0.14"."quote"}"."default" = false; } else f)
    (f: if deps."syn"."1.0.14" ? "proc_macro2" then features_.proc_macro2."${deps."syn"."1.0.14"."proc_macro2" or ""}" deps f else f)
    (f: if deps."syn"."1.0.14" ? "quote" then features_.quote."${deps."syn"."1.0.14"."quote" or ""}" deps f else f)
    (f: if deps."syn"."1.0.14" ? "unicode_xid" then features_.unicode_xid."${deps."syn"."1.0.14"."unicode_xid" or ""}" deps f else f)
  ];


# end

# unicode-xid-0.2.0

  crates.unicode_xid."0.2.0" = deps: { features?(features_."unicode_xid"."0.2.0" deps {}) }: buildRustCrate {
    crateName = "unicode-xid";
    version = "0.2.0";
    description = "Determine whether characters have the XID_Start
or XID_Continue properties according to
Unicode Standard Annex #31.
";
    homepage = "https://github.com/unicode-rs/unicode-xid";
    authors = [ "erick.tryzelaar <erick.tryzelaar@gmail.com>" "kwantam <kwantam@gmail.com>" ];
    sha256 = "1c85gb3p3qhbjvfyjb31m06la4f024jx319k10ig7n47dz2fk8v7";
    features = mkFeatures (features."unicode_xid"."0.2.0" or {});
  };
  features_."unicode_xid"."0.2.0" = deps: f: updateFeatures f (rec {
    unicode_xid."0.2.0".default = (f.unicode_xid."0.2.0".default or true);
  }) [];


# end

# wasi-0.9.0+wasi-snapshot-preview1

  crates.wasi."0.9.0+wasi-snapshot-preview1" = deps: { features?(features_."wasi"."0.9.0+wasi-snapshot-preview1" deps {}) }: buildRustCrate {
    crateName = "wasi";
    version = "0.9.0+wasi-snapshot-preview1";
    description = "Experimental WASI API bindings for Rust";
    authors = [ "The Cranelift Project Developers" ];
    edition = "2018";
    sha256 = "0xa6b3rnsmhi13nvs9q51wmavx51yzs5qdbc7bvs0pvs6iar3hsd";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."wasi"."0.9.0+wasi-snapshot-preview1" or {});
  };
  features_."wasi"."0.9.0+wasi-snapshot-preview1" = deps: f: updateFeatures f (rec {
    wasi = fold recursiveUpdate {} [
      { "0.9.0+wasi-snapshot-preview1"."compiler_builtins" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."compiler_builtins" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false); }
      { "0.9.0+wasi-snapshot-preview1"."core" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."core" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false); }
      { "0.9.0+wasi-snapshot-preview1"."rustc-std-workspace-alloc" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."rustc-std-workspace-alloc" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false); }
      { "0.9.0+wasi-snapshot-preview1"."std" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."std" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1"."default" or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."default" or false); }
      { "0.9.0+wasi-snapshot-preview1".default = (f.wasi."0.9.0+wasi-snapshot-preview1".default or true); }
    ];
  }) [];


# end

  crates.approx.default = crates.approx."0.3.2";
  features_.approx.default = features_.approx."0.3.2";

  crates.autocfg.default = crates.autocfg."1.0.0";
  features_.autocfg.default = features_.autocfg."1.0.0";

  crates.bitflags.default = crates.bitflags."1.2.1";
  features_.bitflags.default = features_.bitflags."1.2.1";

  crates.c2_chacha.default = crates.c2_chacha."0.2.3";
  features_.c2_chacha.default = features_.c2_chacha."0.2.3";

  crates.cfg_if.default = crates.cfg_if."0.1.10";
  features_.cfg_if.default = features_.cfg_if."0.1.10";

  crates.exit.default = crates.exit."0.1.0";
  features_.exit.default = features_.exit."0.1.0";

  crates.getrandom.default = crates.getrandom."0.1.14";
  features_.getrandom.default = features_.getrandom."0.1.14";

  crates.lazy_static.default = crates.lazy_static."1.4.0";
  features_.lazy_static.default = features_.lazy_static."1.4.0";

  crates.libc.default = crates.libc."0.2.66";
  features_.libc.default = features_.libc."0.2.66";

  crates.num_complex.default = crates.num_complex."0.2.4";
  features_.num_complex.default = features_.num_complex."0.2.4";

  crates.num_traits.default = crates.num_traits."0.2.11";
  features_.num_traits.default = features_.num_traits."0.2.11";

  crates.palette.default = crates.palette."0.5.0";
  features_.palette.default = features_.palette."0.5.0";

  crates.palette_derive.default = crates.palette_derive."0.5.0";
  features_.palette_derive.default = features_.palette_derive."0.5.0";

  crates.phf.default = crates.phf."0.8.0";
  features_.phf.default = features_.phf."0.8.0";

  crates.phf_codegen.default = crates.phf_codegen."0.8.0";
  features_.phf_codegen.default = features_.phf_codegen."0.8.0";

  crates.phf_generator.default = crates.phf_generator."0.8.0";
  features_.phf_generator.default = features_.phf_generator."0.8.0";

  crates.phf_shared.default = crates.phf_shared."0.8.0";
  features_.phf_shared.default = features_.phf_shared."0.8.0";

  crates.ppv_lite86.default = crates.ppv_lite86."0.2.6";
  features_.ppv_lite86.default = features_.ppv_lite86."0.2.6";

  crates.proc_macro2.default = crates.proc_macro2."1.0.8";
  features_.proc_macro2.default = features_.proc_macro2."1.0.8";

  crates.quote.default = crates.quote."1.0.2";
  features_.quote.default = features_.quote."1.0.2";

  crates.rand.default = crates.rand."0.7.3";
  features_.rand.default = features_.rand."0.7.3";

  crates.rand_chacha.default = crates.rand_chacha."0.2.1";
  features_.rand_chacha.default = features_.rand_chacha."0.2.1";

  crates.rand_core.default = crates.rand_core."0.5.1";
  features_.rand_core.default = features_.rand_core."0.5.1";

  crates.rand_hc.default = crates.rand_hc."0.2.0";
  features_.rand_hc.default = features_.rand_hc."0.2.0";

  crates.rand_pcg.default = crates.rand_pcg."0.2.1";
  features_.rand_pcg.default = features_.rand_pcg."0.2.1";

  crates.sdl_mandelbrot.default = crates.sdl_mandelbrot."0.1.0";
  features_.sdl_mandelbrot.default = features_.sdl_mandelbrot."0.1.0";

  crates.sdl2.default = crates.sdl2."0.33.0";
  features_.sdl2.default = features_.sdl2."0.33.0";

  crates.sdl2_sys.default = crates.sdl2_sys."0.33.0";
  features_.sdl2_sys.default = features_.sdl2_sys."0.33.0";

  crates.siphasher.default = crates.siphasher."0.3.1";
  features_.siphasher.default = features_.siphasher."0.3.1";

  crates.syn.default = crates.syn."1.0.14";
  features_.syn.default = features_.syn."1.0.14";

  crates.unicode_xid.default = crates.unicode_xid."0.2.0";
  features_.unicode_xid.default = features_.unicode_xid."0.2.0";

  crates.wasi.default = crates.wasi."0.9.0+wasi-snapshot-preview1";
  features_.wasi.default = features_.wasi."0.9.0+wasi-snapshot-preview1";

}
