{
  inputs,
  localSystem,
  pathCwd,
}: let
  inherit (inputs) nixpkgs crane fenix;
  common = import ./common.nix {};

  pkgs = nixpkgs.legacyPackages.${localSystem};

  toolchain = with fenix.packages.${localSystem};
    combine [
      minimal.rustc
      minimal.cargo
      targets.x86_64-pc-windows-gnu.latest.rust-std
    ];

  craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

  termusixCommon = common.termusix {
    inherit craneLib pathCwd;
    lib = pkgs.lib;
  };

  commonArgs = {
    pname = "default";
    version = "0.0.0";

    src = termusixCommon.src;

    strictDeps = true;
    doCheck = false;

    CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";

    TARGET_CC = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";

    RUSTC_LINKER = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";

    depsBuildBuild = with pkgs; [
      pkgsCross.mingwW64.stdenv.cc
      pkgsCross.mingwW64.windows.pthreads
      # pkgsCross.mingwW64.windows.windows_sdk
    ];

    # buildInputs = with pkgs; [
    # pkgsCross.mingwW64.windows.advapi32
    # pkgsCross.mingwW64.windows.shell32
    # pkgsCross.mingwW64.windows.user32
    # pkgsCross.mingwW64.windows.kernel32
    # pkgsCross.mingwW64.windows.gdi32
    # pkgsCross.mingwW64.windows.comdlg32
    # pkgsCross.mingwW64.windows.comctl32
    # pkgsCross.mingwW64.windows.ws2_32
    # pkgsCross.mingwW64.windows.winmm # For audio playback
    # ];
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
  craneLib.buildPackage (commonArgs
    // {
      pname = "termusix";
      version = termusixCommon.crateInfo.version;

      inherit cargoArtifacts;

      cargoExtraArgs = "--package termusix";
    })
