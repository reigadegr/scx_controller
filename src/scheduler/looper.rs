use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    config::PROFILE,
    governor::set_governor,
    utils::node_reader::{lock_value, unlock_value},
};
use compact_str::CompactString;
use libc::pid_t;
use likely_stable::unlikely;
use log::info;

pub struct Looper {
    pub activity_utils: ActivityUtils,
    pub global_package: CompactString,
    pub pid: pid_t,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            activity_utils,
            global_package: CompactString::new(""),
            pid: -1,
        }
    }

    async fn wait_until_exit(&mut self) {
        loop {
            let pid = self.activity_utils.top_app_utils.get_top_pid();
            if unlikely(pid != self.pid) {
                self.game_exit().await;
                return;
            }
            lock_value(b"/proc/hmbird_sched/heartbeat\0", b"1\0").await;
        }
    }

    async fn game_exit(&mut self) {
        set_governor(b"walt\0").await;
        unlock_value(b"/proc/hmbird_sched/scx_enable\0", b"0\0").await;
        self.pid = -1;
    }

    pub async fn enter_loop(&mut self) {
        'outer: loop {
            {
                let pid = self.activity_utils.top_app_utils.get_top_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).await.unwrap_or_default();
                self.global_package = name;
            }

            for i in &PROFILE.hmbird_config {
                for j in &i.packages {
                    if self.global_package == j {
                        info!("Detected target App: {}", self.global_package);
                        info!("dbg: {:?}", i.node_value);
                        set_governor(b"scx\0").await;
                        lock_value(b"/proc/hmbird_sched/scx_enable\0", b"1\0").await;
                        self.wait_until_exit().await;
                        continue 'outer;
                    }
                }
            }
        }
    }
}
