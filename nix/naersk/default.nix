# Overlay for building Rust crates using Nix.

let
    pinned  = fromTOML (builtins.readFile ./pinned.toml);
    tarball = fetchTarball pinned;
in
    import (tarball + "/overlay.nix")
