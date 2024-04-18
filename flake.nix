{
  inputs = {
    nixpkgs.url = "github:NixOs/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  # inputs.flake-utils.inputs.nixpkgs.follows = "nixpkgs";

  outputs = {
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      py = pkgs.python312.withPackages (p: [p.numpy]);

      rust = pkgs.rust-bin.stable.latest.default;
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          py
          rust
        ];
      };
    });
}
