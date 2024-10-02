{
  description = "Manipulate Unity's portable package files";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    flake-parts,
    crane,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      perSystem = {pkgs, ...}: let
        craneLib = crane.mkLib pkgs;
      in {
        packages = rec {
          unitypkg = pkgs.callPackage ./. {inherit craneLib;};
          default = unitypkg;
        };

        formatter = pkgs.alejandra;
      };
    };
}
