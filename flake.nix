{
  inputs = {
    nixpkgs.url = "github:NixOs/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    flake-utils,
    nixpkgs,
    rust-overlay,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (localSystem: let
      overlays = [(import rust-overlay)];
      crossSystem = "aarch64-linux";
      pkgs = import nixpkgs {inherit overlays localSystem;};
      crossPkgs = import nixpkgs {
        inherit localSystem crossSystem overlays;
      };
      py = pkgs.python312.withPackages (p: [p.numpy]);

      rust = crossPkgs.rust-bin.stable.latest.default.override {
        targets = [
          "aarch64-unknown-linux-gnu"
        ];
      };
      craneLib = (crane.mkLib pkgs).overrideToolchain rust;
      src = craneLib.cleanCargoSource (craneLib.path ./.);
      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src;
      };
      crate = craneLib.buildPackage {
        inherit src cargoArtifacts;
      };
    in {
      checks = {
        inherit crate;
      };
      packages = {
        default = crate;
        docs = craneLib.cargoDoc {
          inherit src cargoArtifacts;
        };
        cross = pkgs.callPackage ./default.nix {inherit craneLib;};
      };
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          py
          rust
          rust-analyzer
        ];
      };
    });
}
