# This expression evaluates to a specific version of Nixpkgs,
# configured with specific configuration and overlays.

let
    pinned = fromTOML (builtins.readFile ./pinned.toml);
    tarball = fetchTarball pinned;
    config = { };
    overlays = [ ];
in
    import tarball {
        inherit config overlays;
    }
