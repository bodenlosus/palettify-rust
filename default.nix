{ lib
, stdenv
, rustPlatform
, glib
, llvmPackages
, pkg-config
, ffmpeg
, rustc
, cargo
}:
rustPlatform.buildRustPackage rec {
  pname = "palettify-rust";
  version = "0.0.1";

  src = ./.;

  buildInputs = [
    glib
    ffmpeg
    rustPlatform.bindgenHook
  ];

  nativeBuildInputs = [
    rustPlatform.bindgenHook
    pkg-config
    rustc
    cargo
  ];
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  cargoHash = lib.fakeHash;

  cargoBuildOptions = [
        "--release-lto"
  ];

  LIBCLANG_PATH="${llvmPackages.libclang.lib}";

  meta = with lib; {
    homepage = "";
    description = "Program for applying palettes";
    license = licenses.mit;
  };
}
