{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        brag = pkgs.rustPlatform.buildRustPackage rec {
          pname = "brag";
          version = "0.1.0";
          src = ./brag;
          cargoSha256 = "sha256-WTqLemoZ5qqdk3RBQJSoY+U/sD5Urtugl2zp9ptmS+A="; # Replace with actual hash
          # buildInputs = [pkgs.openssl pkgs.pkgconfig];
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              openssl
              pkg-config
              eza
              fd
              rust-bin.stable.latest.default
            ];
          };

          packages.default = brag;
        }
    );
}
