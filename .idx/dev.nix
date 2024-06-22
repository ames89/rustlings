# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "stable-23.11"; # or "unstable"
  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.stdenv.cc
    pkgs.python3
  ];
  # Sets environment variables in the workspace
  env = {
    RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  };
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      "vadimcn.vscode-lldb"
      "serayuzgur.crates"
      "tamasfe.even-better-toml"
      "mhutchie.git-graph"
      "eamodio.gitlens"
      "rust-lang.rust-analyzer"
    ];
    # Enable previews and customize configuration
    previews = {};
  };
  idx.workspace.onCreate = {
    rustip-install = "RUSTUP_INIT_SKIP_PATH_CHECK='yes';curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh";
  };
}