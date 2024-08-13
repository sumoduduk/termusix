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
          pkgs.darwin.apple_sdk.frameworks.CoreMIDI
          pkgs.darwin.apple_sdk.frameworks.AudioUnit
          pkgs.darwin.apple_sdk.frameworks.OpenAL
          pkgs.darwin.apple_sdk.frameworks.AppKit
        ];

      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = lib.optionalString stdenv.isLinux "${stdenv.cc.targetPrefix}cc";
      CARGO_TARGET_AARCH64_UNKNOWN_APPLE_LINKER = lib.optionalString stdenv.isDarwin "${stdenv.cc.targetPrefix}cc";

      # COREAUDIO_SDK_PATH = lib.optionalString stdenv.isDarwin "${pkgs.darwin.apple_sdk.frameworks.CoreAudio}/Library/Frameworks/CoreAudio.framework/Headers";

      # BINDGEN_EXTRA_CLANG_ARGS = lib.optionalString stdenv.isDarwin (builtins.concatStringsSep " " [
      #   "-I${pkgs.darwin.apple_sdk.frameworks.CoreAudio}/Library/Frameworks/CoreAudio.framework/Headers"
      #   "-I${pkgs.darwin.apple_sdk.frameworks.AudioUnit}/Library/Frameworks/AudioUnit.framework/Headers"
      #   "-F${pkgs.darwin.apple_sdk.frameworks.CoreAudio}/Library/Frameworks"
      #   "-F${pkgs.darwin.apple_sdk.frameworks.AudioUnit}/Library/Frameworks"
      # ]);

      cargoExtraArgs = "--target ${rustTargetTriple}";

      HOST_CC = "${stdenv.cc.nativePrefix}cc";
      TARGET_CC = "${stdenv.cc.targetPrefix}cc";
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
