{
  description = "Cross compiling a rust program for windows";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
      };
    in
    {
      devShells.default = with pkgs; mkShell {
        buildInputs = [
          # Openssl is for example required by the `reqwest` crate
          openssl
          pkg-config
          rust-bin.stable.latest.default
        ];
      };

      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        cargoLock.lockFile = ./Cargo.lock;
        src = ./.;
        name = "my-bevy-sandbox";
        buildInputs = with pkgs; [
          openssl
          pkg-config
          rust-bin.stable.latest.default
        ];
      };
    };
}

