{ ... }:

{
  cachix.enable = false;

  # cargo + rustc are what CI runs (.github/workflows/rust.yml: build + test);
  # the module's default components add clippy, rustfmt and rust-analyzer,
  # which Doom's nil/LSP setup picks up for free.
  languages.rust.enable = true;

  enterShell = ''
    echo "paswitch-rs devshell"
    echo "  cargo: $(cargo --version)"
    echo "  rustc: $(rustc --version)"
  '';
}
