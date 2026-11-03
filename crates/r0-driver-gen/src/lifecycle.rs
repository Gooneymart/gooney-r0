use std::path::PathBuf;
use std::process::Command;
use anyhow::{Result, anyhow};

pub struct KernelModuleManager {
    driver_path: PathBuf,
}

impl KernelModuleManager {
    pub fn new(driver_path: PathBuf) -> Self {
        Self { driver_path }
    }

    pub fn compile(&self) -> Result<()> {
        let status = Command::new("make")
            .current_dir(&self.driver_path)
            .status()?;

        if !status.success() {
            return Err(anyhow!("Failed to compile DKMS kernel module"));
        }
        Ok(())
    }

    pub fn insert_module(&self) -> Result<()> {
        let ko_path = self.driver_path.join("gooney_drv.ko");
        let status = Command::new("sudo")
            .arg("insmod")
            .arg(ko_path)
            .status()?;

        if !status.success() {
            return Err(anyhow!("Failed to insert kernel module (insmod)"));
        }
        Ok(())
    }

    pub fn remove_module(&self) -> Result<()> {
        let status = Command::new("sudo")
            .arg("rmmod")
            .arg("gooney_drv")
            .status()?;

        if !status.success() {
            return Err(anyhow!("Failed to remove kernel module (rmmod)"));
        }
        Ok(())
    }
}
