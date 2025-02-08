// This file demonstrates the usage of `cfg_if` macro to conditionally compile code based on various configuration options.

/// `cfg_if` is a macro that allows you to define code blocks based on various configuration options.
cfg_if::cfg_if! {
    if #[cfg(unix)] {
        fn foo() { /* unix specific functionality */
            println!("unix specific functionality");
         }
    } else if #[cfg(target_pointer_width = "32")] {
        fn foo() { /* non-unix, 32-bit functionality */
            println!("non-unix, 32-bit functionality");
         }
    } else {
        fn foo() { /* fallback implementation */
            println!("fallback implementation");
        }
    }
}

///cfg_if Sample Example
fn cfg_if_sample() {
    // Example 1: Platform specific code (OS detection)

    println!("--- Platform Specific Example ---");

    cfg_if! {
        if #[cfg(target_os = "windows")] {
            println!("You are running on Windows!");
            // Windows-specific code here
            let os_message = "Windows specific message";
            windows_functionality();
            println!("OS Message: {}", os_message);
        } else if #[cfg(target_os = "linux")] {
            println!("You are running on Linux!");
            // Linux-specific code here
            let os_message = "Linux specific message";
            linux_functionality();
            println!("OS Message: {}", os_message);
        } else if #[cfg(target_os = "macos")] {
            println!("You are running on macOS!");
            // macOS-specific code here
            let os_message = "macOS specific message";
            macos_functionality();
            println!("OS Message: {}", os_message);
        } else {
            println!("You are running on an unknown operating system (or one not explicitly handled in this example).");
            // Fallback code for other OSes
            let os_message = "Generic OS message";
            generic_functionality();
            println!("OS Message: {}", os_message);
        }
    }

    println!("\n--- Feature Flag Example ---");

    // Example 2: Feature flags (controlled via Cargo.toml features)
    cfg_if! {
        if #[cfg(feature = "my_feature_a")] {
            println!("Feature 'my_feature_a' is enabled!");
            feature_a_functionality();
        } else if #[cfg(feature = "my_feature_b")] {
            println!("Feature 'my_feature_b' is enabled!");
            feature_b_functionality();
        } else {
            println!("Neither 'my_feature_a' nor 'my_feature_b' is enabled (or no feature specified). Default behavior.");
            default_feature_functionality();
        }
    }

    println!("\n--- Target Architecture Example ---");

    // Example 3: Target Architecture specific code
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            println!("You are on x86_64 architecture!");
            x86_64_optimizations();
        }  else if #[cfg(target_arch = "aarch64")] {
            println!("You are on aarch64 architecture!");
            aarch64_optimizations();
        }  else if #[cfg(target_arch = "wasm32")] {
            println!("You are on wasm32 architecture (likely running in a browser or WASM runtime)!");
            wasm32_functionality();
        } else {
            println!("You are on a different architecture than explicitly handled (x86_64 or aarch64 or wasm32 in this example).");
            generic_architecture_functionality();
        }
    }
}

// --- Dummy functions to represent different functionalities ---

#[cfg(target_os = "windows")]
fn windows_functionality() {
    println!("Performing Windows specific actions.");
    // In real code, you would call Windows API functions or use Windows-specific libraries here.
}

#[cfg(target_os = "linux")]
fn linux_functionality() {
    println!("Performing Linux specific actions.");
    // In real code, you would call Linux system calls or use Linux-specific libraries here.
}

#[cfg(target_os = "macos")]
fn macos_functionality() {
    println!("Performing macOS specific actions.");
    // In real code, you would call macOS frameworks or use macOS-specific libraries here.
}

fn generic_functionality() {
    println!("Performing generic OS actions.");
    // Code that works across different OSes.
}

#[cfg(feature = "my_feature_a")]
fn feature_a_functionality() {
    println!("Executing Feature A functionality.");
    // Functionality related to feature "my_feature_a"
}

#[cfg(feature = "my_feature_b")]
fn feature_b_functionality() {
    println!("Executing Feature B functionality.");
    // Functionality related to feature "my_feature_b"
}

fn default_feature_functionality() {
    println!("Executing default feature functionality.");
    // Default functionality when no specific feature is enabled.
}

#[cfg(target_arch = "x86_64")]
fn x86_64_optimizations() {
    println!("Applying x86_64 specific optimizations.");
    // Optimized code for x86_64 architecture.
}

#[cfg(target_arch = "aarch64")]
fn aarch64_optimizations() {
    println!("Applying aarch64 specific optimizations.");
    // Optimized code for aarch64 architecture.
}

#[cfg(target_arch = "wasm32")]
fn wasm32_functionality() {
    println!("Executing wasm32 specific functionality.");
    // Functionality specific for wasm32 environment.
}

fn generic_architecture_functionality() {
    println!("Executing generic architecture functionality.");
    // Code that works across different architectures.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_if_sample() {
        cfg_if_sample();
    }

    #[test]
    fn test_foo_sample() {
        foo();
    }
}
