# This file is part of sway-workspace-extras
#
# Copyright (c) 2023 Thomas Himmelstoss
#
# This software is subject to the MIT license. You should have
# received a copy of the license along with this program.

{
  description = "Build sway-workspace-extras crate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, ... }:
    let
      supportedSystems = [ "x86_64-linux" ];

      forSupportedSystems = generator:
        let
          generateForSystem = system: generator {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
            craneLib = crane.lib.${system};
          };
        in
        nixpkgs.lib.genAttrs supportedSystems generateForSystem;
    in
    {
      packages = forSupportedSystems ({ system, pkgs, craneLib }:
        {
          package = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);

            buildInputs = with pkgs.lib; [ ]
              ++ optional pkgs.stdenv.isDarwin pkgs.libiconv;
          };

          default = self.packages.${system}.package;
        });

      overlays.addPackage = final: prev: {
        sway-workspace-extras = self.packages.${prev.system}.default;
      };

      devShells = forSupportedSystems ({ system, pkgs, ... }:
        let
          fix = pkgs.writeShellScriptBin "fix" ''
            cargo fmt
            cargo clippy --fix --allow-dirty --allow-staged --all-targets
          '';

          checkFmt = pkgs.writeShellScriptBin "chkfmt" ''
            cargo fmt --check
          '';

          lint = pkgs.writeShellScriptBin "lint" ''
            cargo clippy --all-targets -- --deny warnings
          '';
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.package ];

            nativeBuildInputs = with pkgs; [
              cargo
              rustc
              clippy
              rustfmt
              fix
              checkFmt
              lint
            ];
          };
        });

      formatter = forSupportedSystems ({ pkgs, ... }: pkgs.nixpkgs-fmt);
    };
}