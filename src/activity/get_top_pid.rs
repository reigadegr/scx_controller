use crate::utils::sleep::sleep_secs;
use atoi::atoi;
use dumpsys_rs::Dumpsys;
use inotify::{Inotify, WatchMask};
use libc::pid_t;
use likely_stable::LikelyOption;
#[cfg(debug_assertions)]
use log::debug;
use log::info;
#[cfg(debug_assertions)]
use minstant::Instant;
use stringzilla::{StringZilla, sz};

#[derive(Default)]
pub struct TopPidInfo {
    pid: pid_t,
}

impl TopPidInfo {
    pub fn new(dump: &[u8]) -> Self {
        let multi_window = sz::find(dump, b"Window #1").is_some();

        let pid = if multi_window {
            dump.sz_rsplits(&b"\n")
                .find(|line| sz::find(line, b"Session{").is_some())
        } else {
            dump.sz_splits(&b"\n")
                .find(|line| sz::find(line, b"Session{").is_some())
        };
        let pid = pid
            .and_then_likely(|line| {
                line.sz_rfind(b":").and_then_likely(|pos1| {
                    line[..pos1].sz_rfind(b" ").map(|pos2| &line[pos2 + 1..])
                })
            })
            .and_then_likely(atoi::<pid_t>)
            .unwrap_or_default();
        #[cfg(debug_assertions)]
        println!("当前pid:{pid}");

        Self { pid }
    }
}

pub struct TopAppUtils {
    dumper: Dumpsys,
    inotify: Inotify,
}

impl TopAppUtils {
    pub fn new() -> Self {
        let inotify = Inotify::init().unwrap();
        inotify
            .watches()
            .add("/dev/input", WatchMask::ACCESS)
            .unwrap();

        let dumper = loop {
            match Dumpsys::new("window") {
                Some(d) => break d,
                None => sleep_secs(1),
            }
        };
        Self { dumper, inotify }
    }

    pub fn get_top_pid(&mut self) -> pid_t {
        self.set_top_pid().pid
    }

    pub fn set_top_pid(&mut self) -> TopPidInfo {
        loop {
            match self.inotify.read_events_blocking(&mut [0; 1024]) {
                Ok(_) => break,
                Err(e) => {
                    info!("Failed to read events: {e}, retrying");
                    sleep_secs(1);
                }
            }
        }
        #[cfg(debug_assertions)]
        let start = Instant::now();
        let dump = loop {
            match self.dumper.dump_to_byte::<32768>(&["visible-apps"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump windows: {e}, retrying");
                    sleep_secs(1);
                }
            }
        };
        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();
            debug!("完成时间:{end:?}");
        }

        TopPidInfo::new(&dump)
    }
}
