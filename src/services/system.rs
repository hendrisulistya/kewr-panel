use sysinfo::{CpuExt, System, SystemExt, DiskExt};

pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disk_total: u64,
    pub disk_used: u64,
}

pub fn get_system_info(sys: &mut System) -> SystemInfo {
    sys.refresh_all();

    let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    let disk_total: u64 = sys.disks().iter().map(|disk| disk.total_space()).sum();
    let disk_used: u64 = sys.disks().iter().map(|disk| disk.total_space() - disk.available_space()).sum();

    SystemInfo {
        cpu_usage,
        memory_total,
        memory_used,
        disk_total,
        disk_used,
    }
}