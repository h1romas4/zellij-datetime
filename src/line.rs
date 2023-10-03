use zellij_tile::prelude::*;
use zellij_tile_utils::style;

#[derive(Default)]
pub struct Line {
    background_color: PaletteColor,
    foreground_color: PaletteColor,
    pane_color: PaletteColor,
    separator: (String, String, String),
    space: String,
    padding: i32,
    text_align: TextAlign,
}

enum TextAlign {
    Right,
    Left,
    Center,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Right
    }
}

impl Line {
    pub fn update_style(
        &mut self,
        background_color: (u8, u8, u8),
        foreground_color: (u8, u8, u8),
        pane_color: (u8, u8, u8),
        separator: &(String, String, String),
        padding_adjust: i32,
        text_align: &str,
    ) {
        // set color
        self.background_color = PaletteColor::Rgb(background_color);
        self.foreground_color = PaletteColor::Rgb(foreground_color);
        self.pane_color = PaletteColor::Rgb(pane_color);
        // text align
        self.text_align = match text_align {
            "right" => TextAlign::Right,
            "left" => TextAlign::Left,
            "center" => TextAlign::Center,
            _ => TextAlign::Right,
        };
        // create charctor
        let bg_1 = self.pane_color;
        let bg_2 = self.background_color;
        let space = &style!(bg_2, bg_2).paint(" ").to_string();
        let sep_1 = &style!(bg_2, bg_1).bold().paint(&separator.0).to_string();
        let sep_2 = &style!(bg_1, bg_2).bold().paint(&separator.1).to_string();
        let sep_3 = &style!(bg_1, bg_2).bold().paint(&separator.2).to_string();
        let mut sp_0 = String::new();
        match self.text_align {
            TextAlign::Right | TextAlign::Center => {
                sp_0.push_str(sep_1);
                sp_0.push_str(space);
            }
            TextAlign::Left => {
                sp_0.push_str(space);
                sp_0.push_str(sep_1);
            }
        }
        let mut sp_1 = String::new();
        sp_1.push_str(space);
        sp_1.push_str(sep_2);
        sp_1.push_str(space);
        let mut sp_2 = String::new();
        sp_2.push_str(space);
        sp_2.push_str(sep_3);
        sp_2.push_str(space);
        self.separator = (sp_0, sp_1, sp_2);
        // last space
        self.space = space.to_string();
        // padding (Assume all as half width)
        let length = 9;
        // Getting the exact width is too much processing for a plugin,
        // so it can be adjusted by user specification.
        self.padding = length + padding_adjust;
    }

    pub fn create(&self, cols: usize, timezone: &str, date: &str, time: &str) -> String {
        // padding (partial support for full-width characters)
        let timezone_len = timezone
            .chars()
            .map(|c| if c.is_ascii() { 1 } else { 2 })
            .sum::<usize>();
        let width = ((timezone_len + date.len() + time.len()) as i32) + self.padding;
        let width = width as usize;
        // There are cases where cols may be declared momentarily low at render time.
        let padding: String = if cols as isize - width as isize > 0 {
            let size = match self.text_align {
                TextAlign::Right | TextAlign::Left => cols - width,
                // TODO: Incorrect calculation for odd-numbered characters.
                TextAlign::Center => (cols - width) / 2,
            };
            let space = " ".repeat(size);
            style!(self.foreground_color, self.pane_color)
                .paint(space)
                .to_string()
        } else {
            String::new()
        };

        let timezone = style!(self.foreground_color, self.background_color).paint(timezone);
        let date = style!(self.foreground_color, self.background_color).paint(date);
        let time = style!(self.foreground_color, self.background_color).paint(time);

        match self.text_align {
            TextAlign::Right => {
                format!(
                    "{}{}{}{}{}{}{}{}",
                    padding,
                    self.separator.0,
                    timezone,
                    self.separator.1,
                    date,
                    self.separator.2,
                    time,
                    self.space
                )
            }
            TextAlign::Left => {
                format!(
                    "{}{}{}{}{}{}{}{}",
                    self.space,
                    timezone,
                    self.separator.1,
                    date,
                    self.separator.2,
                    time,
                    self.separator.0,
                    padding,
                )
            }
            TextAlign::Center => {
                format!(
                    "{}{}{}{}{}{}{}{}",
                    padding,
                    self.separator.0,
                    timezone,
                    self.separator.1,
                    date,
                    self.separator.2,
                    time,
                    padding
                )
            }
        }
    }
}
