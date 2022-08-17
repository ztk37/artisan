{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
    name = "nix-shell";

    buildInputs = [
        pkgs.rustfmt
    ];
}