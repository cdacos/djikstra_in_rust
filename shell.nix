with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";

  nativeBuildInputs = [
    rustup
  ];

  buildInputs = [
    # Example run-time additional dependencies
    # openssl
  ];


  # Rely on rustup for the Rust toolchain packaging
  # Note: for some reason if the override command stdout isn't sent to null, direnv fails with: 
  #       direnv: error unmarshal() base64 decoding: illegal base64 data at input byte 1
  shellHook = ''
    rustup override set 1.37.0 > /dev/null
    rustup component add --toolchain 1.37.0 rls rustfmt rust-src clippy
  '';

  # Set environment variables
  RUST_BACKTRACE = 1;
}