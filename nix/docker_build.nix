{
  inputs,
  termusix,
  localSystem,
}: let
  inherit (inputs) nixpkgs;
  pkgs = nixpkgs.legacyPackages.${localSystem};
in
  pkgs.dockerTools.buildLayeredImage {
    name = "termusix";
    tag = "0.1.0";
    config = {
      Entrypoint = ["${termusix}/bin/termusix"];
      Env = [
        "PATH=${pkgs.coreutils}/bin:${pkgs.bash}/bin:${pkgs.alsa-lib}/bin:${pkgs.dbus}/bin"
      ];
    };
    contents = [
      pkgs.coreutils
      pkgs.bash
      pkgs.alsa-lib
      pkgs.dbus
      pkgs.alsa-utils
      termusix
    ];
  }
