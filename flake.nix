{
  description = "Rocket Template";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
  };
  outputs = { self, nixpkgs, fenix }:
    let
      pname = "rocket-template";
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        inherit pname;
        version = "0.1.0";
        src = [./.];
        cargoSha256 = nixpkgs.lib.fakeSha256;
        postInstall = "";
      };
      devShells.${system}.default = pkgs.mkShell {
        name = "${pname}";
        packages = with pkgs; [
          sea-orm-cli
          cargo-watch
          rust-analyzer
          rustc
          cargo
          gcc
          rustfmt
          clippy
        ];
        nativeBuildInputs = with pkgs; [ pkg-config openssl ];
        RUST_SRC_PATH = "${fenix.packages.${system}.complete.rust-src}/lib/rustlib/src/rust/library";
        DATABASE_URL = "postgres://master:password@localhost/tdavis_dev";
      };
    };
}
