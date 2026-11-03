use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Name of the soft-IP under test (e.g., "my_custom_core")
    pub ip_name: String,

    /// Base physical memory-mapped I/O (MMIO) address for the IP
    pub mmio_base_address: u64,

    /// Size of the MMIO region in bytes
    pub mmio_size: u64,

    /// Paths to SystemVerilog, Verilator, or Veryl HDL sources
    pub rtl_sources: Vec<PathBuf>,

    /// Path to target Linux kernel build headers
    pub kernel_headers_path: PathBuf,

    /// Path to the bare-metal RISC-V firmware binary (.bin) for co-simulation
    pub firmware_path: PathBuf,

    /// Path to the DKMS driver template source directory
    pub dkms_source_path: PathBuf,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            ip_name: "riscv_soft_ip".to_string(),
            mmio_base_address: 0x4000_0000,
            mmio_size: 0x0001_0000, // 64KB default MMIO window
            rtl_sources: vec![PathBuf::from("rtl/"), PathBuf::from("veryl/")],
            kernel_headers_path: PathBuf::from("/lib/modules/build"),
            firmware_path: PathBuf::from("firmware/test.bin"),
            dkms_source_path: PathBuf::from("driver/"),
        }
    }
}
