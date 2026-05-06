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
          clippy
          vscode-extensions.vadimcn.vscode-lldb
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.vulkan-loader
        ];
        shellHook = ''
          mkdir -p .zed
          cat > .zed/settings.json <<EOF
          {
            "dap": {
              "CodeLLDB": {
                "binary": "${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/adapter/codelldb"
              }
            },
            "lsp": {
              "rust-analyzer": {
                "initialization_options": {
                  "check": { "command": "clippy" }
                }
              }
            }
          }
          EOF
        '';
      };
    };
}
