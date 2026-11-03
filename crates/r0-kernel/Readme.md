# r0-kernel — RISC-V Kernel Compatibility & Compliance Auditor

**`r0-kernel`** is an independent kernel auditing crate within the **Gooney R0** workspace. It provides automated inspection and compliance validation of local Linux kernel source trees, ensuring that kernel configurations satisfy mandatory requirements before compiling or inserting out-of-tree kernel modules (DKMS).

Designed to support reproducible driver development and hardware/software co-simulation, `r0-kernel` helps identify missing kernel capabilities that could otherwise lead to build failures, module loading issues, or difficult-to-diagnose runtime faults.

---

## Overview

The `r0-kernel` auditing subsystem analyzes a target Linux kernel source tree by inspecting its generated `.config` file and validating selected kernel configuration options against predefined compliance requirements.

Its primary objectives are:

- **Kernel Configuration Auditing** — Inspect Linux kernel configuration settings.
- **DKMS Compatibility Validation** — Verify that mandatory kernel features required for external module development are enabled.
- **Lockdep Verification** — Ensure kernel locking validation support is available for concurrency and synchronization testing.
- **Debugging Capability Assessment** — Identify optional debugging and memory-safety instrumentation features.
- **Pre-Build Compliance Checks** — Detect configuration issues before DKMS compilation and module insertion.

## Architecture

`r0-kernel` operates as part of the `gooney-r0` workspace and is invoked through the `r0-cli` command-line orchestrator.

```text
gooney-r0/
├── kernel/
│   └── linux-riscv/          # Target Linux kernel source tree
│       ├── .config           # Generated kernel configuration
│       └── include/          # Prepared kernel headers
│
├── crates/
│   └── r0-kernel/            # Kernel auditing and compliance engine
│
└── r0-cli/                   # CLI orchestration interface
```

The auditing workflow follows three stages:

1. **Source Inspection** — Locate and inspect the target kernel source tree.
2. **Configuration Analysis** — Parse `.config` and evaluate required kernel options.
3. **Compliance Reporting** — Generate a compatibility report indicating DKMS readiness.

---

## Developer Workflow

To maintain reproducible testing environments and avoid configuration-related failures during soft-IP driver co-simulation, DKMS developers should audit their target Linux kernel before compiling external kernel modules.

### 1. Prerequisites

Prepare a RISC-V Linux kernel source tree inside the workspace.

The kernel must have a generated `.config` and prepared headers.

```bash
cd kernel/linux-riscv
```

Generate the default kernel configuration:

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

### 2. Run Kernel Inspection

Execute the kernel inspection command through the `r0-cli` orchestrator:

```bash
cargo run --bin r0-cli -- kernel inspect --path kernel/linux-riscv
```

The CLI forwards the target source directory to `r0-kernel`, which performs the configuration audit and generates a compliance report.

---

## Compatibility Audits

`r0-kernel` evaluates kernel configuration options according to their importance to the DKMS development workflow.

### Mandatory Configuration Flags

The following configuration options are required for a kernel to pass the current DKMS readiness audit.

| Configuration | Required Value | Purpose |
|---|---|---|
| `CONFIG_MODULES` | `y` | Enables loadable kernel module support, allowing external drivers to be inserted into the kernel. |
| `CONFIG_MODULE_UNLOAD` | `y` | Enables module removal through mechanisms such as `rmmod`, supporting repeatable driver testing and cleanup. |
| `CONFIG_PROVE_LOCKING` | `y` | Enables lock dependency validation (Lockdep), helping detect potential deadlocks and locking violations. |

### Optional Debugging Configuration

These options are recommended for advanced driver development and debugging but are not mandatory for DKMS readiness.

| Configuration | Recommended Value | Purpose |
|---|---|---|
| `CONFIG_DEBUG_INFO` | `y` | Provides debugging information, including DWARF symbols, for stack traces and symbol resolution. |
| `CONFIG_KASAN` | `y` | Enables Kernel Address Sanitizer to detect memory-safety violations such as out-of-bounds accesses and use-after-free errors. |

> **Note:** Optional configuration flags do not affect the mandatory DKMS readiness result. Their availability depends on the target kernel architecture, version, and configuration dependencies.

---

## Understanding the Compatibility Report

After inspection, `r0-kernel` produces a structured report describing the target kernel's configuration and compatibility status.

### Example: Successful Audit

```text
[*] Analyzing kernel source tree at: "kernel/linux-riscv"

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

### Report Field Definitions

| Field | Description |
|---|---|
| **Kernel Version** | Version identifier extracted from the target kernel source tree. |
| **Architecture** | Target kernel architecture detected during inspection. |
| **DKMS Ready** | Overall compliance result based on mandatory configuration requirements. |
| **Configuration Audits** | Individual status of each inspected kernel configuration option. |

### Readiness Status

| Status | Meaning |
|---|---|
| `DKMS Ready: [YES]` | All mandatory kernel configuration requirements are satisfied. |
| `DKMS Ready: [NO - Missing required flags]` | One or more mandatory configuration options are missing or disabled. |

A successful audit indicates that the target kernel satisfies the configuration requirements enforced by `r0-kernel`. It does not independently guarantee that a particular DKMS package will compile or load successfully.

---

## Troubleshooting Missing Configuration Flags

If the inspection reports:

```text
DKMS Ready : [NO - Missing required flags]
```

the target kernel is missing one or more mandatory configuration options.

### 1. Navigate to the Kernel Source Tree

```bash
cd kernel/linux-riscv
```

### 2. Modify the Kernel Configuration

You can enable the required options through the kernel configuration interface:

```bash
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- menuconfig
```

Alternatively, append the required configuration option manually:

```bash
echo "CONFIG_PROVE_LOCKING=y" >> .config
```

Repeat for any other missing mandatory options.

### 3. Resolve Configuration Dependencies

Run `olddefconfig` to reconcile the modified configuration with the kernel's available Kconfig options and dependencies:

```bash
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- olddefconfig
```

**Important:** Manually appending a configuration option does not guarantee that it will remain enabled. Kconfig may disable it if its dependencies are not satisfied. Always inspect the resulting `.config` after running `olddefconfig`.

### 4. Re-Prepare the Kernel

After modifying the configuration, regenerate the prepared build environment:

```bash
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- prepare
```

### 5. Re-run the Compliance Audit

Return to the workspace root:

```bash
cd ../..
```

Execute the inspection command again:

```bash
cargo run --bin r0-cli -- kernel inspect --path kernel/linux-riscv
```

Verify that all mandatory configuration flags are enabled and that the report indicates:

```text
DKMS Ready : [YES]
```

---

## Integration with Gooney R0

`r0-kernel` serves as the kernel configuration validation layer within the Gooney R0 ecosystem.

By enforcing minimum kernel configuration requirements before external module development, it helps establish a more predictable environment for:

- RISC-V Linux kernel development.
- Out-of-tree kernel module compilation.
- DKMS-based driver integration.
- Soft-IP driver development and verification.
- Hardware/software co-simulation workflows.
- Kernel locking and memory-safety debugging.

The auditing subsystem is intended to complement the broader Gooney R0 development workflow by identifying kernel configuration problems before they become build-time or runtime issues.

---

## License

Refer to the repository's root `LICENSE` file for licensing information.
