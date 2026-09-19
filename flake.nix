{
  description = "FlipClock — pomodoro, clock, timer and stopwatch TUI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "flipclock";
          version = "0.1.0";
          # Ship only what the build needs (keeps target/ etc. out of the store).
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./src
            ];
          };
          cargoLock.lockFile = ./Cargo.lock;
          # The invoke command is `clocktui`:
          postInstall = ''
            ln -s $out/bin/flipclock $out/bin/clocktui
          '';
          meta = {
            description = "FlipClock — pomodoro, clock, timer and stopwatch TUI";
            license = pkgs.lib.licenses.mit;
            mainProgram = "clocktui";
          };
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/clocktui";
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
        };
      });
}
