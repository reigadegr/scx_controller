use anyhow::{Result, anyhow};
use hashbrown::HashSet;
use libc::{O_WRONLY, c_void, open, write};
use likely_stable::unlikely;
use once_cell::sync::Lazy;
use std::ffi::CString;
use std::{fs, path::Path};

static GOVS: Lazy<HashSet<i32>> = Lazy::new(|| read_cgroup_dir().expect("Unsupport device"));

pub fn get_govs() -> &'static HashSet<i32> {
    &GOVS
}

fn read_cgroup_dir() -> Result<HashSet<i32>> {
    let task_dir = Path::new("/sys/devices/system/cpu/cpufreq/");
    let entries = fs::read_dir(task_dir)
        .map_err(|e| anyhow!("Cannot read task_dir: {}", e))?
        .filter_map(|entry| {
            entry.ok().and_then(|entry| {
                entry.file_name().into_string().ok().map(|name| {
                    let path = task_dir.join(name).join("scaling_governor");
                    let path = path.to_str().unwrap_or("");
                    let file = CString::new(path).unwrap();
                    unsafe { open(file.as_ptr(), O_WRONLY, 0o664) }
                })
            })
        })
        .collect::<HashSet<_>>();
    Ok(entries)
}

pub fn set_governor<const N: usize>(msg: &[u8]) {
    for fd in get_govs() {
        let bytes_write = unsafe { write(*fd, msg.as_ptr().cast::<c_void>(), N) };
        if unlikely(bytes_write == -1) {
            continue;
        }
    }
}
