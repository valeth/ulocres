{
    description = "Development shell";

    inputs = {
        nixpkgs.url      = "github:nixos/nixpkgs/nixpkgs-unstable";
        flake-utils.url  = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                lib = nixpkgs.lib;
                overlays = [ (import rust-overlay) ];
                pkgs = import nixpkgs { inherit system overlays; };

                rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
                rustfmt = pkgs.rust-bin.nightly.latest.rustfmt;
            in {
                devShells.default = pkgs.mkShell {
                    name = "ulocres";

                    nativeBuildInputs = with pkgs; [
                        rustToolchain
                        pkg-config
                    ];

                    packages = with pkgs; [
                        rustfmt
                        cargo-deny
                        cargo-msrv
                    ];
                };
            });
}
