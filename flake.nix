{
  description = "A password game implementation in rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts.url = "github:hercules-ci/flake-parts";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }  {
      systems = import inputs.systems;
      perSystem = { pkgs, inputs', system, config, ... }:
      let
        stdenv' = if (system == "x86_64-linux") then pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv
                  else pkgs.stdenv;

        naersk = pkgs.callPackage inputs.naersk { stdenv = stdenv'; };
      in {
        packages.default = naersk.buildPackage {
          src = ./.;
        };

        devShells.default = pkgs.mkShell.override { stdenv = stdenv'; } {
          buildInputs = with pkgs; [ rust-analyzer clippy ]
            ++ config.packages.default.nativeBuildInputs
            ++ config.packages.default.buildInputs;
        };
      };
    };
}
