
use std::time::Duration;

use crate::config::ConfigAbstract;

#[derive(Serialize, Deserialize)]
pub struct SettingConfig {

    pub is_close_after_game_launch: bool,
    pub tick_rate: Duration,
}

impl Default for SettingConfig {

    fn default() -> SettingConfig {

        SettingConfig {
            is_close_after_game_launch: false,
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl ConfigAbstract for SettingConfig {

    fn parse_toml(toml: &toml::Value) -> Option<SettingConfig> {

        if let Some(v) = toml.get("setting") {

            let mut config = SettingConfig::default();

            if let Some(v) = v.get("is_close_after_game_launch").and_then(|v| v.as_bool()) {
                config.is_close_after_game_launch = v;
            }

            if let Some(v) = v.get("tick_rate") {
                config.tick_rate = toml::from_str(v.as_str()
                    .expect("Failed to read tick_rate content"))
                    .expect("Failed to convert tick_rate content");
            }

            Some(config)
        } else {
            None
        }
    }
}
