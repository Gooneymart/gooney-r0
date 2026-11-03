// crates/r0-kernel/src/analyzer.rs

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigCheckResult {
    pub flag: String,
    pub enabled: bool,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KernelReport {
    pub kernel_path: PathBuf,
    pub version_string: String,
    pub target_arch: String,
    pub config_checks: Vec<FlagStatus>,
    pub is_dkms_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagStatus {
    pub flag: String,
    pub status: String, // "enabled (y/m)", "disabled", "missing"
    pub required: bool,
}

pub struct KernelAnalyzer {
    kernel_dir: PathBuf,
}

impl KernelAnalyzer {
    pub fn new(kernel_dir: PathBuf) -> Self {
        Self { kernel_dir }
    }

    pub fn analyze(&self) -> Result<KernelReport> {
        let version_string = self.detect_version()?;
        let target_arch = self.detect_arch()?;
        let config_checks = self.audit_config()?;

        // Determine if mandatory flags are met
        let is_dkms_ready = config_checks.iter()
            .filter(|f| f.required)
            .all(|f| f.status.starts_with("enabled"));

        Ok(KernelReport {
            kernel_path: self.kernel_dir.clone(),
            version_string,
            target_arch,
            config_checks,
            is_dkms_ready,
        })
    }

    fn detect_version(&self) -> Result<String> {
        let uts_path = self.kernel_dir.join("include/generated/utsrelease.h");
        if uts_path.exists() {
            let file = File::open(uts_path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if line.starts_with("#define UTS_RELEASE") {
                    let parts: Vec<&str> = line.split('"').collect();
                    if parts.len() >= 2 {
                        return Ok(parts[1].to_string());
                    }
                }
            }
        }
        Ok("Unknown (Run 'make kernelversion' in tree)".to_string())
    }

    fn detect_arch(&self) -> Result<String> {
        Ok("riscv".to_string())
    }

    fn audit_config(&self) -> Result<Vec<FlagStatus>> {
        let config_path = self.kernel_dir.join(".config");
        if !config_path.exists() {
            anyhow::bail!("Kernel .config file not found at {:?}. Run 'make defconfig' first.", config_path);
        }

        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let mut flags = std::collections::HashMap::new();

        for line in reader.lines() {
            let line = line?;
            if let Some((k, v)) = line.split_once('=') {
                flags.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        let mandatory_flags = vec![
            ("CONFIG_MODULES", true, "Required for loading DKMS modules"),
            ("CONFIG_MODULE_UNLOAD", true, "Required for rmmod and leak checks"),
            ("CONFIG_PROVE_LOCKING", true, "Enables Lockdep for concurrency validation"),
            ("CONFIG_DEBUG_INFO", false, "Exposes DWARF symbols for stack traces"),
            ("CONFIG_KASAN", false, "Kernel Address Sanitizer for out-of-bounds MMIO checks"),
        ];

        let mut results = Vec::new();
        for (flag, required, _) in mandatory_flags {
            let status = match flags.get(flag) {
                Some(val) if val == "y" || val == "m" => format!("enabled ({})", val),
                Some(val) => format!("disabled ({})", val),
                None => "missing".to_string(),
            };

            results.push(FlagStatus {
                flag: flag.to_string(),
                status,
                required,
            });
        }

        Ok(results)
    }
}
