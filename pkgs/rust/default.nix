{ rustChannelOf }:
(rustChannelOf {
  date = "2019-09-13";
  channel = "nightly";
  sha256 = "06881g7ba2hzmfq5vaz888d2q762zf4bxjc621rw3g8z702ps7w9";
}).rust.override {
  extensions = [
    "clippy-preview"
    "rls-preview"
    "rustfmt-preview"
    "rust-analysis"
    "rust-std"
    "rust-src"
  ];
  targets = [ "wasm32-unknown-unknown" ];
}
