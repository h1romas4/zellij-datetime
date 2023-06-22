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
    sp_1: String,
    sp_2: String,
    sp_3: String,
}

impl Line {
    pub fn update_style(&mut self, style: Style, datetime_bg_color: (u8, u8, u8)) {
        // pallet
        self.fg_color = style.colors.fg;
        self.bg_color = style.colors.bg;
        self.datetime_bg_color = PaletteColor::Rgb(datetime_bg_color);
        // create charctor
        let bg1 = self.bg_color;
        let bg2 = self.datetime_bg_color;
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
    }

    pub fn render(&self, cols: usize, timezone: &str, date: &str, time: &str) -> String {
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

        // create string
        let timezone = style!(self.fg_color, self.datetime_bg_color).paint(timezone);
        let date = style!(self.fg_color, self.datetime_bg_color).paint(date);
        let time = style!(self.fg_color, self.datetime_bg_color).paint(time);

        format!(
            "{}{}{}{}{}{}{}{}",
            padding, self.sp_1, timezone, self.sp_2, date, self.sp_2, time, self.sp_3
        )
    }
}
