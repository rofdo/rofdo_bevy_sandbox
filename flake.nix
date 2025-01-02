{
  description = "Building my rust project";

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
      buildInputs = with pkgs; [
        openssl
        pkg-config
        rust-bin.stable.latest.default
      ];
    in
    {
      devShells.default = with pkgs; mkShell {
        inherit buildInputs;
      };

      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        cargoLock.lockFile = ./Cargo.lock;
        src = ./.;
        name = "my_bevy_sandbox";
        inherit buildInputs;
      };

      apps.x86_64-linux.default = {
        type = "app";
        program = "${self.packages.x86_64-linux.default}/bin/my_bevy_sandbox";
      };
    };
}

