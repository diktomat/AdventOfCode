{
  description = "Advent of Code 2022 Day 1";
  inputs.utils.url = "github:numtide/flake-utils";
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
          buildInputs = [pkgs.rustc pkgs.just];
        };
      }
    );
}
