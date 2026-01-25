{ stdenv }:

stdenv.mkDerivation {
  pname = "syscall_test";
  version = "0.1.0";

  src = ./.;

  makeFlags = [
    "ARCH=x86_64"
    "CROSS_COMPILE=${stdenv.cc.targetPrefix}"
  ];


  installPhase = ''
    mkdir -p $out/bin
    install -m755 syscall_test $out/bin/syscall_test.elf
  '';

  meta = {
    description = "syscall 2333 test";
    platforms = [ "x86_64-linux" ];
  };
}
