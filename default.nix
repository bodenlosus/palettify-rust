{ lib
, stdenv
, rustPlatform
, glib
, llvmPackages
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

  LIBCLANG_PATH="${llvmPackages.libclang}";

  meta = with lib; {
    homepage = "";
    description = "Program for applying palettes";
    license = licenses.mit;
  };
}
