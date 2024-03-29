{
  inputs = {
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    advisory-db = {
      flake = false;
      url = "github:rustsec/advisory-db";
    };
    ctranslate2-src = {
      flake = false;
      url = "git+https://github.com/OpenNMT/CTranslate2?submodules=1";
    };
    onednn-src = {
      flake = false;
      url = "github:oneapi-src/oneDNN";
    };
  };

  outputs = { self, crane, fenix, flake-utils, nixpkgs, advisory-db, ctranslate2-src, onednn-src }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages.${system};
        craneLib = crane.lib.${system};
        src = craneLib.path ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = with pkgs; rec {
          inherit src;
          strictDeps = true;
          nativeBuildInputs = [ cmake pkg-config ];
          buildInputs = [
            # Add additional build inputs here
            ctranslate2
            fontconfig
            libGL
            libxkbcommon
            openssl
            wayland
          ] ++ (with xorg; [
            libX11
            libXcursor
            libXi
            libXrandr
          ]) ++ lib.optionals stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            libiconv
          ];
          cargoExtraArgs = "--features app";
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          CTRANSLATE2_SRC = ctranslate2-src;
          ONEDNN_SRC = onednn-src;
          CJK_PATH = "${sarasa-gothic}/share/fonts/truetype/Sarasa-Regular.ttc";
        };

        craneLibLLvmTools = craneLib.overrideToolchain
          (fenixPkgs.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]);

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        crate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit crate;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages = {
          default = crate;
        } // pkgs.lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
          my-crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        apps.default = flake-utils.lib.mkApp {
          drv = crate;
        };

        devShells.default = craneLib.devShell (commonArgs // {
          # Inherit inputs from checks.
          # Enable after Cargo.toml and Cargo.lock are present
          checks = self.checks.${system};

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            fenixPkgs.rust-analyzer
            pkgs.xorg.libxcb
          ];
          
          RUST_SRC_PATH = "${fenixPkgs.complete.rust-src}/lib/rustlib/src/rust/library";
        });
      });
}
