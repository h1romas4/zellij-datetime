use kdl::KdlDocument;
use linked_hash_map::LinkedHashMap;

pub struct PluginConfig {
    timezone: LinkedHashMap<String, i32>,
    default_timezone: String,
}

impl Default for PluginConfig {
    fn default() -> Self {
        let default_timezone = "UTC";
        let mut timezone: LinkedHashMap<String, i32> = LinkedHashMap::new();
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

    pub fn get_timezone_next(&self, timezone: &str) -> String {
        let mut iter = self.timezone.iter();
        let mut next_timezone= None;
        while let Some((key, _v)) = iter.next() {
            if key == timezone {
                next_timezone = iter.next().map(|(k, _)| k);
            }
        }
        let next_timezone = match next_timezone {
            Some(timezone) => timezone,
            None => self.timezone.keys().next().unwrap() // first key
        };
        next_timezone.to_string()
    }

    pub fn get_timezone_offset(&self, timezone: &str) -> i32 {
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
