{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        args = {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config clang ];
          buildInputs = with pkgs; [ openssl ];
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        };
      in
      rec {
        defaultPackage = naersk-lib.buildPackage args;

        shellArgs = args // {
          buildInputs = args.buildInputs ++ (with pkgs; [
            cargo
            rustc
            rustfmt
            rustPackages.clippy
            rust-analyzer
          ]);
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
        devShell = pkgs.mkShell shellArgs;
      });
}
