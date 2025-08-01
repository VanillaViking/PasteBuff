{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
buildInputs = [ cargo ]; # your dependencies here
}
