
pub mod manifest;
pub mod tab;
pub mod setting;

use crate::config::tab::{ TabsConfig, TabConfig, ItemConfig };
use crate::config::setting::SettingConfig;
use crate::config::manifest::MANIFEST_CONFIG_NAME;

use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::{ Read, Write };

pub trait ConfigAbstract where Self: Sized {

    fn parse_toml(toml: &toml::Value) -> Option<Self>;
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EngineConfig {

    pub tabs: TabsConfig,
    pub setting: SettingConfig,
}

impl ConfigAbstract for EngineConfig {

    fn parse_toml(toml: &toml::Value) -> Option<EngineConfig> {

        let config = EngineConfig {
            tabs   : TabsConfig::parse_toml(toml)?,
            setting: SettingConfig::parse_toml(toml)?,
        };

        Some(config)
    }
}

impl EngineConfig {

    pub fn init() -> Option<EngineConfig> {

        EngineConfig::search_manifest()
            .and_then(EngineConfig::read_manifest)
            .and_then(|content| toml::from_str(&content).ok())
    }

    pub fn write_manifest(&self) -> Result<(), failure::Error> {

        let content = toml::to_string_pretty(self)?;
        let cwd = env::current_dir()?;

        let mut file = fs::File::create(cwd.join(MANIFEST_CONFIG_NAME))?;
        let _ = file.write(content.as_bytes())?;

        Ok(())
    }

    fn search_manifest() -> Option<PathBuf> {

        let cwd = env::current_dir().ok()?;
        let mut current = cwd.as_path();

        loop {

            let manifest = current.join(MANIFEST_CONFIG_NAME);
            if fs::metadata(&manifest).is_ok() {
                // succeed to find manifest configuration file.
                return Some(manifest)
            }

            // continute search its parent directory.
            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        None
    }

    /// Read the manifest file content to string.
    fn read_manifest(at_path: PathBuf) -> Option<String> {

        let mut file_handle = fs::File::open(at_path).ok()?;
        let mut contents = String::new();
        file_handle.read_to_string(&mut contents).ok()?;

        Some(contents)
    }

    pub fn update(&mut self, op: ConfigOp) -> crate::THLError {

        match op {
            | ConfigOp::None => return Ok(()),
            | ConfigOp::AppendTab { config } => {
                self.tabs.tabs.push(config);
            },
            | ConfigOp::RemoveTab { tab_index } => {
                self.tabs.tabs.remove(tab_index);
            },
            | ConfigOp::AppendGame { tab_index, config } => {
                self.tabs.tabs[tab_index].items.push(config);
            },
            | ConfigOp::RemoveGame { tab_index, item_index } => {
                self.tabs.tabs[tab_index].items.remove(item_index);
            },
        }

        // update local toml file.
        self.write_manifest()
    }
}

pub enum ConfigOp {

    None,
    AppendTab { config: TabConfig },
    RemoveTab { tab_index: usize },
    AppendGame { tab_index: usize, config: ItemConfig },
    RemoveGame { tab_index: usize, item_index: usize },
}
