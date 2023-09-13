# inspired by https://ayats.org/blog/nix-rustup/

{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    # derive the correct compiler versions from rust-toolchain.toml
    rust-overlay.url = "github:oxalica/rust-overlay";

    # for `flake-utils.lib.eachSystem`
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
          config.allowUnfree = false;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPkgs = [ rustToolchain ] ++ (with pkgs; [ openssl pkg-config sqlite cargo-chef ]);
        rustDevPkgs = rustPkgs ++ (with pkgs; [ cargo-watch rust-analyzer ]);
      in
      {
        devShells = {
          default = with pkgs; pkgs.mkShellNoCC {
            buildInputs = rustDevPkgs ++ [
              git
              just
              sqlx-cli
              sqlite-interactive
            ];
          };
          buildRust = with pkgs; pkgs.mkShellNoCC {
            buildInputs = rustPkgs;
          };
        };
      }
    );
}
