mod config;

use chrono::prelude::*;
use std::fs;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::config::PluginConfig;

// FIXME: DateTime backgorund color
static DATETIME_BG_COLOR: (u8, u8, u8) = (32, 32, 32);

static ARROW_SEPARATOR_1: &str = "";
static ARROW_SEPARATOR_2: &str = "";
static ARROW_SPACE: &str = " ";

static INTERVAL_TIME: f64 = 1.0;

#[derive(Default)]
struct State {
    now: Option<DateTime<FixedOffset>>,
    timezone: String,
    timezone_offset: i32,
    before_minute: u32,
    visible: bool,
    style: Style,
    style_update: bool,
    fg_color: PaletteColor,
    bg_color: PaletteColor,
    datetime_bg_color: PaletteColor,
    sp_1: String,
    sp_2: String,
    sp_3: String,
    config: PluginConfig,
}
register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // load setting from config file
        if let Ok(setting) = fs::read_to_string("/host/.zellij-datetime.kdl") {
            self.config.load_config(&setting);
        };
        // get default timezone in config file
        self.timezone = self.config.get_defalut_timezone();
        self.timezone_offset = self.config.get_timezone_offset(&self.timezone);
        // zellij plunin setting
        set_selectable(false);
        subscribe(&[EventType::Timer, EventType::Visible, EventType::ModeUpdate, EventType::Mouse]);
        self.before_minute = u32::MAX;
    }

    fn update(&mut self, event: Event) -> bool {
        let mut render: bool = false;
        match event {
            Event::Visible(true) => {
                set_timeout(0.0);
                self.visible = true;
            }
            Event::Visible(false) => {
                self.visible = false;
            }
            Event::Timer(_t) => {
                // get current time with timezone
                let now = now(self.timezone_offset);
                // render at 1 minute intervals.
                let now_minute = now.minute();
                if self.before_minute != now_minute {
                    render = true;
                    self.before_minute = now_minute;
                    self.now = Some(now);
                }
                if self.visible {
                    set_timeout(INTERVAL_TIME);
                }
            }
            Event::ModeUpdate(mode_info) => {
                if self.style != mode_info.style {
                    self.style_update = true;
                    self.style = mode_info.style;
                }
            },
            Event::Mouse(mouse) => {
                match mouse {
                    Mouse::LeftClick(_size, _align) => {
                        self.change_timezone_next();
                        render = true;
                    },
                    Mouse::RightClick(_, _) => {},
                    Mouse::ScrollUp(_) => {
                        self.change_timezone_prev();
                        render = true;
                    },
                    Mouse::ScrollDown(_) => {
                        self.change_timezone_next();
                        render = true;
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        // should render
        render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        // initialize cursol charctors
        if self.style_update {
            // pallet
            self.fg_color = self.style.colors.fg;
            self.bg_color = self.style.colors.bg;
            self.datetime_bg_color = PaletteColor::Rgb(DATETIME_BG_COLOR);
            let bg1 = self.bg_color;
            let bg2 = self.datetime_bg_color;
            // create charctor
            let arrow = &style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string();
            let sep_1 = &style!(bg2, bg1).bold().paint(ARROW_SEPARATOR_1).to_string();
            let sep_2 = &style!(bg1, bg2).bold().paint(ARROW_SEPARATOR_2).to_string();
            self.sp_1 = String::new();
            self.sp_1.push_str(sep_1);
            self.sp_1.push_str(arrow);
            self.sp_2 = String::new();
            self.sp_2.push_str(arrow);
            self.sp_2.push_str(sep_2);
            self.sp_2.push_str(arrow);
            self.sp_3 = String::new();
            self.sp_3.push_str(arrow);
            self.style_update = false;
        }

        if let Some(now) = self.now {
            // timezone
            let timezone = self.timezone.to_string();
            // date
            let date = format!(
                "{year}-{month:02}-{day:02} {weekday}",
                year = now.year(),
                month = now.month(),
                day = now.day(),
                weekday = now.weekday(),
            );
            // time
            let time = format!(
                "{hour:02}:{minute:02}",
                hour = now.hour(),
                minute = now.minute(),
            );

            // padding
            let width = timezone.len() + date.len() + time.len() + 9;
            // There are cases where cols may be declared momentarily low at render time.
            let padding: String = if cols as isize - width as isize > 0 {
                // only half width char
                let space = ARROW_SPACE.repeat(cols - width);
                style!(self.fg_color, self.bg_color)
                    .paint(space)
                    .to_string()
            } else {
                String::new()
            };

            // render
            let timezone = style!(self.fg_color, self.datetime_bg_color)
                .paint(&timezone)
                .to_string();
            let date = style!(self.fg_color, self.datetime_bg_color)
                .paint(&date)
                .to_string();
            let time = style!(self.fg_color, self.datetime_bg_color)
                .paint(&time)
                .to_string();

            print!(
                "{}{}{}{}{}{}{}{}",
                padding, self.sp_1, timezone, self.sp_2, date, self.sp_2, time, self.sp_3
            );
        }
    }
}

impl State {
    fn change_timezone(&mut self, timezone: String) {
        self.timezone = timezone;
        self.timezone_offset = self.config.get_timezone_offset(&self.timezone);
        self.now = Some(now(self.timezone_offset));
    }

    fn change_timezone_next(&mut self) {
        self.change_timezone(self.config.get_timezone_next(&self.timezone));
    }

    fn change_timezone_prev(&mut self) {
        self.change_timezone(self.config.get_timezone_prev(&self.timezone));
    }
}

fn now(timezone_offset: i32) -> DateTime<FixedOffset> {
    // Timezone may not be obtained by WASI.
    // let now = Local::now();
    Utc::now().with_timezone(&FixedOffset::east(timezone_offset * 3600))
}
