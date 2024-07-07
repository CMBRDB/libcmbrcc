use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        use windows::Win32::System::SystemInformation::{
            GlobalMemoryStatusEx, MEMORYSTATUSEX
        };
    } else if #[cfg(target_os = "linux")] {
        use std::fs;
    } else if #[cfg(target_os = "macos")] {
        use std::process::Command;
    } else if #[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))] {
        use std::ffi::CString;
        use std::mem;
    }
}

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
fn sysctl_by_name<T>(name: &str) -> Result<T, std::io::Error> {
    let mut value: T = unsafe { mem::zeroed() };
    let mut len = mem::size_of::<T>();

    let cname = CString::new(name).expect("CString::new failed");
    let ret = unsafe {
        libc::sysctlbyname(
            cname.as_ptr(),
            &mut value as *mut _ as *mut libc::c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    };

    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(value)
    }
}

pub fn get_free_memory() -> Option<u64> {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            unsafe {
                let mut mem_info = MEMORYSTATUSEX {
                    dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                    ..Default::default()
                };

                return if GlobalMemoryStatusEx(&mut mem_info).is_ok() {
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
        } else if #[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))] {
            let free_pages = sysctl_by_name::<libc::c_int>("vm.stats.vm.v_free_count").unwrap();
            let inactive_p = sysctl_by_name::<libc::c_int>("vm.stats.vm.v_inactive_count").unwrap();
            let page_size  = sysctl_by_name::<libc::c_int>("hw.pagesize").unwrap();

            let free_memory = (free_pages as u64 + inactive_p as u64) * page_size as u64;

            return Some(free_memory / 1024);
        } else {
            // TODO(#25): Support getting free mem amount for *BSD and maybe ios/android
            return None;
        }
    }
}
