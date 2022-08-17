{ nixpkgs ? <nixpkgs> }:
let pkgs = import nixpkgs {}; in
pkgs.rustPlatform.buildRustPackage {
  pname = "repo";
  version = "0.0.0";
  src = ./.;
  cargoSha256 = "sha256-7LzyBYGZg7XY4X4lNPBDhDJTI4hoUx8fgv9yC4ARmHc=";
}