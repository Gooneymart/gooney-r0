use r0_kernel::KernelAnalyzer;
use std::path::PathBuf;
use anyhow::Result;

pub fn execute_inspect(kernel_path: PathBuf) -> Result<()> {
    println!("[*] Analyzing kernel source tree at: {:?}", kernel_path);
    let analyzer = KernelAnalyzer::new(kernel_path);
    let report = analyzer.analyze()?;

    println!("\n=== r0-kernel Compatibility Report ===");
    println!("Kernel Version : {}", report.version_string);
    println!("Architecture   : {}", report.target_arch);
    println!("DKMS Ready     : {}", if report.is_dkms_ready { "[YES]" } else { "[NO - Missing required flags]" });
    
    println!("\nConfiguration Audits:");
    for check in report.config_checks {
        let req_tag = if check.required { "(Required)" } else { "(Optional)" };
        println!("  - {:<24} : {:<15} {}", check.flag, check.status, req_tag);
    }
    println!("======================================\n");

    Ok(())
}
