# Overlay that exports all Neptunos packages.

self: super:

{
    neptunos.rootfs = self.callPackage ../../neptunos/rootfs { };
    neptunos.daemons = self.naersk.buildPackage ../../neptunos/daemons;
    neptunos.linux = self.callPackage ../../neptunos/linux { };
}
