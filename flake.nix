{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) { inherit system; };
        stdenv_mold = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;

        naersk' = pkgs.callPackage naersk { stdenv = stdenv_mold; };
      in rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;

          nativeBuildInputs = [ pkgs.clang pkgs.mold ];
          RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=mold";
        };

        devShell = pkgs.mkShell.override { stdenv = stdenv_mold; } {
          RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=mold";
          buildInputs = with pkgs; [ rust-analyzer ]
            ++ defaultPackage.nativeBuildInputs
            ++ defaultPackage.buildInputs;
        };
      }
    );
}
