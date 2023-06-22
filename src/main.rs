mod config;
mod line;

use chrono::prelude::*;
use std::fs;
use zellij_tile::prelude::*;

use crate::config::Config;
use crate::line::Line;

// FIXME: DateTime backgorund color
static DATETIME_BG_COLOR: (u8, u8, u8) = (32, 32, 32);

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
    line: Line,
    config: Config,
}
register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // load setting from config file
        if let Ok(setting) = fs::read_to_string("/host/.zellij-datetime.kdl") {
            self.config.load_config(&setting);
        };
        // get default timezone in config
        self.timezone = self.config.get_defalut_timezone();
        self.timezone_offset = self.config.get_timezone_offset(&self.timezone);
        // zellij plunin setting
        set_selectable(false);
        subscribe(&[
            EventType::Timer,
            EventType::Visible,
            EventType::ModeUpdate,
            EventType::Mouse,
        ]);
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
            }
            Event::Mouse(mouse) => match mouse {
                Mouse::LeftClick(_size, _align) => {
                    self.change_timezone_next();
                    render = true;
                }
                Mouse::RightClick(_, _) => {}
                Mouse::ScrollUp(_) => {
                    self.change_timezone_prev();
                    render = true;
                }
                Mouse::ScrollDown(_) => {
                    self.change_timezone_next();
                    render = true;
                }
                _ => {}
            },
            _ => {}
        }
        // should render
        render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        // update line style
        if self.style_update {
            self.line.update_style(self.style, DATETIME_BG_COLOR);
        }

        if let Some(now) = self.now {
            // format date
            let date = format!(
                "{year}-{month:02}-{day:02} {weekday}",
                year = now.year(),
                month = now.month(),
                day = now.day(),
                weekday = now.weekday(),
            );
            // format time
            let time = format!(
                "{hour:02}:{minute:02}",
                hour = now.hour(),
                minute = now.minute(),
            );
            // render
            print!("{}", self.line.render(cols, &self.timezone, &date, &time));
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
