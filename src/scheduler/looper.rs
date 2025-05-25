use crate::governor::set_governor;
use crate::utils::node_reader::write_to_byte;
use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    config::PROFILE,
    utils::sleep::sleep_secs,
};
use compact_str::CompactString;
use libc::pid_t;
use likely_stable::unlikely;

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

    fn wait_until_exit(&mut self) {
        loop {
            sleep_secs(1);
            let pid = self.activity_utils.top_app_utils.get_top_pid();
            if unlikely(pid != self.pid) {
                self.game_exit();
                return;
            }
        }
    }

    fn game_exit(&mut self) {
        set_governor::<4>(b"walt\0");
        write_to_byte(b"/proc/hmbird_sched/scx_enable\0",b"0\0")
        self.pid = -1;
    }

    pub fn enter_loop(&mut self) {
        'outer: loop {
            sleep_secs(1);
            {
                let pid = self.activity_utils.top_app_utils.get_top_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).unwrap_or_default();
                self.global_package = name;
            }

            for i in &PROFILE.packages {
                if self.global_package == i {
                    set_governor::<3>(b"scx\0");
                    write_to_byte(b"/proc/hmbird_sched/scx_enable\0", b"1\0");
                    self.wait_until_exit();
                    continue 'outer;
                }
            }
        }
    }
}
