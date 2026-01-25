{ stdenv }:

stdenv.mkDerivation {
  pname = "helloworld";
  version = "0.1.0";

  src = ./.;

  makeFlags = [
    "ARCH=x86_64"
    "CROSS_COMPILE=${stdenv.cc.targetPrefix}"
  ];


  installPhase = ''
    mkdir -p $out/bin
    install -m755 helloworld $out/bin/helloworld.elf
  '';

  meta = {
    description = "A hello world app";
    platforms = [ "x86_64-linux" ];
  };
}
