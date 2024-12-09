{
  description = "SimpleFOC-rs flake";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    esp-rs-nix = {
      url = "github:crabdancing/esp-rs-nix";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        # To import a flake module
        # 1. Add foo to inputs
        # 2. Add foo as a parameter to the outputs function
        # 3. Add here: foo.flakeModule
      ];
      # TODO: make work on systems other than x86_64-linux.
      # - Requires adjustments to upstream `esp-rs-nix`
      # - Possibly requires setting up VMs / acquiring devices compatible with various systems
      systems = ["x86_64-linux"]; # "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        # config,
        # self',
        # inputs',
        pkgs,
        lib,
        system,
        ...
      }: let
        esp-rust = inputs.esp-rs-nix.packages.${pkgs.system}.default;
        # craneLib = inputs.crane.mkLib pkgs;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain esp-rust;
        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;

          nativeBuildInputs = [
            pkgs.gnumake
            pkgs.cargo-make
            pkgs.espflash
          ];

          buildInputs =
            [
              # Add additional build inputs here
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        # craneLib =
        #   craneLib.overrideToolchain
        #   esp-rust;

        # Build *just* the cargo dependencies (of the entire workspace),
        # so we can reuse all of that work (e.g. via cachix) when running in CI
        # It is *highly* recommended to use something like cargo-hakari to avoid
        # cache misses when building individual top-level-crates
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs =
          commonArgs
          // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
            # NB: we disable tests since we'll run them all via cargo-nextest
            doCheck = false;
          };

        fileSetForCrate = crate:
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneLib.fileset.commonCargoSources ./core)
              (craneLib.fileset.commonCargoSources ./spinnies/generic_spinny)
              (craneLib.fileset.commonCargoSources ./spinnies/esp32)
              (craneLib.fileset.commonCargoSources ./peripherals/AS5047P)
              (craneLib.fileset.commonCargoSources crate)
            ];
          };

        sfoc_rs = craneLib.buildPackage (individualCrateArgs
          // {
            pname = "sfoc_rs";
            src = fileSetForCrate ./.;
            # doCheck = true;
            # checkPhase = ''
            #   makers check
            # '';
            # buildPhase = ''
            #   makers
            # '';
          });

        esp32_sfoc = craneLib.buildPackage (individualCrateArgs
          // {
            pname = "esp32_sfoc";
            cargoExtraArgs = "-p esp32_sfoc -Z build-std";

            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              inherit (craneLib.findCargoFiles src) cargoConfigs;
              cargoLockList = [
                ./Cargo.lock

                # Unfortunately this approach requires IFD (import-from-derivation)
                # otherwise Nix will refuse to read the Cargo.lock from our toolchain
                # (unless we build with `--impure`).
                #
                # Another way around this is to manually copy the rustlib `Cargo.lock`
                # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
                # will avoid IFD entirely but will require manually keeping the file
                # up to date!
                "${esp-rust}/lib/rustlib/src/rust/Cargo.lock"
                # "/nix/store/d9r076qnij4h0b1dhm2hq1dchb59k1sp-esp-rs/lib/rustlib/src/rust/library/Cargo.lock"
              ];
            };
            src = fileSetForCrate ./platforms/esp32_sfoc;

            postPatch = ''
              echo code running
              cd platforms/esp32_sfoc
            '';
          });

        # Build the top-level crates of the workspace as individual derivations.
        # This allows consumers to only depend on (and build) only what they need.
        # Though it is possible to build the entire workspace as a single derivation,
        # so this is left up to you on how to organize things
        #
        # Note that the cargo workspace must define `workspace.members` using wildcards,
        # otherwise, omitting a crate (like we do below) will result in errors since
        # cargo won't be able to find the sources for all members.
        spinnies = craneLib.buildPackage (individualCrateArgs
          // {
            pname = "spinnies";
            cargoExtraArgs = "-p spinnies";
            src = fileSetForCrate ./spinnies;
          });
      in rec {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };

        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        devShells.default = devShells.simplefocRsDev;
        devShells.simplefocRsDev = pkgs.mkShell {
          buildInputs =
            (with pkgs; [
              openssl
              pkg-config
              eza
              fd
              cargo-make
              gnumake
              espflash
            ])
            ++ [
              esp-rust
            ];

          shellHook = ''
            export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
          '';
        };

        # packages.spinnies = spinnies;
        packages.default = sfoc_rs;
        packages.esp32_sfoc = esp32_sfoc;
      };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
