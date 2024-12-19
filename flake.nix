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
        inherit (pkgs) mkShell;
        inherit (pkgs.lib) optionals makeLibraryPath;
        inherit (pkgs.stdenv) isLinux isDarwin;

        general-deps = with pkgs; [
          # Rust
          cargo-watch
          cargo-expand
          rust-analyzer-unwrapped
          # Toml
          taplo
          # Other
          vulkan-loader
          vulkan-tools
        ];

        linux-deps = with pkgs; [
          # Wayland
          libxkbcommon
          wayland
          # Xorg
          xorg.libX11
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          # Other
          openssl
          pkg-config
          alsa-lib
          udev
          vulkan-validation-layers
        ];

        darwin-deps = with pkgs.darwin.apple_sdk.frameworks; [
          AppKit
          ApplicationServices
          AudioToolbox
          AudioUnit
          Carbon
          CoreAudio
          CoreFoundation
          CoreGraphics
          CoreServices
          CoreVideo
          CoreMIDI
          Foundation
          IOKit
          QuartzCore
          Metal
          OpenAL
          Security
          pkgs.libiconv
        ];

        web-deps = with pkgs; [
          wasm-bindgen-cli
          binaryen
        ];

        extensions = [
          "clippy"
          "rustfmt"
          "rust-src"
        ];
      in
      {
        devShells = {
          # Regular shell
          default =
            let
              toolchain = pkgs.rust-bin.nightly.latest.default.override {
                inherit extensions;
                targets = optionals isDarwin [
                  "x86_64-apple-darwin"
                  "aarch64-apple-darwin"
                ];
              };
              platform = pkgs.makeRustPlatform { inherit (toolchain) cargo rustc; };
            in
            mkShell rec {
              buildInputs =
                [
                  toolchain
                  platform.bindgenHook
                ]
                ++ general-deps
                ++ optionals isLinux linux-deps
                ++ optionals isDarwin darwin-deps;

              RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
              LD_LIBRARY_PATH = makeLibraryPath buildInputs;
              RUSTFLAGS = "-Zshare-generics=y -Zthreads=0";
            };
          # For web builds
          web =
            let
              toolchain = pkgs.rust-bin.nightly.latest.default.override {
                targets = [ "wasm32-unknown-unknown" ];
              };
            in
            mkShell rec {
              buildInputs = [ toolchain ] ++ general-deps ++ web-deps;

              RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
              LD_LIBRARY_PATH = makeLibraryPath buildInputs;
              RUSTFLAGS = "-Zshare-generics=y -Zthreads=0";
            };
        };
      }
    );
}
