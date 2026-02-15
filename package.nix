# TODO: move this to nixpkgs
# This file aims to be a replacement for the nixpkgs derivation.

{
  buildFeatures ? [ ],
  buildNoDefaultFeatures ? false,
  buildPackages,
  dbus,
  fetchFromGitHub,
  installManPages ? stdenv.buildPlatform.canExecute stdenv.hostPlatform,
  installShellCompletions ? stdenv.buildPlatform.canExecute stdenv.hostPlatform,
  installShellFiles,
  lib,
  openssl,
  pkg-config,
  rustPlatform,
  stdenv,
}:

let
  version = "0.1.0";
  hash = "";
  cargoHash = "";

  inherit (stdenv.hostPlatform)
    isLinux
    isWindows
    isAarch64
    ;

  emulator = stdenv.hostPlatform.emulator buildPackages;
  exe = stdenv.hostPlatform.extensions.executable;

  # dbus-secret-service feature is part of default cargo features
  hasSecretService = hasDbusSecretService || builtins.elem "zbus-secret-service" buildFeatures;
  hasDbusSecretService = !buildNoDefaultFeatures || builtins.elem "dbus-secret-service" buildFeatures;

  dbus' = dbus.overrideAttrs (old: {
    env = (old.env or { }) // {
      NIX_CFLAGS_COMPILE =
        (old.env.NIX_CFLAGS_COMPILE or "")
        # required for D-Bus on Linux AArch64
        + lib.optionalString (isLinux && isAarch64) " -mno-outline-atomics";
    };
  });

in
rustPlatform.buildRustPackage {
  inherit cargoHash version buildNoDefaultFeatures;

  pname = "mimosa";

  src = fetchFromGitHub {
    inherit hash;
    owner = "pimalaya";
    repo = "mimosa";
    rev = "v${version}";
  };

  env = {
    # OpenSSL should not be provided by vendors, not even on Windows
    OPENSSL_NO_VENDOR = "1";
  };

  nativeBuildInputs =
    [ ]
    ++ lib.optional hasDbusSecretService pkg-config
    ++ lib.optional (installManPages || installShellCompletions) installShellFiles;

  buildInputs =
    [ ]
    ++ lib.optional hasSecretService openssl
    # D-Bus is provided by vendors on Windows
    ++ lib.optional (hasDbusSecretService && !isWindows) dbus';

  buildFeatures =
    buildFeatures
    # D-Bus is provided by vendors on Windows
    ++ lib.optional (hasDbusSecretService && isWindows) "vendored";

  doCheck = false;

  postInstall =
    lib.optionalString (lib.hasInfix "wine" emulator) ''
      export WINEPREFIX="''${WINEPREFIX:-$(mktemp -d)}"
      mkdir -p $WINEPREFIX
    ''
    + ''
      mkdir -p $out/share/{completions,man}
      ${emulator} "$out"/bin/mimosa${exe} manuals "$out"/share/man
      ${emulator} "$out"/bin/mimosa${exe} completions -d "$out"/share/completions bash elvish fish powershell zsh
    ''
    + lib.optionalString installManPages ''
      installManPage "$out"/share/man/*
    ''
    + lib.optionalString installShellCompletions ''
      installShellCompletion --cmd mimosa \
        --bash "$out"/share/completions/mimosa.bash \
        --fish "$out"/share/completions/mimosa.fish \
        --zsh "$out"/share/completions/_mimosa
    '';

  meta = {
    description = "CLI to manage passwords";
    mainProgram = "mimosa";
    homepage = "https://github.com/pimalaya/mimosa";
    changelog = "https://github.com/pimalaya/mimosa/blob/v${version}/CHANGELOG.md";
    license = lib.licenses.agpl3Plus;
    maintainers = with lib.maintainers; [ soywod ];
  };
}
