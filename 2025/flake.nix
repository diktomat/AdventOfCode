{
  description = "Advent of Code 2025";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.*";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    overlays = [
      (import rust-overlay)
      (final: prev: {
        rustToolchain = prev.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      })
    ];

    allSystems = [
      "aarch64-darwin"
    ];

    forAllSystems = f:
      nixpkgs.lib.genAttrs allSystems (
        system:
          f {
            pkgs = import nixpkgs {inherit overlays system;};
          }
      );
  in {
    devShells = forAllSystems (
      {pkgs}: {
        default = pkgs.mkShell {
          packages =
            (with pkgs; [
              rustToolchain
              bacon
              cargo-nextest
              just
            ])
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [libiconv]);
        };
      }
    );
  };
}
