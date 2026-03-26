{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell rec {

  packages = with pkgs; [
    trunk
  ];

  buildInputs = with pkgs; [
    pkg-config
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libxcb
    libxkbcommon
    vulkan-loader
    wayland
    openssl
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
  '';
}
