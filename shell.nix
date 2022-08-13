{ pkgs ? import ./nix/nixpkgs.nix {}
}: pkgs.mkShell {
    name = "dev-shell";
}