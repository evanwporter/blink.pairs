{
  description = "SystemVerilog block-keyword matching for Neovim";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";

    blink-lib.url = "github:saghen/blink.lib";
    blink-lib.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    nixpkgs,
    blink-lib,
    self,
    ...
  }: let
    inherit (nixpkgs) lib;
    inherit (lib.attrsets) genAttrs mapAttrs' nameValuePair;
    inherit (lib.fileset) fileFilter toSource unions;
    inherit (lib.strings) hasPrefix;

    systems = ["x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"];
    forAllSystems = genAttrs systems;
    nixpkgsFor = forAllSystems (system:
      import nixpkgs {
        inherit system;
        overlays = [blink-lib.overlays.default];
      });

    version = "0.1.0";
    sv-matchit-package = {
      rustPlatform,
      vimPlugins,
      vimUtils,
    }:
      vimUtils.buildVimPlugin {
        pname = "sv-matchit";
        inherit version;
        src = toSource {
          root = ./.;
          fileset = unions [
            (fileFilter (file: file.hasExt "lua") ./lua)
            ./queries
          ];
        };

        dependencies = [
          (vimPlugins.blink-lib or (throw "vimPlugins.blink-lib not found; did you include its overlay?"))
        ];

        preInstall = ''
          mkdir -p lib
          ln -s $parser_lib/lib/libsv_matchit.* lib/
        '';

        nvimSkipModules = [
          "sv-matchit.rust"
        ];

        env.parser_lib = rustPlatform.buildRustPackage {
          pname = "sv-matchit-lib";
          inherit version;
          src = toSource {
            root = ./.;
            fileset = unions [
              (fileFilter (file: file.hasExt "rs") ./.)
              (fileFilter (file: hasPrefix "Cargo" file.name) ./.) # Cargo.*
              ./.cargo
            ];
          };
          cargoLock.lockFile = ./Cargo.lock;
          doCheck = false;
        };

        passthru = {inherit rustPlatform;};
      };
  in {
    packages = forAllSystems (system: rec {
      sv-matchit = nixpkgsFor.${system}.callPackage sv-matchit-package {};
      default = sv-matchit;
    });

    overlays.default = final: prev: {
      vimPlugins = prev.vimPlugins.extend (_: _: {
        sv-matchit = final.callPackage sv-matchit-package {};
      });
    };

    devShells = forAllSystems (
      system: let
        pkgs = nixpkgsFor.${system};
        packages = self.packages.${system};
      in {
        default = pkgs.mkShell {
          name = "sv-matchit";
          inputsFrom = [
            packages.sv-matchit.parser_lib
          ];
          packages = [pkgs.rust-analyzer];
        };
      }
    );

    checks = forAllSystems (system: mapAttrs' (n: nameValuePair "package-${n}") (removeAttrs self.packages.${system} ["default"]));
  };
}
