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
        naersk' = pkgs.callPackage naersk {};
      in rec {
        defaultPackage = naersk'.buildPackage { src = ./.; };

        devShell = pkgs.mkShell {
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        
          buildInputs = with pkgs; [ rust-analyzer ]
            ++ defaultPackage.nativeBuildInputs
            ++ defaultPackage.buildInputs;
        };
      }
    );
}
