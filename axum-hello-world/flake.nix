{
  description = "dist doc build environment";
  nixConfig.bash-prompt = "[nix(dist doc)] ";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        darwin_pkgs = if pkgs.system == "x86_64-darwin" || pkgs.system == "aarch64-darwin" then [ pkgs.darwin.apple_sdk.frameworks.Security ] else [ ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = darwin_pkgs ++ [
            gnumake
            openssl
            pkg-config
            binaryen
            cargo-deny
            terraform
            openssh
            gnutar
            gnused
            bash
            sqlx-cli
            (rust-bin.stable.latest.default.override
              {
                targets = [
                  "wasm32-unknown-unknown"
                  "x86_64-unknown-linux-musl"
                ];
              })
          ];
        };
      }
    );
}
