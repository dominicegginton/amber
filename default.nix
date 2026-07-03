{ lib
, rustPlatform
, rustfmt
}:

rustPlatform.buildRustPackage {
  pname = "amber";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  cargoPackageName = "amber";
  nativeBuildInputs = [ rustfmt ];
  meta = {
    description = "Small Tools - Beautiful Terminals";
    homepage = "https://github.com/dominicegginton/amber";
    platforms = lib.platforms.all;
  };
}
