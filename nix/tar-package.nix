{
  runCommand,
  termusix,
  architecture,
  ...
}: let
  version = termusix.version;
in
  runCommand "tar-the-file" {inherit version;} ''
    mkdir -p $out
    tar -czvf $out/termusix-v$version-mac-${architecture}.tar.gz -C ${termusix}/bin termusix
  ''
