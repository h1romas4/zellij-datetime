mod config;
mod line;

use chrono::prelude::*;
use std::fs;
use zellij_tile::prelude::*;

use crate::config::Config;
use crate::line::Line;

static INTERVAL_TIME: f64 = 1.0;

#[derive(Default)]
struct State {
    now: Option<DateTime<Utc>>,
    timezone: String,
    timezone_offset: i32,
    before_minute: u32,
    visible: bool,
    style: Style,
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
        // for making minute comparisons
        self.before_minute = u32::MAX;
        // zellij plunin setting
        set_selectable(false);
        subscribe(&[
            EventType::Timer,
            EventType::Visible,
            EventType::ModeUpdate,
            EventType::Mouse,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut render: bool = false;
        match event {
            Event::Visible(visible) => {
                // TODO:
                // If the Zellij session is detached, it is called with false,
                // but if it is reattached, this event is not fired.
                // Working on a way to restart the timer when it is reattached.
                if visible {
                    set_timeout(0.0);
                }
                self.visible = visible;
            }
            Event::Timer(_t) => {
                // get current time without timezone
                let now = now();
                // render at 1 minute intervals
                let now_minute = now.minute();
                if self.before_minute != now_minute {
                    self.before_minute = now_minute;
                    self.now = Some(now);
                    render = true;
                }
                if self.visible {
                    set_timeout(INTERVAL_TIME);
                }
            }
            Event::ModeUpdate(mode_info) => {
                if self.style != mode_info.style {
                    self.style = mode_info.style;
                    self.line
                        .update_style(self.style, self.config.get_backgound_color());
                }
            }
            Event::Mouse(mouse) => match mouse {
                Mouse::LeftClick(_size, _align) => {
                    self.change_timezone_next();
                    render = true;
                }
                Mouse::RightClick(_, _) => {
                    // write characters to the STDIN of the focused pane
                    self.write_now();
                }
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
        render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        if let Some(now) = self.now() {
            let date = format!(
                "{year}-{month:02}-{day:02} {weekday}",
                year = now.year(),
                month = now.month(),
                day = now.day(),
                weekday = now.weekday(),
            );
            let time = format!(
                "{hour:02}:{minute:02}",
                hour = now.hour(),
                minute = now.minute(),
            );
            print!("{}", self.line.create(cols, &self.timezone, &date, &time));
        }
    }
}

impl State {
    fn change_timezone(&mut self, timezone: String) {
        self.timezone = timezone;
        self.timezone_offset = self.config.get_timezone_offset(&self.timezone);
    }

    fn change_timezone_next(&mut self) {
        self.change_timezone(self.config.get_next_timezone(&self.timezone));
    }

    fn change_timezone_prev(&mut self) {
        self.change_timezone(self.config.get_prev_timezone(&self.timezone));
    }

    fn now(&self) -> Option<DateTime<FixedOffset>> {
        self.now
            .map(|now| now.with_timezone(&FixedOffset::east(&self.timezone_offset * 3600)))
    }

    fn write_now(&self) {
        if let Some(now) = self.now() {
            let datetime = format!(
                "{year}{month:02}{day:02}_{hour:02}{minute:02}",
                year = now.year(),
                month = now.month(),
                day = now.day(),
                hour = now.hour(),
                minute = now.minute(),
            );
            write(datetime.into_bytes());
        }
    }
}

fn now() -> DateTime<Utc> {
    // Timezone may not be obtained by WASI.
    // let now = Local::now();
    Utc::now()
}
