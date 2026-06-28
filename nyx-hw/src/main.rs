//! # Nyx Hardware Manager
//!
//! Auto-detects hardware and manages drivers/firmware.
//! Runs as a systemd service, listens for udev events.

use sysinfo::System;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("nyx_hw=info")
        .init();

    info!("Nyx Hardware Manager starting...");

    let hw = HardwareInfo::detect();
    info!("CPU: {} ({} cores)", hw.cpu_name, hw.cpu_cores);
    info!("RAM: {} MB total", hw.ram_total_mb);
    info!("GPU: {}", hw.gpu_name);
    info!("GPU driver recommendation: {:?}", hw.gpu_driver);

    Ok(())
}

/// Detected hardware information
struct HardwareInfo {
    cpu_name: String,
    cpu_cores: usize,
    ram_total_mb: u64,
    gpu_name: String,
    gpu_vendor: GpuVendor,
    gpu_driver: DriverRecommendation,
}

#[derive(Debug)]
enum GpuVendor {
    Amd,
    Intel,
    Nvidia,
    Unknown,
}

#[derive(Debug)]
enum DriverRecommendation {
    /// Mesa — no action needed, works out of the box
    Mesa,
    /// NVIDIA proprietary — offer to install
    NvidiaProprietary,
    /// Unknown — try mesa, fall back to software
    Fallback,
}

impl HardwareInfo {
    fn detect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_name = sys.cpus().first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".into());
        let cpu_cores = sys.cpus().len();
        let ram_total_mb = sys.total_memory() / 1024 / 1024;

        // GPU detection would use lspci or /sys/class/drm on Linux
        // For now, placeholder
        let gpu_name = "Auto-detected on Linux".into();
        let gpu_vendor = GpuVendor::Unknown;
        let gpu_driver = match gpu_vendor {
            GpuVendor::Amd | GpuVendor::Intel => DriverRecommendation::Mesa,
            GpuVendor::Nvidia => DriverRecommendation::NvidiaProprietary,
            GpuVendor::Unknown => DriverRecommendation::Fallback,
        };

        Self {
            cpu_name,
            cpu_cores,
            ram_total_mb,
            gpu_name,
            gpu_vendor,
            gpu_driver,
        }
    }
}
