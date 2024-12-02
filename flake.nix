{
  description = "SimpleFOC-rs flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    esp-rs-nix = {
      url = "github:crabdancing/esp-rs-nix";
    };
    nixpkgs-esp-dev = {
      url = "github:mirrexagon/nixpkgs-esp-dev";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {...} @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        esp-rust = inputs.esp-rs-nix.packages.${pkgs.system}.default;
        overlays = [(import inputs.rust-overlay)];
        pkgs = import inputs.nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs =
            (with pkgs; [
              openssl
              pkg-config
              eza
              fd
              cargo-make
            ])
            ++ [
              esp-rust
            ];

          shellHook = ''
            export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
          '';
        };
      }
    );
}
