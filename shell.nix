{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = [
        pkgs.darwin.apple_sdk.frameworks.Security
    ];

    nativeBuildInputs = [
        pkgs.cargo
        pkgs.libiconv
        pkgs.openssl
        pkgs.pkgconfig
        pkgs.pre-commit
        pkgs.rustc
    ];
}
