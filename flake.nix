{
  description = "vale-langjam";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.devshell.url = "github:numtide/devshell";
  inputs.mozilla-overlay = {
    type = "github";
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    flake = false;
  };

  outputs = { self, nixpkgs, mozilla-overlay, flake-utils, devshell }:
    {
      overlay = import ./overlay.nix;
    }
    //
    (
      flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
			  (import mozilla-overlay)
			  devshell.overlay
              self.overlay
            ];
          };
        in
        {
		  legacyPackages = pkgs.vale;
          packages = flake-utils.lib.flattenTree pkgs.vale;
		  devShell = pkgs.mkDevShell.fromTOML ./devshell.toml;

          # Additional checks on top of the packages
          checks = { };

        }
      )
    );
}
