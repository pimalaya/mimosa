{
  nixpkgs ? <nixpkgs>,
  system ? builtins.currentSystem,
  pkgs ? import nixpkgs { inherit system; },
  pimalaya ? import (fetchTarball "https://github.com/pimalaya/nix/archive/master.tar.gz"),
  ...
}@args:

let
  inherit (pkgs) dbus;
  shell = pimalaya.mkShell (removeAttrs args [ "pimalaya" ]);

in
shell.overrideAttrs (prev: {
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    dbus
  ];

  buildInputs = (prev.buildInputs or [ ]) ++ [
    dbus
  ];
})
