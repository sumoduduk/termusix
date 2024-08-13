{
  inputs,
  termusix,
  localSystem,
}: let
  inherit (inputs) nixpkgs;
  pkgs = nixpkgs.legacyPackages.${localSystem};

  entry_script = pkgs.writeText ''
    PCM_CARD=${PCM_CARD:-0}
    CTL_CARD=${CTL_CARD:-0}

    echo "Configuring ALSA with PCM card $ALSA_PCM_CARD and CTL card $ALSA_CTL_CARD"
    echo "pcm.!default {
        type hw
        card $PCM_CARD
    }

    ctl.!default {
        type hw
        card $CTL_CARD
    }" > /etc/asound.conf

    # Execute the provided command
    exec "$@"
  '';
in
  pkgs.dockerTools.buildImage {
    name = "termusix";
    tag = "latest";
    config = {
      Entrypoint = ["${termusix}/bin/termusix"];
    };
  }
