{ bash, closureInfo, coreutils, e2fsprogs, eudev, neptunos, runCommand }:

let
    closure = closureInfo {
        rootPaths = [
            bash
            coreutils
            eudev
            neptunos.daemons
        ];
    };

################################################################################
#               TODO: Kernel modules, makeModulesClosure                       #
################################################################################

    env = {
        nativeBuildInputs = [
            e2fsprogs
        ];
    };
in
    runCommand "neptunos-rootfs" env ''

        # Create a root directory.
        mkdir --parents rootdir/{dev,/nix/store,proc,run,sys}
        cp --archive $(< ${closure}/store-paths) rootdir/nix/store
        ln --symbolic ${bash} rootdir/bash
        ln --symbolic ${coreutils} rootdir/coreutils
        ln --symbolic ${eudev} rootdir/eudev

        mkdir --parents rootdir/etc/udev/rules.d
        cat <<'EOF' > rootdir/etc/udev/rules.d/00.rules
        ENV{MODALIAS}=="?*", RUN{builtin}+="kmod load '$env{MODALIAS}'"
        EOF

        # Create a root file system.
        mkdir --parents $out
        truncate --size=200M $out/rootfs
        mkfs.ext4 $out/rootfs -d rootdir

    ''
