
use crate::config::ConfigAbstract;

#[derive(Serialize, Deserialize)]
pub struct TabsConfig {

    tabs: Vec<TabConfig>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TabConfig {

    name: String,
    items: Vec<ItemConfig>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ItemConfig {

    name: String,
    path: String,
}

impl Default for TabsConfig {

    fn default() -> TabsConfig {

        let welcome_tab = TabConfig {
            name: String::from("Welcome"),
            items: vec![],
        };

        TabsConfig {
            tabs: vec![welcome_tab],
        }
    }
}

impl ConfigAbstract for TabsConfig {

    fn parse_toml(toml: &toml::Value) -> Option<TabsConfig> {

        if let Some(v) = toml.get("tab").and_then(|tabs| tabs.as_array()) {

            let tabs = v.iter().filter_map(|tab| {
                TabConfig::parse_toml(&tab)
            }).collect();

            let config = TabsConfig {
                tabs
            };

            Some(config)
        } else {
            None
        }
    }
}

impl ConfigAbstract for TabConfig {

    fn parse_toml(toml: &toml::Value) -> Option<TabConfig> {

        let name = toml.get("name")
            .and_then(|name| name.as_str())?.to_owned();

        if let Some(v) = toml.get("item").and_then(|items| items.as_array()) {

            let items = v.iter().filter_map(|item| {
                ItemConfig::parse_toml(&item)
            }).collect();

            let config = TabConfig {
                name, items
            };

            Some(config)
        } else {
            None
        }
    }
}

impl ConfigAbstract for ItemConfig {

    fn parse_toml(toml: &toml::Value) -> Option<ItemConfig> {

        let name = toml.get("name")
            .and_then(|name| name.as_str())?.to_owned();
        let path = toml.get("path")
            .and_then(|path| path.as_str())?.to_owned();

        let config = ItemConfig {
            name, path,
        };
        Some(config)
    }
}
