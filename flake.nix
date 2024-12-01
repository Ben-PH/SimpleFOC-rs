{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    esp32 = {
      url = "github:knarkzel/esp32";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, esp32, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        idf-rust = esp32.packages.x86_64-linux.esp32;
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            idf-rust
            rust-bin.stable.latest.default
            cargo-make



          ];

          shellHook = ''
            export PATH="${idf-rust}/.rustup/toolchains/esp/bin:$PATH"
            export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
          '';
        };
      }
    );
}

