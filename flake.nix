{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest;

        rustPlatform =
          let
            rustVersion = rust.default.override {
              # include source for LSP
              extensions = [ "rust-src" "rustfmt" ];
            };
          in
          pkgs.makeRustPlatform {
            cargo = rustVersion;
            rustc = rustVersion;
          };

        brag =
          let
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
          rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
      in
      {
        devShells.default = pkgs.mkShell {
          inputsFrom = [ brag ];
          buildInputs = with pkgs; [
            eza
            fd
            rust.rust-analyzer
          ];
        };

        packages.default = brag;
      }
    );
}
