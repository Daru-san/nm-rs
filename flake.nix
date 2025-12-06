{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        lib = pkgs.lib;
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            pkg-config
            gir-rs
            networkmanager
            glib
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath (
            with pkgs;
            [
              networkmanager
              glib
            ]
          );

          GIR_DIRS = lib.makeSearchPathOutput "dev" "share/gir-1.0" [ pkgs.networkmanager ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
