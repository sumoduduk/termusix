{
  description = "Build termusix app with flake.nix";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (localSystem: {
      packages = let
        pkgs = nixpkgs.legacyPackages.${localSystem};

        definetermusixPkgs = {}:
          {
            termusix_aarch64-linux = import ./nix/cross-build.nix {
              inherit localSystem inputs;
              pathCwd = ./.;
              crossSystem = "aarch64-linux";
              rustTargetTriple = "aarch64-unknown-linux-gnu";
            };

            termusix_x86_64-linux = import ./nix/cross-build.nix {
              inherit localSystem inputs;
              pathCwd = ./.;
              crossSystem = "x86_64-linux";
              rustTargetTriple = "x86_64-unknown-linux-gnu";
            };

            termusix_x86_64-windows = import ./nix/window-build.nix {
              inherit localSystem inputs;
              pathCwd = ./.;
            };

            termusix-pkgbuild = pkgs.callPackage ./nix/pkgbuild.nix {
              termusix = self.packages.${localSystem}.termusix_x86_64-linux;
            };

            get_termusix_version =
              pkgs.runCommand "get_termusix_version" {
              } ''
                mkdir -p $out
                echo ${self.packages.${localSystem}.termusix_x86_64-linux.version} > $out/version.txt
              '';

            termusix-docker = import ./nix/docker_build.nix {
              inherit localSystem inputs;
              termusix = self.packages.${localSystem}.termusix_x86_64-linux;
            };
          }
          // (
            if localSystem == "aarch64-darwin"
            then {
              termusix_aarch64-apple = import ./nix/cross-build.nix {
                inherit localSystem inputs;
                pathCwd = ./.;
                crossSystem = "aarch64-darwin";
                rustTargetTriple = "aarch64-apple-darwin";
              };

              tar-darwin-arm = pkgs.callPackage ./nix/tar-package.nix {
                termusix = self.packages.${localSystem}.termusix_aarch64-apple;
                architecture = "arm";
              };

              get_termusix_version_darwin =
                pkgs.runCommand "get_termusix_version_darwin" {
                } ''
                  mkdir -p $out
                    echo ${self.packages.${localSystem}.termusix_aarch64-apple.version} > $out/version.txt
                '';
            }
            else if localSystem == "x86_64-darwin"
            then {
              termusix_x86_64-apple = import ./nix/cross-build.nix {
                inherit localSystem inputs;
                pathCwd = ./.;
                crossSystem = "x86_64-darwin";
                rustTargetTriple = "x86_64-apple-darwin";
              };

              tar-darwin-x86_64 = pkgs.callPackage ./nix/tar-package.nix {
                termusix = self.packages.${localSystem}.termusix_x86_64-apple;
                architecture = "intel";
              };
            }
            else {}
          );
      in
        definetermusixPkgs {};

      apps.default = flake-utils.lib.mkApp {
        drv = nixpkgs.lib.getAttr "termusix_${localSystem}" self.packages.${localSystem};
      };

      devShells.default = let
        pkgs = nixpkgs.legacyPackages.${localSystem};
      in
        pkgs.mkShell {
          packages = with pkgs; [
            patchelf
          ];
        };
    })
    // {
      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;
    };
}
