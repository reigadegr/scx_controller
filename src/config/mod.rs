pub mod format_profile;
use crate::utils::node_reader::{read_file, write_to_byte};
use compact_str::CompactString;
use format_profile::format_toml;
use serde::Deserialize;
use std::{collections::HashSet, sync::LazyLock};

pub static PROFILE: LazyLock<Config> = LazyLock::new(|| {
    let profile_path = b"/data/adb/modules/scx_controller/app_config.toml\0";
    let profile = read_file::<65536>(profile_path).unwrap();
    let format_rs = format_toml(&profile);
    let profile: Config = toml::from_str(&profile).unwrap();
    write_to_byte(profile_path, format_rs.as_bytes()).unwrap();
    profile
});

#[derive(Deserialize)]
pub struct Config {
    pub hmbird_config: HashSet<HmbirdConfig>,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct HmbirdConfig {
    pub packages: Box<[CompactString]>,
    pub cluster_separate: i32,
    pub cpu7_tl: i32,
    pub cpu_cluster_masks: i32,
    pub cpuctrl_high: i32,
    pub cpuctrl_low: i32,
    pub highres_tick_ctrl_dbg: i32,
    pub hmbird_preempt_policy: i32,
    pub hmbirdcore_debug: i32,
    pub iso_free_rescue: i32,
    pub isoctrl_high_ratio: i32,
    pub isoctrl_low_ratio: i32,
    pub isolate_ctrl: i32,
    pub misfit_ds: i32,
    pub parctrl_high_ratio: i32,
    pub parctrl_high_ratio_l: i32,
    pub parctrl_low_ratio: i32,
    pub parctrl_low_ratio_l: i32,
    pub partial_ctrl: i32,
    pub save_gov: i32,
    pub scx_shadow_tick_enable: i32,
    pub slim_for_app: i32,
    pub slim_stats: i32,
    pub watchdog_enable: i32,
}
