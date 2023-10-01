{
  description = "A simple flake with a devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, gitignore, flake-utils, fenix, naersk, ... }:
    let
      inherit (gitignore.lib) gitignoreSource;

      muslConfig = pkgs:
        let
          ccExport = ''
            export CC=x86_64-unknown-linux-musl-gcc
          '';
        in
        (with pkgs; {
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          shellHook = ccExport;
          preBuild = ccExport;
          depsBuildBuild = [ stdenv.cc ];
        });

      mkToolchain = import ./rust-toolchain.nix fenix;

      mkDevShell = buildPlatform: targetPlatform: pkgs: shell:
        let
          rust-toolchain = mkToolchain.fromTarget {
            inherit pkgs buildPlatform targetPlatform;
          };
        in
        pkgs.mkShell
          {
            CARGO_BUILD_TARGET = targetPlatform;

            buildInputs = with pkgs; [
              rust-toolchain
            ];
          } // shell;


      mkDevShells = buildPlatform:
        let
          pkgs = import nixpkgs { system = buildPlatform; };
          defaultShell = mkDevShell buildPlatform null pkgs { };
        in
        {
          default = defaultShell;
          musl = mkDevShell buildPlatform "x86_64-unknown-linux-musl" pkgs.pkgsStatic (muslConfig pkgs.pkgsStatic);
        };

      mkPackage = pkgName: pkgs: buildPlatform: targetPlatform: package:
        let
          toolchain = mkToolchain.fromTarget {
            inherit pkgs buildPlatform targetPlatform;
          };

          naersk' = naersk.lib.${buildPlatform}.override
            {
              cargo = toolchain;
              rustc = toolchain;
            };

          package' = {
            name = pkgName;
            src = gitignoreSource ./.;

            doCheck = true;
            buildInputs = with pkgs; [ toolchain ];
          } // pkgs.lib.optionalAttrs (!isNull targetPlatform) {
            CARGO_BUILD_TARGET = targetPlatform;
          } // package;
        in
        naersk'.buildPackage package';

      mkPackages = buildPlatform: pkgName:
        let
          pkgs = import nixpkgs { system = buildPlatform; };
          mkPackageWithTarget = mkPackage pkgName pkgs buildPlatform;
          defaultPackage = mkPackage pkgName pkgs buildPlatform null { };
        in
        {
          default = defaultPackage;
          musl = mkPackage pkgName pkgs.pkgsStatic buildPlatform "x86_64-unknown-linux-musl" (muslConfig pkgs.pkgsStatic);
        };
    in

    flake-utils.lib.eachDefaultSystem (system: {
      devShells = mkDevShells system;
      packages = mkPackages system "axum-hello-world";
    });
}
