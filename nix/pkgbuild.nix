{
  runCommand,
  termusix,
  pkgrel ? "1",
}: let
  sha256 = runCommand "termusix-sha256" {} ''
    sha256sum ${termusix}/bin/termusix | awk '{print $1}'
  '';

  pkgdesc = "A terminal-based music player with a user-friendly terminal UI, built with Rust.";

  pkgbuild = ''
    pkgname=termusix-bin
     pkgdesc="${pkgdesc}"
     pkgrel=${pkgrel}
     pkgver=${termusix.version}
     url="https://github.com/sumoduduk/termusix"
     license=("GPL-3.0")
     arch=("x86_64")
     provides=("termusix")
     conflicts=("termusix")
     depends=("alsa-lib" "dbus")
     source=("https://github.com/sumoduduk/termusix/releases/download/v$pkgver/termusix-$CARCH-linux")
     sha256sums=("%%SHA256SUM%%")

     package() {
        mv termusix-x86_64-linux termusix
        install -Dm755 termusix -t "$pkgdir/usr/bin"
     }
  '';

  srcinfo = ''
    pkgbase = termusix-bin
    	pkgdesc = ${pkgdesc}
    	pkgver = ${termusix.version}
    	pkgrel = ${pkgrel}
    	url = https://github.com/sumoduduk/termusix
    	arch = x86_64
    	license = GPL-3.0
    	provides = termusix
    	conflicts = termusix
      depends = alsa-lib
      depends = dbus
    	source = https://github.com/sumoduduk/termusix/releases/download/v${termusix.version}/termusix-x86_64-linux
    	sha256sums = %%SHA256SUM%%

    pkgname = termusix-bin
  '';
in
  runCommand "termusix-bin-aur" {inherit srcinfo pkgbuild;} ''
    mkdir -p $out
    cp ${termusix}/bin/termusix $out/termusix

    chmod 777 $out/termusix
    patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 --set-rpath /lib/x86_64-linux-gnu $out/termusix

    sha256=$(sha256sum $out/termusix | awk '{print $1}')


    echo "$srcinfo" | sed "s/%%SHA256SUM%%/$sha256/" > $out/.SRCINFO
    echo "$pkgbuild" | sed "s/%%SHA256SUM%%/$sha256/" > $out/PKGBUILD
    rm $out/termusix
  ''
