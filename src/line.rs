use zellij_tile::prelude::*;
use zellij_tile_utils::style;

static ARROW_SEPARATOR_1: &str = "";
static ARROW_SEPARATOR_2: &str = "";
static ARROW_SPACE: &str = " ";

#[derive(Default)]
pub struct Line {
    fg_color: PaletteColor,
    bg_color: PaletteColor,
    datetime_bg_color: PaletteColor,
    separator: (String, String, String),
}

impl Line {
    pub fn update_style(&mut self, style: Style, datetime_bg_color: (u8, u8, u8)) {
        // pallet
        self.fg_color = style.colors.fg;
        self.bg_color = style.colors.bg;
        self.datetime_bg_color = PaletteColor::Rgb(datetime_bg_color);
        // create charctor
        let bg_1 = self.bg_color;
        let bg_2 = self.datetime_bg_color;
        let arrow = style!(bg_2, bg_2).bold().paint(ARROW_SPACE);
        let sep_1 = style!(bg_2, bg_1).bold().paint(ARROW_SEPARATOR_1);
        let sep_2 = style!(bg_1, bg_2).bold().paint(ARROW_SEPARATOR_2);
        let mut sp_0 = String::new();
        sp_0.push_str(&sep_1);
        sp_0.push_str(&arrow);
        let mut sp_1 = String::new();
        sp_1.push_str(&arrow);
        sp_1.push_str(&sep_2);
        sp_1.push_str(&arrow);
        let mut sp_2 = String::new();
        sp_2.push_str(&arrow);
        self.separator = (sp_0, sp_1, sp_2);
    }

    pub fn create(&self, cols: usize, timezone: &str, date: &str, time: &str) -> String {
        // padding (support full width)
        let timezone_len = timezone
            .chars()
            .map(|c| if c.is_ascii() { 1 } else { 2 })
            .sum::<usize>();
        let width = timezone_len + date.len() + time.len() + 9;
        // There are cases where cols may be declared momentarily low at render time.
        let padding: String = if cols as isize - width as isize > 0 {
            let space = ARROW_SPACE.repeat(cols - width);
            style!(self.fg_color, self.bg_color)
                .paint(space)
                .to_string()
        } else {
            String::new()
        };

        let timezone = style!(self.fg_color, self.datetime_bg_color).paint(timezone);
        let date = style!(self.fg_color, self.datetime_bg_color).paint(date);
        let time = style!(self.fg_color, self.datetime_bg_color).paint(time);

        format!(
            "{}{}{}{}{}{}{}{}",
            padding,
            self.separator.0,
            timezone,
            self.separator.1,
            date,
            self.separator.1,
            time,
            self.separator.2
        )
    }
}
