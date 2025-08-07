use std::process;

use bigdecimal::ToPrimitive;
use sysinfo::{Pid, System};

/// .sysinfo example
fn sysinfo_sample() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("System:{}", system.available_memory());

    println!("Operating System: {:?}", System::name());
    println!("OS Version: {:?}", System::os_version());
    println!("OS Arch: {:?}", System::cpu_arch());
    println!("Distribution ID,{:?}", System::distribution_id());
    println!("Kernel Version: {:?}", System::kernel_version());
    println!("Long OS Version: {:?}", System::long_os_version());
}

/// process stats sample
fn process_stats_sample() {
    let mut system = System::new_all();
    system.refresh_all();

    let pid = process::id();

    if let Some(process) = system.process(Pid::from_u32(pid)) {
        println!("id: {:?}", process.pid());
        println!("name:{:?}", process.name());
        println!("cpu: {:?}", process.cpu_usage());
        println!("memory: {:?}", process.memory());
        println!("start time:{:?}", process.start_time());
        println!("run time: {:?}", process.run_time());
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
}
