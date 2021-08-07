{ nixpkgs ? import nix/nixpkgs }:

let
    # For our build script we need several Perl packages.
    # Create a wrapped Perl in which these are available.
    perlPackages = p: [ p.IPCSystemSimple ];
    perl = nixpkgs.perl.withPackages perlPackages;

    # We want to build FreeType with additional options.
    freetype = nixpkgs.freetype.overrideAttrs (old: {
        patches = old.patches ++ [
            # FreeType is configurable through a header file.
            # We modify the header file by applying a patch.
            tools/freetype-ftoption.patch
        ];
    });
in
    nixpkgs.mkShell {
        nativeBuildInputs = [
            nixpkgs.cargo        # Used for building Rust crates.
            nixpkgs.e2fsprogs    # Used for creating rootfs.
            nixpkgs.nix          # Used for building Linux.
            nixpkgs.qemu         # Used for testing in a virtual machine.
            nixpkgs.rust-bindgen # Used for generating Rust from C headers.
            nixpkgs.rustfmt      # Used by bindgen only.
            perl                 # The build script is written in Perl.
        ];

        buildInputs = [
            freetype # Used for accessing font data.
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
