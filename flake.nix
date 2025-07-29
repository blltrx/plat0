{
  description = "";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        {
          devShells.default = import ./default.nix { inherit nixpkgs system; };
          packages.default = import ./default.nix {inherit nixpkgs system; };
        }
      );
}
