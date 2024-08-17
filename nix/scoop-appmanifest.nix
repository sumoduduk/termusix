{
  runCommand,
  termusix,
}: let
  pkgdesc = "A terminal-based music player with a user-friendly terminal UI, built with Rust.";
  version = termusix.version;

  appManifest = ''
    {
     "version" : "${version}",
     "description": "${pkgdesc}",
     "homepage": "https://github.com/sumoduduk/termusix",
     "license": "GPL-3.0",
     "architecture": {
        "64bit": {
          "url": "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-x86_64-windows.exe#/termusix.exe",
          "hash": "%%SHA256SUM%%"
        }
      },
     "bin": "termusix.exe",
     "checkver": "github",
     "autoupdate": {
        "architecture": {
          "64bit": {
            "url": "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-x86_64-windows.exe#/termusix.exe"
          }
        }
     }
    }
  '';
in
  runCommand "termusix-scoop" {inherit appManifest termusix;} ''
    mkdir -p $out

    sha256=$(sha256sum $termusix/bin/termusix.exe | awk '{print $1}')

    echo "$appManifest" | sed "s/%%SHA256SUM%%/$sha256/" > $out/termusix.json
  ''
