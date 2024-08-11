{
  runCommand,
  termusixArm,
  termusixIntel,
}: let
  version = termusixArm.version;

  rb_build = ''
    class Termusix < Formula
      desc "Blazingly fast and safe utility written in Rust for reorganizing folders by grouping files based on their extensions."
      homepage "https://github.com/sumoduduk/termusix"
      version "${version}"

      on_macos do
        on_intel do
          url "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-v${version}-mac-intel.tar.gz"
          sha256 "%%SHA256SUMINTEL%%"
        end

        on_arm do
          url "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-v${version}-mac-arm.tar.gz"
          sha256 "%%SHA256SUMARM%%"
        end
      end

      def install
        bin.install "termusix"
      end
    end
  '';
in
  runCommand "package-homebrew" {inherit rb_build;} ''
    # tar the termusixArm
    tar -czvf termusix-arm.tar.gz -C ${termusixArm}/bin termusix

    # tar the termusixIntel
    tar -czvf termusix-intel.tar.gz -C ${termusixIntel}/bin termusix

    # calculate the sha256sum for termusixArm
    sha256_arm=$(sha256sum termusix-arm.tar.gz | awk '{ print $1 }')

    # calculate the sha256sum for termusixIntel
    sha256_intel=$(sha256sum termusix-intel.tar.gz | awk '{ print $1 }')

    # sed the shasum in rb build and save to out
    mkdir -p $out
    echo "$rb_build" | sed "s/%%SHA256SUMARM%%/$sha256_arm/" | sed "s/%%SHA256SUMINTEL%%/$sha256_intel/" > $out/termusix.rb
  ''
