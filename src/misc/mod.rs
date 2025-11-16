pub mod logger;
use crate::utils::node_reader::write_to_byte;
use itoa::Buffer;
use libc::{getpid, pthread_self, pthread_setname_np};
use likely_stable::unlikely;
use log::info;
use logger::{init_log, log_metainfo};

pub async fn init_misc() {
    working_in_background().await;
    init_log();
    set_main_thread_name(b"proc_ctrl\0");
    log_metainfo();
    print_misc();
}

async fn working_in_background() {
    unsafe {
        let pid = getpid();
        let mut itoa_buf = Buffer::new();
        let pid = itoa_buf.format(pid).as_bytes();
        let _ = write_to_byte(b"/dev/cpuset/background/tasks\0", pid).await;
    }
}

fn set_main_thread_name(name: &[u8]) {
    let thread_name = if unlikely(name.len() > 15) {
        &name[..15]
    } else {
        name
    };

    unsafe {
        pthread_setname_np(pthread_self(), thread_name.as_ptr());
    }
}

fn print_misc() {
    info!("免费软件，禁止商用");
    info!("Free software, not for commercial use.");
    info!("开源地址: https://github.com/reigadegr/scx_controller");
}
