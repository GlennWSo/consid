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
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      py = pkgs.python312.withPackages (p: [p.numpy]);

      rust = pkgs.rust-bin.stable.latest.default;
      craneLib = crane.mkLib pkgs;
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
      packages.default = crate;
      packages.docs = craneLib.cargoDoc {
        inherit src cargoArtifacts;
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
