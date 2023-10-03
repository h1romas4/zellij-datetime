mod config;
mod line;

use chrono::prelude::*;
use std::collections::BTreeMap;
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
    line: Line,
    config: Config,
    permission_granted: bool,
}
register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        // configuration
        self.config.configuration(&configuration);
        self.reset_default_timezone();
        self.line.update_style(
            self.config.get_background_color(),
            self.config.get_foreground_color(),
            self.config.get_pane_color(),
            self.config.get_separator(),
            self.config.get_padding_adjust(),
            self.config.get_text_align(),
        );
        // initialize state
        self.before_minute = u32::MAX;
        // subscribe
        subscribe(&[
            EventType::PermissionRequestResult,
            EventType::Timer,
            EventType::Visible,
            EventType::Mouse,
        ]);
        // permission
        self.permission_granted = false;
        let mut permission = vec![];
        if self.config.get_enable_right_click() {
            permission.push(PermissionType::WriteToStdin);
        }
        if !permission.is_empty() {
            request_permission(&permission);
        } else {
            // Unselectable if no permission query
            set_selectable(false);
        }
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render: bool = false;
        match event {
            Event::PermissionRequestResult(result) => {
                if result == PermissionStatus::Granted {
                    self.permission_granted = true;
                }
                // TODO:
                // The default time zone disappears only at the first interactive query of permissions.
                // I'm not sure what the cause is. Currently being addressed by re-setting.
                // If authorization is processed by the permission cache,
                // the default time zone will be set normally without the next line.
                self.reset_default_timezone();
                // Use focus until permission authentication.
                set_selectable(false);
            }
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
                    should_render = true;
                }
                if self.visible {
                    set_timeout(INTERVAL_TIME);
                }
            }
            Event::Mouse(mouse) => match mouse {
                Mouse::LeftClick(_size, _align) => {
                    self.change_timezone_next();
                    should_render = true;
                }
                Mouse::RightClick(_, _) => {
                    // write characters to the STDIN of the focused pane
                    if self.config.get_enable_right_click() && self.permission_granted {
                        self.write_now();
                    }
                }
                Mouse::ScrollUp(_) => {
                    self.change_timezone_prev();
                    should_render = true;
                }
                Mouse::ScrollDown(_) => {
                    self.change_timezone_next();
                    should_render = true;
                }
                _ => {}
            },
            _ => {}
        }
        should_render
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
    fn reset_default_timezone(&mut self) {
        self.timezone = self.config.get_default_timezone();
        self.timezone_offset = self.config.get_timezone_offset(&self.timezone);
    }

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
