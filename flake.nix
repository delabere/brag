{
  description = "A devShell example";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , crane
    , flake-utils
    , nixpkgs
    , rust-overlay
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust = pkgs.rust-bin.stable.latest;
        rustToolchain = rust.default.override {
          extensions = [
            # include source for LSP
            "rust-src"
          ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        brag = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;
        };
      in
      {
        packages.default = brag;
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            eza
            fd
            rust.rust-analyzer
          ];
        };
      }
    );
}
