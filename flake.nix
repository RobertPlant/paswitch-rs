{
  description = "List and swap to pulse sinks by name";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          cargo = pkgs.lib.importTOML ./Cargo.toml;
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = cargo.package.name;
            inherit (cargo.package) version;
            src = pkgs.lib.cleanSource self;
            cargoLock.lockFile = ./Cargo.lock;

            # pactl is called at runtime; bake in the store path instead of trusting $PATH.
            postPatch = ''
              substituteInPlace src/commands.rs \
                --replace-fail '"pactl"' '"${pkgs.lib.getExe' pkgs.pulseaudio "pactl"}"'
            '';

            meta = {
              inherit (cargo.package) description;
              homepage = "https://github.com/RobertPlant/paswitch-rs";
              license = pkgs.lib.licenses.gpl3Only;
              mainProgram = "paswitch-rs";
              platforms = pkgs.lib.platforms.linux;
            };
          };
        }
      );
    };
}
