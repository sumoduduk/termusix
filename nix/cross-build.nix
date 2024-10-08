{
  inputs,
  localSystem,
  crossSystem,
  rustTargetTriple,
  pathCwd,
  ...
}: let
  inherit (inputs) nixpkgs crane rust-overlay;

  common = import ./common.nix {};

  pkgs = import nixpkgs {
    inherit crossSystem localSystem;
    overlays = [(import rust-overlay)];
  };

  craneLib = (crane.mkLib pkgs).overrideToolchain (pkgs:
    pkgs.rust-bin.stable.${common.rustVersion}.minimal.override {
      targets = [rustTargetTriple];
    });

  createExpression = {
    lib,
    pkg-config,
    stdenv,
  }: let
    termusixCommon = common.termusix {
      inherit lib craneLib pathCwd;
    };

    commonArgs = {
      pname = "default";
      version = "0.0.0";

      src = termusixCommon.src;

      doCheck = false;
      strictDeps = true;

      nativeBuildInputs =
        [
          stdenv.cc
          pkg-config
        ]
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.rustPlatform.bindgenHook
        ];

      buildInputs =
        [
          #pkgs.openssl
          pkgs.dbus
        ]
        ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
          pkgs.alsa-lib
        ]
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
          pkgs.darwin.apple_sdk.frameworks.Foundation
          # pkgs.darwin.apple_sdk.frameworks.Security
          # pkgs.darwin.apple_sdk.frameworks.IOKit
          pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          pkgs.darwin.apple_sdk.frameworks.CoreServices
          pkgs.darwin.apple_sdk.frameworks.CoreAudio
          pkgs.darwin.apple_sdk.frameworks.AudioToolbox
          # pkgs.darwin.apple_sdk.frameworks.CoreMIDI
          # pkgs.darwin.apple_sdk.frameworks.IOKit
          pkgs.darwin.apple_sdk.frameworks.AudioUnit
        ];

      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
      CARGO_TARGET_AARCH64_UNKNOWN_APPLE_LINKER = "${stdenv.cc.targetPrefix}cc";
      # COREAUDIO_SDK_PATH = "${pkgs.darwin.apple_sdk.path}/System/Library/Frameworks/CoreAudio.framework/Versions/A/Headers";

      cargoExtraArgs = "--target ${rustTargetTriple}";

      HOST_CC = "${stdenv.cc.nativePrefix}cc";
      TARGET_CC = "${stdenv.cc.targetPrefix}cc";

      NIX_OUTPATH_USED_AS_RANDOM_SEED = "aaaaaaaaaa";
    };

    cargoArtifacts = craneLib.buildDepsOnly commonArgs;
  in
    craneLib.buildPackage (commonArgs
      // {
        pname = "termusix";
        version = termusixCommon.crateInfo.version;

        inherit cargoArtifacts;

        cargoExtraArgs = "${commonArgs.cargoExtraArgs} --package termusix";
      });
in
  pkgs.callPackage createExpression {}
