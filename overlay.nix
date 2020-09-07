final: prev:
rec {
  vales = {
	pkgs = prev.callPackage ./pkgs { };
    vale-swift = prev.naersk.buildPackage {
      src = final.builtins.filterSource (path: type: type != "directory" || final.builtins.baseNameOf path != "target") ./.;
      remapPathPrefix = true;
    };
  };
}
