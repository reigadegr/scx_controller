use crate::utils::node_reader::{get_proc_path, read_to_byte};
use anyhow::Result;
use compact_str::CompactString;
use libc::pid_t;
use stringzilla::sz;

pub fn get_process_name(pid: pid_t) -> Result<CompactString> {
    let cmdline = get_proc_path::<32, 8>(pid, b"/cmdline");

    let buffer = read_to_byte::<128>(&cmdline)?;

    let pos = sz::find(buffer, b":");
    if let Some(sub) = pos {
        let buffer = &buffer[..sub];
        let buffer = CompactString::from_utf8(buffer)?;
        return Ok(buffer);
    }

    let pos = sz::find(buffer, b"\0");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);

    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}
