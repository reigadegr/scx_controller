use crate::utils::node_reader::lock_value;
use anyhow::{Result, anyhow};
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use std::{fs, path::Path};

static GOVS: Lazy<HashSet<Vec<u8>>> = Lazy::new(|| read_cgroup_dir().expect("Unsupport device"));

pub fn get_govs() -> &'static HashSet<Vec<u8>> {
    &GOVS
}

fn read_cgroup_dir() -> Result<HashSet<Vec<u8>>> {
    let task_dir = Path::new("/sys/devices/system/cpu/cpufreq/");
    let entries = fs::read_dir(task_dir)
        .map_err(|e| anyhow!("Cannot read task_dir: {}", e))?
        .filter_map(|entry| {
            entry.ok().and_then(|entry| {
                entry.file_name().into_string().ok().map(|name| {
                    let path = task_dir.join(name).join("scaling_governor\0");
                    path.to_str().unwrap_or("").as_bytes().to_vec()
                })
            })
        })
        .collect::<HashSet<_>>();
    Ok(entries)
}

pub fn set_governor(msg: &[u8]) {
    for path in get_govs() {
        lock_value(path, msg);
    }
}
