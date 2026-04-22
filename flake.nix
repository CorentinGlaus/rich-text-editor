{
  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          rustfmt
          rust-analyzer
          cargo
          gcc
          pkg-config
          wayland
          libxkbcommon
          clippy
          vulkan-loader
          vulkan-tools
          wgsl-analyzer
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.vulkan-loader
        ];
      };
    };
}
