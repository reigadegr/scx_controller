pub mod format_profile;
use crate::utils::node_reader::{read_file, write_to_byte};
use compact_str::CompactString;
use format_profile::format_toml;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use toml::Table;

pub static PROFILE: LazyLock<Config> = LazyLock::new(|| {
    let profile_path = b"/data/adb/modules/hmbird_controller/app_config.toml\0";
    let profile = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(read_file::<65536>(profile_path))
    })
    .unwrap();

    let format_rs = format_toml(&profile);
    let profile: Config = toml::from_str(&profile).unwrap();

    let _ = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(write_to_byte(profile_path, format_rs.as_bytes()))
    });

    profile
});

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub hmbird_config: Vec<HmbirdConfig>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct HmbirdConfig {
    pub packages: Box<[CompactString]>,
    pub node_value: Table,
}
