{ nixpkgs ? import nix/nixpkgs }:

nixpkgs.mkShell {
    nativeBuildInputs = [
        nixpkgs.cargo
        nixpkgs.qemu
    ];
}
