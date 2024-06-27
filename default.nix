{
  rustPlatform,
  lib,
}: let
  inherit ((builtins.fromTOML (builtins.readFile ./unitypkg-cli/Cargo.toml)).package) version;
in
  rustPlatform.buildRustPackage {
    pname = "unitypkg-cli";
    inherit version;

    src = ./.;

    cargoHash = "sha256-EnfAFP/4cmtUuzmmXn9A6AYG3Yud2zBBYm0ORe282uc=";

    meta = with lib; {
      description = "Manipulate Unity's portable package files";
      homepage = "https://github.com/bddvlpr/unitypkg";
      license = licenses.mit;
      maintainers = with maintainers; [bddvlpr];
      mainProgram = "unitypkg-cli";
    };
  }
