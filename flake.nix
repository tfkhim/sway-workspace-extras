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
          generateForSystem = system: generator rec {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
            craneLib = crane.mkLib pkgs;
          };
        in
        nixpkgs.lib.genAttrs supportedSystems generateForSystem;
    in
    {
      packages = forSupportedSystems ({ system, pkgs, craneLib }:
        {
          package = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);

            strictDeps = true;

            buildInputs = with pkgs.lib; [ ]
              ++ optional pkgs.stdenv.isDarwin pkgs.libiconv;

            meta = with pkgs.lib; {
              description = "A set of commands to move focus or containers to new Sway workspaces";
              license = licenses.mit;
              platforms = platforms.linux;
              mainProgram = "sway-workspace-extras";
            };
          };

          default = self.packages.${system}.package;
        });

      overlays.addPackage = final: prev: {
        sway-workspace-extras = self.packages.${prev.system}.default;
      };

      devShells = forSupportedSystems ({ system, pkgs, craneLib, ... }:
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
          default = craneLib.devShell {
            inputsFrom = [ self.packages.${system}.package ];

            # This environment variable is required by rust-analyzer
            # to find the source and expand proc macros. See:
            # https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            packages = [
              fix
              checkFmt
              lint
            ];
          };
        });

      formatter = forSupportedSystems ({ pkgs, ... }: pkgs.nixpkgs-fmt);
    };
}
