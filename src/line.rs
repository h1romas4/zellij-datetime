use zellij_tile::prelude::*;
use zellij_tile_utils::style;

static ARROW_SEPARATOR_1: &str = "";
static ARROW_SEPARATOR_2: &str = "";
static ARROW_SPACE: &str = " ";

#[derive(Default)]
pub struct Line {
    backgound_color: PaletteColor,
    foreground_color: PaletteColor,
    pane_color: PaletteColor,
    separator: (String, String, String),
}

impl Line {
    pub fn update_style(
        &mut self,
        backgound_color: (u8, u8, u8),
        foreground_color: (u8, u8, u8),
        pane_color: (u8, u8, u8),
    ) {
        // set color
        self.backgound_color = PaletteColor::Rgb(backgound_color);
        self.foreground_color = PaletteColor::Rgb(foreground_color);
        self.pane_color = PaletteColor::Rgb(pane_color);
        // create charctor
        let bg_1 = self.pane_color;
        let bg_2 = self.backgound_color;
        let arrow = &style!(bg_2, bg_2).bold().paint(ARROW_SPACE).to_string();
        let sep_1 = &style!(bg_2, bg_1)
            .bold()
            .paint(ARROW_SEPARATOR_1)
            .to_string();
        let sep_2 = &style!(bg_1, bg_2)
            .bold()
            .paint(ARROW_SEPARATOR_2)
            .to_string();
        let mut sp_0 = String::new();
        sp_0.push_str(sep_1);
        sp_0.push_str(arrow);
        let mut sp_1 = String::new();
        sp_1.push_str(arrow);
        sp_1.push_str(sep_2);
        sp_1.push_str(arrow);
        let mut sp_2 = String::new();
        sp_2.push_str(arrow);
        self.separator = (sp_0, sp_1, sp_2);
    }

    pub fn create(&self, cols: usize, timezone: &str, date: &str, time: &str) -> String {
        // padding (partial support for full-width characters)
        let timezone_len = timezone
            .chars()
            .map(|c| if c.is_ascii() { 1 } else { 2 })
            .sum::<usize>();
        let width = timezone_len + date.len() + time.len() + 9;
        // There are cases where cols may be declared momentarily low at render time.
        let padding: String = if cols as isize - width as isize > 0 {
            let space = ARROW_SPACE.repeat(cols - width);
            style!(self.foreground_color, self.pane_color)
                .paint(space)
                .to_string()
        } else {
            String::new()
        };

        let timezone = style!(self.foreground_color, self.backgound_color).paint(timezone);
        let date = style!(self.foreground_color, self.backgound_color).paint(date);
        let time = style!(self.foreground_color, self.backgound_color).paint(time);

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
