{}: {
  rustVersion = "1.79.0";

  termusix = {
    lib,
    craneLib,
    pathCwd,
  }: let
    nonCargoBuildFiles = path: _type: builtins.match ".*(sql|md)$" path != null;
    includeFilesFilter = path: type:
      (craneLib.filterCargoSources path type) || (nonCargoBuildFiles path type);
  in {
    crateInfo = craneLib.crateNameFromCargoToml {cargoToml = pathCwd + "/Cargo.toml";};

    src = lib.cleanSourceWith {
      src = pathCwd;
      filter = includeFilesFilter;
    };
  };
}
