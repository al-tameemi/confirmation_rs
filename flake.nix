{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      buildInputs = with pkgs; [ glib glib.dev pkg-config gtk4 gtk4.dev ];
      nativeBuildInputs = with pkgs; [ rust-bin.stable.latest.complete ];
    in
    with pkgs; rec
    {
      devShells.default = mkShell rec {
        inherit buildInputs nativeBuildInputs;
        LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        PKG_CONFIG_PATH = "${lib.makeLibraryPath buildInputs}";
      };
    }
  );
}
