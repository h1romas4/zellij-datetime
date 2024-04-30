use std::collections::BTreeMap;

use csscolorparser::Color;
use linked_hash_map::LinkedHashMap;

static DEFAULT_TIMEZONE: &str = "UTC";
static DEFAULT_BACKGROUND_COLOR: &str = "#0080a0";
static DEFAULT_FOREGROUND_COLOR: &str = "#ffffff";
static DEFAULT_PANE_COLOR: &str = "#1e1e1e";
static DEFAULT_ARROW_SEPARATOR_1: &str = "";
static DEFAULT_ARROW_SEPARATOR_2: &str = "";
static DEFAULT_ARROW_SEPARATOR_3: &str = "";
static DEFAULT_TEXT_ALIGN: &str = "right";

pub struct Config {
    timezone: LinkedHashMap<String, f64>,
    default_timezone: String,
    date_format: String,
    time_format: String,
    background_color: (u8, u8, u8),
    foreground_color: (u8, u8, u8),
    pane_color: (u8, u8, u8),
    enable_right_click: bool,
    separator: (String, String, String),
    padding_adjust: i32,
    text_align: String,
    enable_debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        let default_timezone = DEFAULT_TIMEZONE;
        let mut timezone: LinkedHashMap<String, f64> = LinkedHashMap::new();
        timezone.insert(default_timezone.to_string(), 0.0);
        Config {
            timezone,
            default_timezone: default_timezone.to_string(),
            date_format: "%Y-%m-%d %a".to_string(),
            time_format: "%H:%M".to_string(),
            background_color: parse_color(DEFAULT_BACKGROUND_COLOR).unwrap(),
            foreground_color: parse_color(DEFAULT_FOREGROUND_COLOR).unwrap(),
            pane_color: parse_color(DEFAULT_PANE_COLOR).unwrap(),
            enable_right_click: false,
            separator: (
                DEFAULT_ARROW_SEPARATOR_1.to_string(),
                DEFAULT_ARROW_SEPARATOR_2.to_string(),
                DEFAULT_ARROW_SEPARATOR_3.to_string(),
            ),
            padding_adjust: 0,
            text_align: DEFAULT_TEXT_ALIGN.to_string(),
            enable_debug: false,
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

    pub fn get_timezone_offset(&self, timezone: &str) -> f64 {
        match self.timezone.get(timezone) {
            Some(value) => *value,
            None => 0.0,
        }
    }

    pub fn get_date_format(&self) -> &String {
        &self.date_format
    }

    pub fn get_time_format(&self) -> &String {
        &self.time_format
    }

    pub fn get_background_color(&self) -> (u8, u8, u8) {
        self.background_color
    }

    pub fn get_foreground_color(&self) -> (u8, u8, u8) {
        self.foreground_color
    }

    pub fn get_pane_color(&self) -> (u8, u8, u8) {
        self.pane_color
    }

    pub fn get_enable_right_click(&self) -> bool {
        self.enable_right_click
    }

    pub fn get_separator(&self) -> &(String, String, String) {
        &self.separator
    }

    pub fn get_padding_adjust(&self) -> i32 {
        self.padding_adjust
    }

    pub fn get_text_align(&self) -> &String {
        &self.text_align
    }

    #[allow(unused)]
    pub fn get_enable_debug(&self) -> bool {
        self.enable_debug
    }

    pub fn configuration(&mut self, configuration: &BTreeMap<String, String>) {
        let mut timezone: LinkedHashMap<String, f64> = LinkedHashMap::new();
        let mut default_timezone: Option<String> = None;

        for (key, value) in configuration {
            match key.as_str() {
                // Option key BTreeMap is sorted
                "timezone1" | "timezone2" | "timezone3" | "timezone4" | "timezone5"
                | "timezone6" | "timezone7" | "timezone8" | "timezone9" => {
                    let value: Vec<&str> = value.split('/').collect();
                    if value.len() == 2 {
                        if let Ok(offset) = value[1].parse::<f64>() {
                            timezone.insert(value[0].trim().to_string(), offset);
                        }
                    }
                }
                "default_timezone" => {
                    default_timezone = Some(value.trim().to_string());
                }
                "date_format" => {
                    self.date_format = value.trim().to_string();
                }
                "time_format" => {
                    self.time_format = value.trim().to_string();
                }
                "background_color" => {
                    if let Ok(color) = parse_color(value) {
                        self.background_color = (color.0, color.1, color.2);
                    }
                }
                "foreground_color" => {
                    if let Ok(color) = parse_color(value) {
                        self.foreground_color = (color.0, color.1, color.2);
                    }
                }
                "pane_color" => {
                    if let Ok(color) = parse_color(value) {
                        self.pane_color = (color.0, color.1, color.2);
                    }
                }
                "enable_right_click" => {
                    self.enable_right_click = value.trim().parse().unwrap_or(false);
                }
                "arrow_separator1" => {
                    self.separator.0 = get_first_char_or_blank(value.trim());
                }
                "arrow_separator2" => {
                    self.separator.1 = get_first_char_or_blank(value.trim());
                }
                "arrow_separator3" => {
                    self.separator.2 = get_first_char_or_blank(value.trim());
                }
                "padding_adjust" => {
                    self.padding_adjust = value.trim().parse().unwrap_or(0);
                }
                "text_align" => {
                    self.text_align = value.trim().to_ascii_lowercase().to_string();
                }
                "enable_debug" => {
                    self.enable_debug = value.trim().parse().unwrap_or(false);
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

fn get_first_char_or_blank(string: &str) -> String {
    if let Some(first) = string.chars().next() {
        return first.to_string();
    }
    "".to_string()
}
