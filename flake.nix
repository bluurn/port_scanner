{
  description = "Port scanner CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
    }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "port_scanner";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = {
              description = "Simple port scanner CLI";
              mainProgram = "port_scanner";
            };
          };
        }
      );

      devShells = eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              rustc
              rustfmt
              rust-analyzer

              just
              nixfmt-rfc-style
            ];
          };
        }
      );

      formatter = eachSystem (
        system:
        nixpkgs.legacyPackages.${system}.nixfmt-rfc-style
      );

      checks = eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          package = self.packages.${system}.default;

          cargo-test = pkgs.rustPlatform.buildRustPackage {
            pname = "port_scanner-tests";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            checkPhase = ''
              cargo test
            '';

            installPhase = ''
              mkdir -p $out
            '';
          };
        }
      );
    };
}
