use std::collections::BTreeMap;

use csscolorparser::Color;
use linked_hash_map::LinkedHashMap;

static DEFAULT_TIMEZONE: &str = "UTC";
static DEFAULT_BACKGROUND_COLOR: &str = "#0080a0";
static DEFAULT_FOREGROUND_COLOR: &str = "#ffffff";
static DEFAULT_PANE_COLOR: &str = "#1e1e1e";

pub struct Config {
    timezone: LinkedHashMap<String, i32>,
    default_timezone: String,
    backgound_color: Option<(u8, u8, u8)>,
    foreground_color: Option<(u8, u8, u8)>,
    pane_color: Option<(u8, u8, u8)>,
    enable_right_click: bool,
}

impl Default for Config {
    fn default() -> Self {
        let default_timezone = DEFAULT_TIMEZONE;
        let mut timezone: LinkedHashMap<String, i32> = LinkedHashMap::new();
        timezone.insert(default_timezone.to_string(), 0);
        Config {
            timezone,
            default_timezone: default_timezone.to_string(),
            backgound_color: Some(parse_color(DEFAULT_BACKGROUND_COLOR).unwrap()),
            foreground_color: Some(parse_color(DEFAULT_FOREGROUND_COLOR).unwrap()),
            pane_color: Some(parse_color(DEFAULT_PANE_COLOR).unwrap()),
            enable_right_click: false,
        }
    }
}

impl Config {
    pub fn get_default_timezone(&self) -> String {
        self.default_timezone.to_string()
    }

    pub fn get_next_timezone(&self, timezone: &str) -> String {
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

    pub fn get_prev_timezone(&self, timezone: &str) -> String {
        let mut prev = None;
        for k in self.timezone.keys() {
            if k == timezone {
                break;
            }
            prev = Some(k);
        }
        let timezone = match prev {
            Some(prev) => prev,
            None => self.timezone.keys().last().unwrap(), // last key
        };
        timezone.to_string()
    }

    pub fn get_timezone_offset(&self, timezone: &str) -> i32 {
        match self.timezone.get(timezone) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub fn get_backgound_color(&self) -> Option<(u8, u8, u8)> {
        self.backgound_color
    }

    pub fn get_foreground_color(&self) -> Option<(u8, u8, u8)> {
        self.foreground_color
    }

    pub fn get_pane_color(&self) -> Option<(u8, u8, u8)> {
        self.pane_color
    }

    pub fn get_enable_right_click(&self) -> bool {
        self.enable_right_click
    }

    pub fn configuration(&mut self, configuration: &BTreeMap<String, String>) {
        let mut timezone: LinkedHashMap<String, i32> = LinkedHashMap::new();
        let mut default_timezone: Option<String> = None;

        for (key, value) in configuration {
            match key.as_str() {
                "timezone1" | "timezone2" | "timezone3" | "timezone4" | "timezone5"
                | "timezone6" | "timezone7" | "timezone8" | "timezone9" => {
                    let value: Vec<&str> = value.split('/').collect();
                    if value.len() == 2 {
                        if let Ok(offset) = value[1].parse() {
                            timezone.insert(value[0].trim().to_string(), offset);
                        }
                    }
                }
                "default_timezone" => {
                    default_timezone = Some(value.trim().to_string());
                }
                "background_color" => {
                    if let Ok(color) = parse_color(value) {
                        self.backgound_color = Some((color.0, color.1, color.2));
                    }
                }
                "foreground_color" => {
                    if let Ok(color) = parse_color(value) {
                        self.foreground_color = Some((color.0, color.1, color.2));
                    }
                }
                "pane_color" => {
                    if let Ok(color) = parse_color(value) {
                        self.pane_color = Some((color.0, color.1, color.2));
                    }
                }
                "enable_right_click" => {
                    self.enable_right_click = value.trim().parse().unwrap_or(false);
                }
                _ => {}
            }
        }

        if !timezone.is_empty() {
            self.default_timezone = timezone.keys().next().unwrap().to_string();
            if let Some(default_timezone) = default_timezone {
                if timezone.contains_key(&default_timezone) {
                    self.default_timezone = default_timezone;
                }
            }
            self.timezone = timezone;
        }
    }
}

fn parse_color(color: &str) -> Result<(u8, u8, u8), &str> {
    if let Ok(color) = color.to_string().trim().parse::<Color>() {
        let color = color.to_rgba8();
        return Ok((color[0], color[1], color[2]));
    }
    Err("Color format parse error")
}
