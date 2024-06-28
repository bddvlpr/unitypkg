{
  rustPlatform,
  installShellFiles,
  stdenv,
  buildPackages,
  lib,
}: let
  inherit ((builtins.fromTOML (builtins.readFile ./unitypkg-cli/Cargo.toml)).package) version;
in
  rustPlatform.buildRustPackage {
    pname = "unitypkg-cli";
    inherit version;

    src = ./.;

    nativeBuildInputs = [installShellFiles];

    postInstall = let
      unitypkg-cli = "${stdenv.hostPlatform.emulator buildPackages} $out/bin/unitypkg-cli";
    in
      lib.optionalString (stdenv.hostPlatform.emulatorAvailable buildPackages) ''
        installShellCompletion --cmd unitypkg-cli \
          --bash <(${unitypkg-cli} completions bash) \
          --fish <(${unitypkg-cli} completions fish) \
          --zsh <(${unitypkg-cli} completions zsh)
      '';

    cargoHash = "sha256-7Qutyozuo9RR+Trl63G/kmYXNdH7NhB0ewppTHpT6z8=";

    meta = with lib; {
      description = "Manipulate Unity's portable package files";
      homepage = "https://github.com/bddvlpr/unitypkg";
      license = licenses.mit;
      maintainers = with maintainers; [bddvlpr];
      mainProgram = "unitypkg-cli";
    };
  }
