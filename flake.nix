{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShells.default =
          with pkgs;
          mkShell rec {
            buildInputs = [
              # Linux
              openssl
              pkg-config
              alsa-lib
              udev
              # Wayland
              libxkbcommon
              wayland
              # Xorg
              xorg.libX11
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              # Vulkan
              vulkan-tools
              vulkan-loader
              vulkan-validation-layers
              # Linkers
              mold
              clang
              # Rust
              cargo-watch
              rust-analyzer
              rust-bin.nightly.latest.default
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
      }
    );
}
