{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust Toolchain & Development
    rustc
    cargo
    rustfmt
    clippy

    # RISC-V Cross-Compilation Toolchain
    riscv64-linux-gnu-gcc
    gnumake
    git
    pkg-config

    # Simulation & Verification Tooling
    verilator
  ];

  shellHook = ''
    # Set environment variables for cross-compilation consistency
    export CC_riscv64_linux_gnu="riscv64-linux-gnu-gcc"

    echo "=================================================="
    echo " 🚀 gooney-ringzero (gooney-r0) Development Shell"
    echo "=================================================="
    echo " Rust:      $(rustc --version)"
    echo " RISC-V CC: $(riscv64-linux-gnu-gcc --version | head -n 1)"
    echo " Verilator: $(verilator --version)"
    echo "=================================================="
  '';
}
