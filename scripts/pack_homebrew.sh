#!/bin/bash

# Check if all arguments are provided
if [ "$#" -ne 3 ]; then
  echo "Usage: $0 <sha_for_termusixArm> <sha_for_termusixIntel> <version>"
  exit 1
fi

# Assign arguments to variables
termusixArm="$1"
termusixIntel="$2"
version="$3"

# Define rb_build content
rb_build=$(cat <<EOF
class Termusix < Formula
  desc "A terminal-based music player with a user-friendly terminal UI, built with Rust."
  homepage "https://github.com/sumoduduk/termusix"
  version "${version}"

  depends_on "libvorbis" 
  depends_on "libogg"    

  on_macos do
    on_intel do
      url "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-v${version}-mac-intel.tar.gz"
      sha256 "$termusixIntel"
    end

    on_arm do
      url "https://github.com/sumoduduk/termusix/releases/download/v${version}/termusix-v${version}-mac-arm.tar.gz"
      sha256 "$termusixArm"
    end
  end

  def install
    bin.install "termusix"
  end

  test do
    system "#{bin}/termusix", "--version"
  end
end
EOF
)

mkdir -p rb_build
echo "$rb_build" > rb_build/termusix.rb
