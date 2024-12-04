{
  inputs = {
    nixpkgs.url = "github:NixOS/Nixpkgs/nixos-24.11";
    nixpkgs-unstable.url = "github:NixOS/Nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , nixpkgs-unstable
    , flake-utils
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      lib = pkgs.lib;

      shell = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.rustc
        ];
      };
    in
    {
      devShells = {
        default = shell;
      };
    });
}
