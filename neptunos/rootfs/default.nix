{ closureInfo, e2fsprogs, neptunos, runCommand }:

let
    closure = closureInfo {
        rootPaths = [
            neptunos.daemons
        ];
    };

    env = {
        nativeBuildInputs = [
            e2fsprogs
        ];
    };
in
    runCommand "neptunos-rootfs" env ''

        # Create a root directory.
        mkdir --parents rootdir/nix/store
        cp --archive $(< ${closure}/store-paths) rootdir/nix/store

        # Create a root file system.
        mkdir --parents $out
        truncate --size=50M $out/rootfs
        mkfs.ext4 $out/rootfs -d rootdir

    ''
