{
  craneLib,
  installShellFiles,
  stdenv,
  buildPackages,
  lib,
}: let
  commonArgs = {
    src = craneLib.cleanCargoSource ./.;
    strictDeps = true;

    doCheck = false;

    nativeBuildInputs = [installShellFiles];

    inherit (craneLib.crateNameFromCargoToml {src = ./crates/unitypkg-cli;}) pname version;

    postInstall = let
      unitypkg-cli = "${stdenv.hostPlatform.emulator buildPackages} $out/bin/unitypkg-cli";
    in
      lib.optionalString (stdenv.hostPlatform.emulatorAvailable buildPackages) ''
        installShellCompletion --cmd unitypkg-cli \
          --bash <(${unitypkg-cli} completions bash) \
          --fish <(${unitypkg-cli} completions fish) \
          --zsh <(${unitypkg-cli} completions zsh)
      '';
  };
in
  craneLib.buildPackage (commonArgs
    // {
      meta = with lib; {
        description = "Manipulate Unity's portable package files";
        homepage = "https://github.com/bddvlpr/unitypkg";
        license = licenses.mit;
        maintainers = with maintainers; [bddvlpr];
        mainProgram = "unitypkg-cli";
      };
    })
