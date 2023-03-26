{
  description = "Advent of Code 2022 Day 2";
  inputs.utils.url = "github:numtide/flake-utils";
  inputs.utils.inputs.nixpkgs.follows = "nixpkgs";
  outputs = {
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [pkgs.go pkgs.just];
        };
      }
    );
}
