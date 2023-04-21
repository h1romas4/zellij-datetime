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
}
register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[EventType::Timer, EventType::Visible]);
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
                }
                self.mode_info = mode_info;
            }
            _ => {
                self.render = false;
            }
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

        // TODO: respect theme pallet
        let pallet = self.mode_info.style.colors;
        let fg1 = PaletteColor::Rgb((128, 128, 128));
        let fg2 = PaletteColor::Rgb((255, 255, 255));
        let bg1 = PaletteColor::Rgb((0, 0, 0));
        let bg2 = PaletteColor::Rgb((32, 32, 32));

        // create line string
        let mut line = String::new();
        line.push_str(&style!(bg2, bg1).bold().paint(ARROW_SEPARATOR_1).to_string());
        line.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
        line.push_str(&style!(fg1, bg2).paint(&date).to_string());
        line.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
        line.push_str(&style!(fg1, bg2).bold().paint(ARROW_SEPARATOR_2).to_string());
        line.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());
        line.push_str(&style!(fg2, bg2).bold().paint(&time).to_string());
        line.push_str(&style!(bg2, bg2).bold().paint(ARROW_SPACE).to_string());

        // padding (ANSI only)
        let padding = " ".repeat(cols - (date.len() + time.len() + 6));

        // render
        print!("{}", style!(pallet.fg, pallet.bg).paint(padding));
        print!("{}", line);
    }
}
