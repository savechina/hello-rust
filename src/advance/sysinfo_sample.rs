use sysinfo::{Components, Disks, Networks, Pid, System};

/// 系统信息概览
/// 获取操作系统、CPU、内存等基本信息
pub fn sysinfo_sample() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("=== 系统信息概览 ===\n");

    // 操作系统信息
    println!("操作系统：{:?}", System::name());
    println!("系统版本：{:?}", System::os_version());
    println!("内核版本：{:?}", System::kernel_version());
    println!("系统架构：{:?}", System::cpu_arch());
    println!("发行版 ID：{:?}", System::distribution_id());
    println!("完整版本：{:?}\n", System::long_os_version());

    // CPU 信息
    println!("=== CPU 信息 ===\n");
    if let Some(cpu) = system.cpus().first() {
        println!("CPU 品牌：{:?}", cpu.brand());
        println!("CPU 频率：{} MHz\n", cpu.frequency());
    }
    println!(
        "CPU 核心数：{}\n",
        System::physical_core_count().unwrap_or(0)
    );

    // 内存信息
    println!("=== 内存信息 ===\n");
    let total_memory = system.total_memory();
    let available_memory = system.available_memory();
    let used_memory = total_memory - available_memory;

    println!("总内存：{} MB", total_memory / 1024 / 1024);
    println!("可用内存：{} MB", available_memory / 1024 / 1024);
    println!("已用内存：{} MB", used_memory / 1024 / 1024);
    println!(
        "内存使用率：{:.1}%\n",
        (used_memory as f64 / total_memory as f64) * 100.0
    );

    // 交换空间
    println!("总交换空间：{} MB", system.total_swap() / 1024 / 1024);
    println!("已用交换空间：{} MB\n", system.used_swap() / 1024 / 1024);
}

/// 进程统计信息
/// 获取特定进程的 CPU、内存使用情况
pub fn process_stats_sample() {
    let mut system = System::new_all();
    system.refresh_all();

    let pid = std::process::id();

    println!("=== 进程统计 (PID: {}) ===\n", pid);

    if let Some(process) = system.process(Pid::from_u32(pid)) {
        println!("进程名：{:?}", process.name());
        println!("进程 ID：{:?}", process.pid());
        println!("父进程 ID：{:?}", process.parent());
        println!("CPU 使用率：{:.1}%", process.cpu_usage());
        println!("内存使用：{} KB", process.memory());
        println!("虚拟内存：{} KB", process.virtual_memory());
        println!("启动时间：{:?}", process.start_time());
        println!("运行时间：{} 秒", process.run_time());
        println!("进程状态：{:?}", process.status());
    } else {
        println!("未找到进程 {}", pid);
    }
}

/// 磁盘信息
/// 获取所有磁盘和分区信息
pub fn disk_info_sample() {
    let disks = Disks::new_with_refreshed_list();

    println!("=== 磁盘信息 ===\n");

    for disk in &disks {
        println!("设备：{:?}", disk.name());
        println!("挂载点：{:?}", disk.mount_point());
        println!("文件系统：{:?}", disk.file_system());
        println!("总容量：{} GB", disk.total_space() / 1024 / 1024 / 1024);
        println!(
            "可用空间：{} GB",
            disk.available_space() / 1024 / 1024 / 1024
        );
        println!(
            "使用率：{:.1}%",
            ((disk.total_space() - disk.available_space()) as f64 / disk.total_space() as f64)
                * 100.0
        );
        println!("可移动：{}", disk.is_removable());
        println!();
    }
}

/// 网络接口信息
/// 获取所有网络接口的收发数据
pub fn network_info_sample() {
    let networks = Networks::new_with_refreshed_list();

    println!("=== 网络接口信息 ===\n");

    for (interface_name, data) in &networks {
        println!("接口：{}", interface_name);
        println!("接收数据：{} MB", data.received() / 1024 / 1024);
        println!("发送数据：{} MB", data.transmitted() / 1024 / 1024);
        println!("接收包数：{}", data.packets_received());
        println!("发送包数：{}", data.packets_transmitted());
        println!();
    }
}

/// 硬件组件信息
/// 获取传感器温度等硬件信息
pub fn hardware_components_sample() {
    let components = Components::new_with_refreshed_list();

    println!("=== 硬件组件信息 ===\n");

    for component in &components {
        println!("组件：{:?}", component.label());
        println!("当前温度：{:?}°C", component.temperature());
        println!("最高温度：{:?}°C", component.max());
        println!("危险温度：{:?}°C\n", component.critical());
    }
}

/// 所有进程列表
/// 获取并显示所有运行中的进程
pub fn process_list_sample() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("=== 进程列表 (前 10 个) ===\n");

    let mut processes: Vec<_> = system.processes().values().collect();
    processes.sort_by_key(|p| p.memory());
    processes.reverse();

    for (i, process) in processes.iter().take(10).enumerate() {
        println!(
            "{}. {} (PID: {}) - 内存：{} KB - CPU: {:.1}%",
            i + 1,
            process.name().to_str().unwrap_or("unknown"),
            process.pid(),
            process.memory(),
            process.cpu_usage()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysinfo_sample() {
        sysinfo_sample();
    }

    #[test]
    fn test_process_stats_sample() {
        process_stats_sample();
    }

    #[test]
    fn test_disk_info_sample() {
        disk_info_sample();
    }

    #[test]
    fn test_network_info_sample() {
        network_info_sample();
    }

    #[test]
    fn test_hardware_components_sample() {
        hardware_components_sample();
    }

    #[test]
    fn test_process_list_sample() {
        process_list_sample();
    }
}
