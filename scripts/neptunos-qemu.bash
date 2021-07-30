#!/usr/bin/env bash

set -efuo pipefail

# Build all the components.
linux=$(nix-build --no-out-link -A neptunos.linux)
daemons=$(nix-build --no-out-link -A neptunos.daemons)
rootfs=$(nix-build --no-out-link -A neptunos.rootfs)

# Configure the kernel command line.
kernel_flags="
    root=/dev/sda rw
    init=$daemons/bin/neptunos-init-daemon
"

# Create a copy-on-write overlay on the rootfs from the Nix store.
qemu-img create -f qcow2 -F raw -b "$rootfs"/rootfs scripts/rootfs.qcow2

# Start QEMU, booting our kernel with our root file system.
qemu-system-x86_64           \
    -enable-kvm              \
    -kernel "$linux"/bzImage \
    -append "$kernel_flags"  \
    -drive file=scripts/rootfs.qcow2,format=qcow2
