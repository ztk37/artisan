{ nixpkgs ? <nixpkgs> }:
let pkgs = import nixpkgs {}; in
pkgs.rustPlatform.buildRustPackage {
  pname = "artisan";
  version = "0.0.0";
  src = ./.;
  cargoSha256 = "sha256-LEN6OplCCCj/IYAVFqEmmlQsl9ngo6T6T4e4Hj2Cw+8=";
}