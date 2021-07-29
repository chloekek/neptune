# Nixpkgs pinned to a specific version.

let
    pinned   = fromTOML (builtins.readFile ./pinned.toml);
    tarball  = fetchTarball pinned;
    config   = { };
    overlays = [ (import ../naersk) (import ../neptunos) ];
in
    import tarball { inherit config overlays; }
