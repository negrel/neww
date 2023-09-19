{ pkgs, fetchFromGitHub, buildLuaPackage, lua }:
buildLuaPackage {
  pname = "lgi";
  version = "0.9.2-1";
  src = pkgs.fetchFromGitHub {
    owner = "lgi-devs";
    repo = "lgi";
    rev = "975737940d4463abc107fc366b9ab817e9217e0b";
    sha256 = "sha256-G13BrqUmHwRtlmBVafo0LiwsX4nL/muw0/9cca+sigg=";
  };

  propagatedBuildInputs = [ lua ] ++ (with pkgs;
    [
      gobject-introspection
      pkg-config
    ]);

  meta = {
    homepage = "http://github.com/pavouk/lgi";
    description = "Lua bindings to GObject libraries";
    license.fullName = "MIT/X11";
  };
}

