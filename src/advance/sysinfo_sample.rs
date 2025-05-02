use sysinfo::System;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysinfo_sample() {
        sysinfo_sample();
    }
}
