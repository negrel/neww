{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    let
      outputsWithoutSystem = { };
      outputsWithSystem = flake-utils.lib.eachDefaultSystem
        (system:
          let
            pkgs = import nixpkgs { inherit system; };
            luaPkgs = pkgs.luajitPackages;
            lib = pkgs.lib;
            buildInputs = with pkgs; [
              luajit
              luajitPackages.luarocks
              luajitPackages.luacheck
              pkg-config
              gobject-introspection
              gtk4
              cairo
            ] ++ (with self.packages."${system}"; [ lgi gtk4-layer-shell ]);
          in
          {
            devShells = {
              default = pkgs.mkShell {
                inherit buildInputs;
                LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
                LUA_PATH = "src/?/init.lua;src/?.lua;src/?;.venv/share/lua/5.1/?.lua;${pkgs.luajit}/share/lua/5.1/?.lua;";
                LUA_CPATH = "${self.packages.${system}.lgi}/lib/lua/5.1/?.so";
              };
            };
            packages = rec {
              lgi = pkgs.callPackage ./nix/lgi.nix {
                inherit (luaPkgs) lua buildLuaPackage;
                inherit pkgs;
              };
              gtk4-layer-shell = pkgs.callPackage ./nix/gtk4-layer-shell.nix { };
            };
          });
    in
    outputsWithSystem // outputsWithoutSystem;
}
