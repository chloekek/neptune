{ nixpkgs ? import nix/nixpkgs }:

let
    # For our build script we need several Perl packages.
    # Create a wrapped Perl in which these are available.
    perlPackages = p: [ p.IPCSystemSimple ];
    perl = nixpkgs.perl.withPackages perlPackages;
in
    nixpkgs.mkShell {
        nativeBuildInputs = [
            nixpkgs.cargo     # Used for building Rust crates.
            nixpkgs.e2fsprogs # Used for creating rootfs.
            nixpkgs.nix       # Used for building Linux.
            nixpkgs.qemu      # Used for testing in a virtual machine.
            perl              # The build script is written in Perl.
        ];

        # These executables need to be available to the build script,
        # as it will symlink them into the /bin directory in the rootfs.
        # TODO: Figure out whether this works with cross compilation.
        NP_NIX_EXECUTABLES = [
            "${nixpkgs.bash}/bin/bash"
            "${nixpkgs.coreutils}/bin/ls"
        ];

        # These fonts need to be available to the build script,
        # as it will symlink them into the /fonts directory in the rootfs.
        NP_NIX_FONTS = [
            "${nixpkgs.freefont_ttf}/share/fonts/truetype/FreeSans.ttf"
            "${nixpkgs.freefont_ttf}/share/fonts/truetype/FreeSerif.ttf"
        ];
    }
