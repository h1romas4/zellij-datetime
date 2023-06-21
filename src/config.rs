use kdl::KdlDocument;
use std::collections::HashMap;

pub struct PluginConfig {
    timezone: HashMap<String, i32>,
    default_timezone: String,
}

impl Default for PluginConfig {
    fn default() -> Self {
        let default_timezone = "UTC";
        let mut timezone: HashMap<String, i32> = HashMap::new();
        timezone.insert(default_timezone.to_string(), 0);
        PluginConfig {
            timezone,
            default_timezone: default_timezone.to_string(),
        }
    }
}

impl PluginConfig {
    pub fn get_defalut_timezone(&self) -> String {
        self.default_timezone.to_string()
    }

    pub fn get_timezone_define_count(&self) -> usize {
        self.timezone.len()
    }

    pub fn get_timezone(&self, timezone: &str) -> i32 {
        match self.timezone.get(timezone) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub fn load_config(&mut self, setting: &str) {
        if let Ok(doc) = setting.parse::<KdlDocument>() {
            // timezone tree (TODO: using KQL)
            if let Some(timezone) = doc.get("timezone") {
                if let Some(children) = timezone.children() {
                    for node in children.nodes() {
                        if node.name().value() == "define" && node.entries().len() >= 2 {
                            if let Ok(offset) = node.entries()[1].to_string().trim().parse() {
                                self.timezone.insert(
                                    node.entries()[0].to_string().trim().replace('"', ""),
                                    offset,
                                );
                            }
                        }
                    }
                }
            }
            // default timezone
            if let Some(defalut_timezone) = doc.get_arg("defalut_timezone") {
                self.default_timezone = defalut_timezone.to_string().trim().replace('"', "");
            }
        }
    }
}
