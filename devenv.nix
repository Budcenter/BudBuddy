{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/basics/
  # env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.sqlx-cli
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
  };

  dotenv.enable = true;

  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/services/
  services.postgres = {
    enable = true;
    listen_addresses = "127.0.0.1";
    # settings = {
    #   ssl = "on";
    #   ssl_cert_file = "./server.crt";
    #   ssl_key_file = "./server.key";
    # };
  };

  # https://devenv.sh/scripts/
  # scripts.hello.exec = ''
  #   echo hello from $GREET
  # '';

  # https://devenv.sh/basics/
  enterShell = ''
    git --version # Use packages
    nix --version
    devenv -V
    cargo -V
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    cargo test
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;
  git-hooks.hooks = {
    # clippy.enable = true;
    rustfmt = {
      enable = true;
      settings = {
        emit = "files";
      };
    };
    cargo-check.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
