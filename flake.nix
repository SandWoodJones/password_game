{
  description = "A password game implementation in rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }  {
      systems = [ "x86_64-linux" ];
      perSystem = { pkgs, inputs', config, ... }:
      let
        stdenv' = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
        naersk' = pkgs.callPackage inputs.naersk { stdenv = stdenv'; };
      in {
        packages.default = naersk'.buildPackage {
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
