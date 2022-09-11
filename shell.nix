{ pkgs ? import <nixpkgs> {}
}:

let
    inherit (pkgs) callPackage mkShell;

    artisan = callPackage ./default.nix {};
in mkShell {
    name = "nix-shell";

    buildInputs = [
        artisan
    ];
}