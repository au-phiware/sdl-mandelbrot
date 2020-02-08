with (import ./base.nix);
(cargo.sdl_mandelbrot {}).override {
  inherit rust;
  crateOverrides = defaultCrateOverrides // {
    "sdl-mandelbrot" = attrs: { buildInputs = [ SDL2 ]; };
  };
}
