{
  description = "Shadow Design — ambiente di sviluppo (Rust + Dioxus 0.7 + SCSS + graphify)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system:
        f (import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        }));

      # graphify non è in nixpkgs: viene installato da PyPI in un venv locale al progetto.
      graphifyVersion = "0.9.95";
    in
    {
      devShells = forAllSystems (pkgs:
        let
          rust = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
            targets = [ "wasm32-unknown-unknown" ];
          };
          python = pkgs.python313;
        in
        {
          default = pkgs.mkShell {
            packages = [
              rust
              # Deve coincidere con la versione di `wasm-bindgen` in Cargo.lock. Va prima di
              # dioxus-cli nel PATH: il wrapper di dx aggiunge in coda la sua versione solo come fallback.
              pkgs.wasm-bindgen-cli_0_2_129
              pkgs.dioxus-cli # `dx serve` / `dx build`
              pkgs.dart-sass # compilazione SCSS fuori da dx (scripts/check_classes.py)
              pkgs.binaryen # wasm-opt per le build release
              python
              pkgs.uv
              pkgs.nushell # deploy.nu
            ];

            UV_PYTHON = "${python}/bin/python3";
            UV_PYTHON_DOWNLOADS = "never";

            # L'output va su stderr: la shell è usata anche come wrapper del server MCP (stdio).
            shellHook = ''
              root="$(git rev-parse --show-toplevel 2>/dev/null)"
              if [ -f "$root/flake.nix" ] && [ -f "$root/crate/scss/main.scss" ]; then
                tools="$root/.tools/graphify"
                stamp="$tools/.version"
                if [ "$(cat "$stamp" 2>/dev/null)" != "${graphifyVersion}+${python.version}" ]; then
                  echo "[shadow-design] installo graphifyy ${graphifyVersion} in .tools/graphify" >&2
                  rm -rf "$tools"
                  uv venv --quiet "$tools" >&2 \
                    && uv pip install --quiet --python "$tools/bin/python" "graphifyy[mcp]==${graphifyVersion}" >&2 \
                    && echo "${graphifyVersion}+${python.version}" > "$stamp"
                fi
                export PATH="$tools/bin:$PATH"
                # ADR-0001: pre-commit esegue scripts/lint_design.py
                if [ "$(git -C "$root" config core.hooksPath)" != ".githooks" ]; then
                  git -C "$root" config core.hooksPath .githooks
                fi
              else
                echo "[shadow-design] fuori dal repo: graphify non disponibile (lancia nix develop dalla root)" >&2
              fi
            '';
          };
        });
    };
}
