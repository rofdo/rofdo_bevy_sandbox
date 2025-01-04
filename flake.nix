{
  description = "Building my rust project";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, flake-utils, nixpkgs, rust-overlay, ... }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
      };
      nativeBuildInputs = with pkgs; [
        pkg-config
        gcc
        rust-bin.stable.latest.default
      ];
      buildInputs = with pkgs; [
        openssl
        xorg.libX11
        alsa-lib
        udev
        libxkbcommon
        wayland
        vulkan-loader
        vulkan-headers
        vulkan-tools
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        mesa
        xorg.libXinerama
        xorg.libXext
        xorg.libXfixes
        xorg.libXtst
      ];
      libraryPath = pkgs.lib.makeLibraryPath buildInputs;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        LD_LIBRARY_PATH = "${libraryPath}:$LD_LIBRARY_PATH";
        inherit buildInputs nativeBuildInputs;
      };
      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "my_bevy_sandbox";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        src = ./.;
        inherit buildInputs nativeBuildInputs;
        
        postFixup = ''
          patchelf --set-rpath "${libraryPath}" $out/bin/my_bevy_sandbox
        '';
      };
      packages.x86_64-linux.server = pkgs.rustPlatform.buildRustPackage {
        pname = "server";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        src = ./.;
        inherit buildInputs nativeBuildInputs;
        
        postFixup = ''
          patchelf --set-rpath "${libraryPath}" $out/bin/my_bevy_sandbox
        '';
      };
      apps.x86_64-linux.default = {
        type = "app";
        program = "${self.packages.x86_64-linux.default}/bin/my_bevy_sandbox";
      };
      apps.x86_64-linux.server = {
        type = "app";
        program = "${self.packages.x86_64-linux.default}/bin/server";
      };
    };
}
