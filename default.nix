{ lib
, stdenv
, rustPlatform
, glib
}:
rustPlatform.buildRustPackage rec {
  pname = "palettify-rust";
  version = "0.0.1";

  src = ./.;

  buildInputs = [
    glib
  ];
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  cargoHash = lib.fakeHash;

  cargoBuildOptions = [
        "--release-lto"
  ];

  meta = with lib; {
    homepage = "";
    description = "Program for applying palettes";
    license = licenses.mit;
  };
}
