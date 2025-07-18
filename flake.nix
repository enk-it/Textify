{
  description = "Rust development environment with Leptonica for leptonica-sys";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            pkg-config
            clang
            llvmPackages.libclang
            cmake
            meson
            ninja
            gdb
            tesseract
          ];

          buildInputs = with pkgs; [
            leptonica
            glibc.dev
            zlib # Required by Leptonica
          ];

          # Required for bindgen to find libclang
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

          # Include paths for bindgen to find headers
          BINDGEN_EXTRA_CLANG_ARGS = ''
            -I${pkgs.glibc.dev}/include
            -I${pkgs.llvmPackages.libclang.lib}/include
            -I${pkgs.leptonica}/include/leptonica
            -I${pkgs.zlib.dev}/include
          '';

          # Ensure pkg-config can find Leptonica
          PKG_CONFIG_PATH = "${pkgs.leptonica}/lib/pkgconfig:${pkgs.zlib}/lib/pkgconfig";

          # Linker flags for Rust (optional, as pkg-config should handle this)
          RUSTFLAGS = "-L ${pkgs.leptonica}/lib -L ${pkgs.zlib}/lib";

          # Optional: For rust-analyzer or other tools needing Rust source
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          shellHook = ''
            echo "Rust development environment ready!"
            echo "LIBCLANG_PATH: $LIBCLANG_PATH"
            echo "PKG_CONFIG_PATH: $PKG_CONFIG_PATH"
            echo "Leptonica lib: ${pkgs.leptonica}/lib"
          '';
        };
      }
    );
}