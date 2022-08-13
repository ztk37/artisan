# re-export nixpkgs from niv sources
{ sources ? import ./sources.nix }: import sources.nixpkgs {}
