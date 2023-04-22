use chrono::prelude::*;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

static ARROW_SEPARATOR_1: &str = "";
static ARROW_SEPARATOR_2: &str = "";
static ARROW_SPACE: &str = " ";

static INTERVAL_TIME: f64 = 1.0;

#[derive(Default)]
struct State {
    render: bool,
    mode_info: ModeInfo,
    mode_update: bool,
    init: bool,
    pallet_fg: PaletteColor,
    pallet_bg: PaletteColor,
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
        self.init = false;
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Visible(true) => {
                set_timeout(INTERVAL_TIME);
                self.render = true;
            }
            Event::Visible(false) => {
                self.render = false;
            }
            Event::Timer(_t) => {
                if self.render {
                    set_timeout(INTERVAL_TIME);
                }
            }
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    self.render = true;
                    self.mode_update = true;
                }
                self.mode_info = mode_info;
            }
            _ => {}
        }

        self.render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        // TODO: suport timezone or add plugin setting
        // Timezone may not be obtained by WASI.
        // let now = Local::now();
        let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));

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
            // "{hour:02}:{minute:02}:{sec:02}",
            "{hour:02}:{minute:02}",
            hour = now.hour(),
            minute = now.minute(),
            // sec = now.second(),
        );

        // pallet
        let bg2 = PaletteColor::Rgb((32, 32, 32));

        // initialize cursol charctors
        if self.mode_update {
            if !self.init {
                // pallet
                self.pallet_fg = self.mode_info.style.colors.fg;
                self.pallet_bg = self.mode_info.style.colors.bg;
                // create line string
                let bg1 = self.pallet_bg;
                self.lp_1 = String::new();
                self.lp_1.push_str(&style!(bg2, bg1).bold().paint(ARROW_SEPARATOR_1).to_string());
                self.lp_1.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
                self.lp_2 = String::new();
                self.lp_2.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
                self.lp_2.push_str(&style!(bg1, bg2).bold().paint(ARROW_SEPARATOR_2).to_string());
                self.lp_2.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
                self.lp_3 = String::new();
                self.lp_3.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
                self.init = true;
            }
            self.mode_update = false;
        }

        // padding
        let width = date.len() + time.len() + 6;
        // There are cases where cols may be declared momentarily low at render time.
        if cols as isize - width as isize > 0 {
            // padding (ANSI only)
            let padding = " ".repeat(cols - width);
            self.padding = format!("{}", style!(self.pallet_fg, self.pallet_bg).paint(padding));
        } else {
            self.padding = String::new();
        }

        // render
        let date = style!(self.pallet_fg, bg2).paint(&date).to_string();
        let time = style!(self.pallet_fg, bg2).paint(&time).to_string();
        print!("{}{}{}{}{}{}", self.padding, self.lp_1, date, self.lp_2, time, self.lp_3);
    }
}
