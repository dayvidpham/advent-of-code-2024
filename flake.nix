{
  inputs = {
    nixpkgs.url = "github:NixOS/Nixpkgs/nixos-24.11";
    nixpkgs-unstable.url = "github:NixOS/Nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs-unstable";
    };
  };

  outputs =
    { self
    , nixpkgs
    , nixpkgs-unstable
    , flake-utils
    , fenix
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      lib = pkgs.lib;

      fenixPkgs = fenix.packages."${system}";
      rust-channel = fenixPkgs.default;
      rust-toolchain = rust-channel.toolchain;
      rust-stdlib = rust-channel.rust-std;

      shell = pkgs.mkShell {
        packages = [
          rust-toolchain
          fenixPkgs.rust-analyzer
        ];
        RUST_SRC_PATH = "${rust-stdlib}/lib/rustlib/x86_64-unknown-linux-gnu/lib";

        allowSubstitutes = false;
      };
    in
    {
      devShells = {
        default = shell;
      };
    });
}
