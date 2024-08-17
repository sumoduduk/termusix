{
  runCommand,
  termusix,
}: let
  sha256 = runCommand "termusix-sha256" {} ''
    sha256sum ${termusix}/bin/termusix | awk '{print $1}'
  '';

  pkgdesc = "A terminal-based music player with a user-friendly terminal UI, built with Rust.";
  version = termusix.version;

  appManifest = builtins.toJSON {
    description = pkgdesc;
    homepage = "https://github.com/sumoduduk/termusix";
    license = "GPL-3.0";
    architecture = {
      "64bit" = {
        url = "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-x86_64-windows.exe";
        hash = "%%SHA256SUM%%";
      };
    };
    bin = "termusix.exe";
    checkver = "github";
    autoupdate = {
      architecture = {
        "64bit" = {
          url = "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-x86_64-windows.exe";
        };
      };
    };
  };
in
  runCommand "termusix-scoop" {inherit appManifest;} ''
    mkdir -p $out

    sha256=$(sha256sum $out/termusix | awk '{print $1}')

    echo "$appManifest" | sed "s/%%SHA256SUM%%/$sha256/" > $out/termusix.json
  ''
