{ lib, linux_5_12, linuxConfig, linuxManualConfig, runCommand, stdenv }:

let
    # Select a Linux kernel version to build.
    # We will build our own Linux kernel,
    # and only use this for src and version.
    inherit (linux_5_12) src version;

    # Generate the Linux configuration file by executing `make defconfig`.
    # Then append our own custom configuration file onto it.
    configfile =
        let generated = linuxConfig { inherit src version; }; in
        runCommand "" {} ''cat ${generated} ${./extra.config} > $out'';

    # Take the configuration file we generated
    # and use it to compile the Linux kernel.
    linux = linuxManualConfig {
        inherit configfile lib src stdenv version;
    };
in
    linux
