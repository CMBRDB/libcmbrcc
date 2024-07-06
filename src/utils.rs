use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        use std::ptr;
        use windows::Win32::System::SystemInformation::{
            GlobalMemoryStatusEx, MEMORYSTATUSEX
        };
    } else if #[cfg(target_os = "linux")] {
        use std::fs;
    } else if #[cfg(target_os = "macos")] {
        use std::process::Command;
    }

}

pub fn get_free_memory() -> Option<u64> {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            unsafe {
                let mut mem_info = MEMORYSTATUSEX {
                    dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                    ..Default::default()
                }

                return GlobalMemoryStatusEx(&mut mem_info) != 0 {
                    Some(mem_info.ullAvailPhys / 1024)
                } else {
                    None
                };
            }
        } else if #[cfg(target_os = "linux")] {
            let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
            for line in meminfo.lines() {
                if line.starts_with("MemFree:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return parts[1].parse().ok();
                    }
                }
            }

            return None;
        } else if #[cfg(target_os = "macos")] {
            let output = Command::new("sysctl")
                .arg("vm.page_free_count")
                .output()
                .ok()?;

            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(pages_free) = parts[1].parse::<u64>() {
                    // macOS has 4096 bytes per page
                    return Some(pages_free * 4096 / 1024);
                }
            }

            return None;
        } else {
            // TODO: Support getting free mem amount for *BSD and maybe ios/android
            return None;
        }
    }
}
