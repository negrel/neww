{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # NUR Rust toolchains and rust analyzer nightly for nix.
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-utils, nixpkgs, fenix, self, ... }@inputs:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ fenix.overlays.default ];
          };
          lib = pkgs.lib;

          pkgBuildInputs = with pkgs; [
            pkg-config
            gtk4
            luajit
          ] ++ [ self.packages."${system}".gtk4-layer-shell ];
        in
        {
          devShells = {
            default = pkgs.mkShell rec {
              buildInputs = with pkgs; [ ] ++ pkgBuildInputs ++ (
                with pkgs.fenix; [
                  complete.toolchain
                  rust-analyzer-nightly
                ]
              );
              LD_LIBRARY_PATH = "${lib.makeLibraryPath pkgBuildInputs}";
            };
          };
          packages = {
            gtk4-layer-shell = pkgs.callPackage ./nix/gtk4-layer-shell.nix { };
          };
        });
}

