use chrono::prelude::*;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

// FIXME: UTC+9
static TIMEZONE_OFFSET: i32 = 9;
// Dark gray
static DATETIME_BG_COLOR: (u8, u8, u8) = (32, 32, 32);

static ARROW_SEPARATOR_1: &str = "";
static ARROW_SEPARATOR_2: &str = "";
static ARROW_SPACE: &str = " ";

static INTERVAL_TIME: f64 = 1.0;

#[derive(Default)]
struct State {
    now: Option<DateTime<FixedOffset>>,
    before_now: u32,
    visible: bool,
    mode_info: ModeInfo,
    mode_update: bool,
    pallet_fg: PaletteColor,
    pallet_bg: PaletteColor,
    datetime_bg_color: PaletteColor,
    lp_1: String,
    lp_2: String,
    lp_3: String,
    padding: String,
}
register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[
            EventType::Timer,
            EventType::Visible,
            EventType::ModeUpdate,
        ]);
        self.before_now = u32::MAX;
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
                // TODO: suport timezone or add plugin setting
                // Timezone may not be obtained by WASI.
                // let now = Local::now();
                let now = Utc::now()
                    .with_timezone(&FixedOffset::east(TIMEZONE_OFFSET * 3600));
                // render at 1 minute intervals.
                let now_minute = now.minute();
                if self.before_now != now_minute {
                    render = true;
                    self.before_now = now_minute;
                    self.now = Some(now);
                }
                if self.visible {
                    set_timeout(INTERVAL_TIME);
                }
            }
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    render = true;
                    self.mode_update = true;
                    self.mode_info = mode_info;
                }
            }
            _ => {}
        }
        // should render
        render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        // initialize cursol charctors
        if self.mode_update {
            // pallet
            self.pallet_fg = self.mode_info.style.colors.fg;
            self.pallet_bg = self.mode_info.style.colors.bg;
            self.datetime_bg_color = PaletteColor::Rgb(DATETIME_BG_COLOR);
            // create line string
            let bg1 = self.pallet_bg;
            let bg2 = self.datetime_bg_color;
            self.lp_1 = String::new();
            self.lp_1
                .push_str(&style!(bg2, bg1).bold().paint(ARROW_SEPARATOR_1).to_string());
            self.lp_1
                .push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
            self.lp_2 = String::new();
            self.lp_2
                .push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
            self.lp_2
                .push_str(&style!(bg1, bg2).bold().paint(ARROW_SEPARATOR_2).to_string());
            self.lp_2
                .push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
            self.lp_3 = String::new();
            self.lp_3
                .push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
            self.mode_update = false;
        }

        if let Some(now) = self.now {
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
            let width = date.len() + time.len() + 6;
            // There are cases where cols may be declared momentarily low at render time.
            if cols as isize - width as isize > 0 {
                // only half width char
                let padding = ARROW_SPACE.repeat(cols - width);
                self.padding = style!(self.pallet_fg, self.pallet_bg).paint(padding).to_string();
            } else {
                self.padding = String::new();
            }

            // render
            let bg2 = self.datetime_bg_color;
            let date = style!(self.pallet_fg, bg2).paint(&date).to_string();
            let time = style!(self.pallet_fg, bg2).paint(&time).to_string();

            print!(
                "{}{}{}{}{}{}",
                self.padding, self.lp_1, date, self.lp_2, time, self.lp_3
            );
        }
    }
}
