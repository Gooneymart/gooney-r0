# Gooney RingZero (`gooney-r0`)

<div align="center">

### RISC-V Hardware/Software Co-Simulation & Design Verification Framework

[![Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/Architecture-RISC--V-blue)](https://riscv.org/)
[![Simulation](https://img.shields.io/badge/Simulation-Verilator-green)](https://verilator.org/)
[![Kernel](https://img.shields.io/badge/Kernel-Linux-red?logo=linux)](https://www.kernel.org/)
[![Build](https://img.shields.io/badge/Build-Cargo-brown?logo=rust)](https://doc.rust-lang.org/cargo/)
[![License: MPL-2.0](https://img.shields.io/badge/License-MPL--2.0-blue.svg)](https://opensource.org/license/mpl-2-0)

**Bridging RISC-V RTL Simulation, Software Verification, and Linux Kernel Driver Development.**

</div>

---

## Overview

**Gooney RingZero (`gooney-r0`)** is a modular, high-performance Rust-based Design Verification (DV) and hardware/software co-simulation framework engineered for RISC-V soft-IP development.

It establishes a reproducible verification environment that bridges RTL simulation, software-driven hardware testing, and Linux kernel-space driver validation.

Built around a modular Rust workspace, Gooney RingZero integrates **Verilator-based cycle-accurate simulation**, automated kernel configuration auditing, and DKMS-compatible driver source generation into a unified verification pipeline.

The framework is designed to simplify the development and validation of custom RISC-V hardware peripherals and soft-IP components by connecting hardware behavior with software-level execution and kernel-space interaction.

### Core Objectives

- **Hardware/Software Co-Design** — Establish a unified verification environment for RISC-V hardware and software components.
- **RTL Co-Simulation** — Integrate Verilator through Rust Foreign Function Interface (FFI) bindings for cycle-accurate hardware simulation.
- **Automated Driver Generation** — Generate C-based Linux kernel driver sources and DKMS package definitions from soft-IP register maps.
- **Kernel Compliance Auditing** — Inspect Linux kernel source trees and validate mandatory configuration requirements.
- **Reproducible Verification** — Provide a structured workflow for testing hardware functionality and software integration.

---

## System Architecture

Gooney RingZero is organized as a Cargo workspace containing five specialized Rust crates.

Each crate is responsible for a distinct layer of the verification pipeline.

```text
                     GOONEY RINGZERO
                         gooney-r0
                             |
              +--------------+--------------+
              |                             |
          r0-cli                       r0-core
     CLI Orchestrator              Core Primitives
              |                             |
    +---------+---------+                   |
    |         |         |                   |
 r0-sim  r0-driver-gen r0-kernel            |
    |         |         |                   |
    v         v         v                   |
 Verilator  DKMS/C    Linux Kernel          |
    |       Driver    Compliance            |
    |      Generation  Auditing             |
    +---------+---------+-------------------+
              |
              v
      Hardware/Software
       Verification Flow
```

### Core Components

| Crate | Responsibility |
|---|---|
| **`r0-core`** | Provides core bus transaction primitives, telemetry structures, shared data types, and common verification interfaces. |
| **`r0-sim`** | Implements Verilator Foreign Function Interface (FFI) bindings for cycle-accurate RTL co-simulation. |
| **`r0-driver-gen`** | Generates C-based Linux kernel driver templates and DKMS package source code targeting soft-IP register maps. |
| **`r0-kernel`** | Inspects Linux kernel source trees and audits `.config` settings against predefined compatibility requirements. |
| **`r0-cli`** | Provides the top-level command-line interface for orchestrating the verification pipeline. |

---

## Quick Start

### Prerequisites

Ensure the following development tools are available:

| Dependency | Purpose |
|---|---|
| Rust Toolchain | Workspace development and compilation |
| Cargo | Rust package and workspace management |
| Verilator | RTL simulation and hardware co-simulation |
| RISC-V GNU Toolchain | Cross-compilation for RISC-V targets |
| Linux Kernel Sources | Kernel configuration auditing and driver development |
| DKMS | Dynamic Kernel Module Support for external driver management |

### 1. Clone the Repository

```bash
git clone https://github.com/<YOUR-USERNAME>/gooney-r0.git
cd gooney-r0
```

### 2. Prepare the RISC-V Linux Kernel

Gooney RingZero uses a local Linux kernel source tree for kernel configuration auditing and driver development.

Prepare the kernel source tree inside `kernel/linux-riscv/`:

```bash
cd kernel/linux-riscv
```

Generate the default RISC-V kernel configuration:

```bash
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- defconfig
```

Prepare the kernel build environment:

```bash
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- prepare
```

Return to the workspace root:

```bash
cd ../..
```

### 3. Run Kernel Compliance Auditing

Before compiling or inserting out-of-tree soft-IP drivers, execute the kernel inspection command:

```bash
cargo run --bin r0-cli -- kernel inspect --path kernel/linux-riscv
```

The `r0-kernel` crate inspects the target kernel configuration and validates mandatory DKMS compatibility requirements.

**Example: Successful Compliance Report**

```text
=== r0-kernel Compatibility Report ===
Kernel Version : 7.4.0-rc1
Architecture   : riscv
DKMS Ready     : [YES]

Configuration Audits:
  - CONFIG_MODULES            : enabled (y)      (Required)
  - CONFIG_MODULE_UNLOAD      : enabled (y)      (Required)
  - CONFIG_PROVE_LOCKING      : enabled (y)      (Required)
  - CONFIG_DEBUG_INFO         : missing          (Optional)
  - CONFIG_KASAN              : missing          (Optional)
======================================
```

A successful audit confirms that all mandatory kernel configuration requirements enforced by `r0-kernel` are satisfied.

For detailed auditing instructions, refer to the `r0-kernel` documentation.

### 4. Build the Workspace

Compile all Rust crates within the workspace:

```bash
cargo build --workspace
```

For an optimized release build:

```bash
cargo build --workspace --release
```

---

## Repository Structure

The project follows a modular workspace layout separating Rust crate implementations, local kernel environments, firmware payloads, and generated driver packages.

```text
gooney-r0/
│
├── crates/
│   ├── r0-core/              # Core primitives and shared types
│   ├── r0-sim/               # Verilator FFI and simulation bindings
│   ├── r0-driver-gen/        # DKMS and driver source generator
│   ├── r0-kernel/            # Linux kernel compliance auditor
│   └── r0-cli/               # CLI orchestration interface
│
├── kernel/
│   └── linux-riscv/          # Local RISC-V Linux kernel source
│
├── firmware/                 # Bare-metal RISC-V firmware and payloads
│
├── load/                     # Staged DKMS modules and insertion scripts
│
├── Cargo.toml                # Workspace manifest
├── Cargo.lock                # Dependency lockfile
├── LICENSE                   # MPL-2.0 license
└── README.md                 # Project documentation
```

---

## Verification Workflow

The Gooney RingZero verification pipeline is designed around the interaction between hardware simulation, software interfaces, and kernel-space driver integration.

### Verification Pipeline

1. **Hardware Simulation** — Use `r0-sim` to interface with Verilator and execute RTL simulations.
2. **Transaction Processing** — Use `r0-core` to provide shared bus transaction structures and telemetry.
3. **Driver Generation** — Use `r0-driver-gen` to generate Linux kernel driver sources and DKMS definitions for target soft-IP peripherals.
4. **Kernel Auditing** — Use `r0-kernel` to validate kernel configuration requirements before driver integration.
5. **Workflow Orchestration** — Use `r0-cli` to coordinate the available verification and development operations.

This architecture provides a foundation for repeatable hardware/software validation throughout the RISC-V soft-IP development lifecycle.

---

## Design Philosophy

Gooney RingZero follows several core engineering principles:

- **Modularity** — Independent Rust crates with clearly separated responsibilities.
- **Reproducibility** — Consistent build and verification environments.
- **Hardware/Software Integration** — Connecting RTL behavior with software-driven testing.
- **Automation** — Reducing repetitive driver generation and kernel configuration validation tasks.
- **Transparency** — Providing inspectable simulation, telemetry, and verification workflows.

---

## License

This project is licensed under the **Mozilla Public License Version 2.0 (MPL-2.0)**.

See the [LICENSE](LICENSE) file for the complete license text.

Official license: https://www.mozilla.org/MPL/2.0/

---

<div align="center">

**Gooney RingZero — Engineering the Boundary Between Hardware and Software.**

*Part of the Gooneymart RISC-V Development Ecosystem.*
for corporate licensing, please email to gooneymart@gmail.com
</div>
