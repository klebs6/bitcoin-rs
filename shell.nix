# Native libraries for the existing Rust toolchain.
# Usage from this checkout: nix-shell --run make
let
  nixpkgs = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/d233902339c02a9c334e7e593de68855ad26c4cb.tar.gz";
    sha256 = "sha256-30sZNZoA1cqF5JNO9fVX+wgiQYjB7HJqqJ4ztCDeBZE=";
  };
  pkgs = import nixpkgs { config = {}; overlays = []; };
in pkgs.mkShell {
  name = "bitcoin-rs";
  nativeBuildInputs = [ pkgs.pkg-config pkgs.gnumake ];
  buildInputs = [ pkgs.libevent pkgs.openssl pkgs.clang ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
