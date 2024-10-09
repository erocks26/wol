{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
	pname = "wol";
	version = "0.1";
	cargoLock.lockFile = ./Cargo.lock;
	src = pkgs.lib.cleanSource ./.;
}
