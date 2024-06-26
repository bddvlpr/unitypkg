{
  rustPlatform,
  lib,
}: let
  inherit ((builtins.fromTOML (builtins.readFile ./Cargo.toml)).package) version;
in
  rustPlatform.buildRustPackage {
    pname = "unitypkg";
    inherit version;

    src = ./.;

    cargoHash = "sha256-HB8We2K0D4xZFhGdy4505dxve2EGbuhJHFSkso7NZEU=";

    meta = with lib; {
      description = "Manipulate Unity's portable package files";
      homepage = "https://github.com/bddvlpr/unitypkg";
      license = licenses.mit;
      maintainers = with maintainers; [bddvlpr];
      mainProgram = "unitypkg";
    };
  }
