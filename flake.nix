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
        inherit (pkgs) lib mkShell stdenv;

        general-deps = with pkgs; [
          # Vulkan
          vulkan-tools
          vulkan-loader
          vulkan-validation-layers
          # Linkers
          mold
          clang
          # Rust
          cargo-watch
          rust-analyzer-unwrapped
          # Toml
          taplo
        ];

        linux-deps = with pkgs; [
          openssl
          pkg-config
          alsa-lib
          udev
          speechd
          # Wayland
          libxkbcommon
          wayland
          # Xorg
          xorg.libX11
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];

        web-deps = with pkgs; [ wasm-bindgen-cli binaryen ];

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
              toolchain = pkgs.rust-bin.nightly.latest.default.override { inherit extensions; };
            in
            mkShell rec {
              buildInputs = [ toolchain ] ++ general-deps ++ lib.optionals stdenv.isLinux linux-deps;

              # For speechd
              shellHook = ''
                export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
              '';

              RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
              LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
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
              LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
            };
        };
      }
    );
}
