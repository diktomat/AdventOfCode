{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    allSystems = [
      "aarch64-darwin"
    ];

    forAllSystems = f:
      nixpkgs.lib.genAttrs allSystems (
        system:
          f {
            pkgs = import nixpkgs {inherit system;};
          }
      );
  in {
    devShells = forAllSystems (
      {pkgs}: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            just
            ruby_3_4
            rubyPackages_3_4.solargraph
          ];
        };
      }
    );
  };
}
