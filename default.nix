with (import <nixpkgs> {});
stdenv.mkDerivation {
  name = "sdl-mandelbrot";
  src = ./.;
  doCheck = true;
  doInstallCheck = true;
  buildInputs = [ SDL2 ];
  system = "x86_64-linux";
}
