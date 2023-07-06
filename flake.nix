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
          luaPkgs = pkgs.luajitPackages;

          pkgBuildInputs = with pkgs; [
            pkg-config
            gtk4
            luajit
            dbus

            openssl
          ] ++ (with self.packages."${system}"; [ gtk4-layer-shell lgi ]);
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
              LUA_PATH = "${self.packages.${system}.lgi}/share/lua/5.1/?.lua;${self.packages.${system}.lgi}/share/lua/5.1/lgi/?.lua";
              LUA_CPATH = "${self.packages.${system}.lgi}/lib/lua/5.1/?.so";
            };
          };
          packages = {
            lgi = pkgs.callPackage ./nix/lgi.nix {
              inherit (luaPkgs) lua buildLuaPackage;
              inherit pkgs;
            };
            gtk4-layer-shell = pkgs.callPackage ./nix/gtk4-layer-shell.nix { };
          };
        });
}

