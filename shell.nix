with (import ./base.nix);
stdenv.mkDerivation {
  name = "sdl-mandelbrot-shell";
  buildInputs = [
    rust
    rustChannel.cargo
    SDL2
  ];
}
