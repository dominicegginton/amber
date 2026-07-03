{ lib
, rustPlatform
, rustfmt
}:

rustPlatform.buildRustPackage {
  pname = "amber";
  version = "0.1.0";
  src = lib.source.clearSources ./.;
  cargoLock.lockFile = ./Cargo.lock;
  cargoPackageName = "amber";
  nativeBuildInputs = [ rustfmt ];
  meta = {
    description = "Beautiful terminals";
    homepage = "https://github.com/dominicegginton/amber";
    platforms = lib.platforms.all;
  };
}
