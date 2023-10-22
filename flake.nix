{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.73.0";
          packageFun = import ./Cargo.nix;
        };

        workspaceShell = rustPkgs.workspaceShell {
          packages = with pkgs; [ pkg-config openssl ];
        };
      in
      rec {
        devShells = {
          default = workspaceShell;
        };

        packages = {
          waybar-cy-live-weather = (rustPkgs.workspace.waybar-cy-live-weather { });
          default = packages.waybar-cy-live-weather;
        };
      }
    );
}
