{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
	pname = "wol";
	version = "0.1";
	src = fetchFromGithub {
		owner = erocks26
		repo = wol
		rev = version
		hash = ;
}
