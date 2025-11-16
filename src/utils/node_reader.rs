use super::guard::FileGuard;
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use core::ptr::copy_nonoverlapping;
use itoa::Buffer;
use libc::{O_CREAT, O_TRUNC, O_WRONLY, c_void, chmod, open, pid_t, write};
use likely_stable::unlikely;
use std::{io::ErrorKind, str::from_utf8};
use stringzilla::sz;
use tokio::{fs::File, io::AsyncReadExt};

pub fn lock_value(path: &[u8], value: &[u8]) {
    unsafe {
        let _ = chmod(path.as_ptr(), 0o666);
        let _ = write_to_byte(path, value);
        let _ = chmod(path.as_ptr(), 0o444);
    }
}

pub fn unlock_value(path: &[u8], value: &[u8]) {
    unsafe {
        let _ = chmod(path.as_ptr(), 0o666);
        let _ = write_to_byte(path, value);
    }
}

pub async fn read_file<const N: usize>(file: &[u8]) -> Result<CompactString> {
    let buffer = read_to_byte::<N>(file).await?;
    let pos = sz::find(buffer, b"\0");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub async fn read_to_byte<const N: usize>(file: &[u8]) -> Result<[u8; N]> {
    let end = sz::find(file, b"\0").unwrap_or(N);
    let file = &file[..end];
    let file = from_utf8(file)?;

    let mut file = File::open(file)
        .await
        .map_err(|e| anyhow!("Cannot open file: {e}"))?;

    let mut buffer = [0u8; N];

    match file.read_exact(&mut buffer).await {
        Ok(_) => Ok(buffer),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(buffer),
        Err(e) => Err(e.into()),
    }
}

pub fn write_to_byte(file: &[u8], msg: &[u8]) -> Result<()> {
    unsafe {
        let fd = open(file.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC);
        if unlikely(fd == -1) {
            return Err(anyhow!("Cannot open file."));
        }
        let _fd_guard = FileGuard::new(fd);
        let bytes_write = write(fd, msg.as_ptr().cast::<c_void>(), msg.len());

        if unlikely(bytes_write == -1) {
            return Err(anyhow!("Cannot write file."));
        }
    }
    Ok(())
}

pub fn get_proc_path<const N: usize, const L: usize>(id: pid_t, file: &[u8]) -> [u8; N] {
    let mut buffer = [0u8; N];
    buffer[0..6].copy_from_slice(b"/proc/");

    let mut itoa_buf = Buffer::new();
    let id = itoa_buf.format(id).as_bytes();

    let id_length = id.len();

    unsafe {
        copy_nonoverlapping(id.as_ptr(), buffer.as_mut_ptr().add(6), id_length);
        copy_nonoverlapping(file.as_ptr(), buffer.as_mut_ptr().add(6 + id_length), L);
    }
    buffer
}
