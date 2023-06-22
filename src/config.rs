use kdl::KdlDocument;
use linked_hash_map::LinkedHashMap;

pub struct Config {
    timezone: LinkedHashMap<String, i32>,
    default_timezone: String,
}

impl Default for Config {
    fn default() -> Self {
        let default_timezone = "UTC";
        let mut timezone: LinkedHashMap<String, i32> = LinkedHashMap::new();
        // default config
        timezone.insert(default_timezone.to_string(), 0);
        timezone.insert("PDT".to_string(), -7);
        timezone.insert("JST".to_string(), 9);
        Config {
            timezone,
            default_timezone: default_timezone.to_string(),
        }
    }
}

impl Config {
    pub fn get_defalut_timezone(&self) -> String {
        self.default_timezone.to_string()
    }

    pub fn get_timezone_next(&self, timezone: &str) -> String {
        let mut iter = self.timezone.keys();
        let mut next = None;
        while let Some(k) = iter.next() {
            if k == timezone {
                next = iter.next();
            }
        }
        let timezone = match next {
            Some(next) => next,
            None => self.timezone.keys().next().unwrap(), // first key
        };
        timezone.to_string()
    }

    pub fn get_timezone_prev(&self, timezone: &str) -> String {
        let mut prev = None;
        for k in self.timezone.keys() {
            if k == timezone {
                break;
            }
            prev = Some(k);
        }
        let timezone = match prev {
            Some(prev) => prev,
            None => self.timezone.keys().last().unwrap() , // last key
        };
        timezone.to_string()
    }

    pub fn get_timezone_offset(&self, timezone: &str) -> i32 {
        match self.timezone.get(timezone) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub fn load_config(&mut self, setting: &str) {
        let mut config_timezone: LinkedHashMap<String, i32> = LinkedHashMap::new();
        if let Ok(doc) = setting.parse::<KdlDocument>() {
            // timezone tree (TODO: using KQL)
            if let Some(timezone) = doc.get("timezone") {
                if let Some(children) = timezone.children() {
                    for node in children.nodes() {
                        if node.name().value() == "define" && node.entries().len() >= 2 {
                            if let Ok(offset) = node.entries()[1].to_string().trim().parse() {
                                config_timezone.insert(
                                    node.entries()[0].to_string().trim().replace('"', ""),
                                    offset,
                                );
                            }
                        }
                    }
                }
            }
            // override defalut config
            if !config_timezone.is_empty() {
                self.timezone = config_timezone;
            }
            // default timezone
            if let Some(defalut_timezone) = doc.get_arg("defalut_timezone") {
                let timezone = defalut_timezone.to_string().trim().replace('"', "");
                if self.timezone.contains_key(&timezone) {
                    self.default_timezone = timezone;
                } else {
                    // first key
                    self.default_timezone = self.timezone.keys().next().unwrap().to_string();
                }
            }
        }
    }
}
