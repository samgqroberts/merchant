use ansi_commands::frame::{Frame, RenderResult, Renderer};
use ansi_commands::style::{Attribute, Attributes, Color, ContentStyle, StyledContent};
use captured_write::CapturedWrite;
use crossterm::queue;

pub struct CrosstermRenderer;

impl Renderer for CrosstermRenderer {
    fn render(&self, frame: &Frame) -> RenderResult {
        let mut writer = CapturedWrite::new();

        for cmd in frame.commands().iter() {
            match cmd {
                ansi_commands::frame::Cmd::ClearScreen => {
                    queue!(
                        writer,
                        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
                    )
                    .unwrap() // todo;
                }
                ansi_commands::frame::Cmd::MoveTo(x, y) => {
                    queue!(writer, crossterm::cursor::MoveTo(*x as u16, *y as u16)).unwrap()
                    // todo;
                }
                ansi_commands::frame::Cmd::MoveUp(y) => {
                    queue!(writer, crossterm::cursor::MoveUp(*y as u16)).unwrap()
                    // todo;
                }
                ansi_commands::frame::Cmd::MoveDown(y) => {
                    queue!(writer, crossterm::cursor::MoveDown(*y as u16)).unwrap()
                    // todo;
                }
                ansi_commands::frame::Cmd::MoveLeft(x) => {
                    queue!(writer, crossterm::cursor::MoveLeft(*x as u16)).unwrap()
                    // todo;
                }
                ansi_commands::frame::Cmd::MoveRight(x) => {
                    queue!(writer, crossterm::cursor::MoveRight(*x as u16)).unwrap()
                    // todo;
                }
                ansi_commands::frame::Cmd::HideCursor => {
                    queue!(writer, crossterm::cursor::Hide).unwrap() // todo;
                }
                ansi_commands::frame::Cmd::ShowCursor => {
                    queue!(writer, crossterm::cursor::Show).unwrap() // todo;
                }
                ansi_commands::frame::Cmd::MoveToNextLine(y) => {
                    queue!(writer, crossterm::cursor::MoveToNextLine(*y as u16)).unwrap()
                    // todo;
                }
                ansi_commands::frame::Cmd::Print(printable) => {
                    queue!(writer, crossterm::style::Print(printable.raw_text())).unwrap()
                    // todo;
                }
            }
        }
        RenderResult {
            result: writer.buffer,
            cursor: (0, 0),
            show_cursor: true,
        }
    }
}

fn convert_color(color: Color) -> crossterm::style::Color {
    match color {
        Color::Reset => crossterm::style::Color::Reset,
        Color::Black => crossterm::style::Color::Black,
        Color::DarkGrey => crossterm::style::Color::DarkGrey,
        Color::Red => crossterm::style::Color::Red,
        Color::DarkRed => crossterm::style::Color::DarkRed,
        Color::Green => crossterm::style::Color::Green,
        Color::DarkGreen => crossterm::style::Color::DarkGreen,
        Color::Yellow => crossterm::style::Color::Yellow,
        Color::DarkYellow => crossterm::style::Color::DarkYellow,
        Color::Blue => crossterm::style::Color::Blue,
        Color::DarkBlue => crossterm::style::Color::DarkBlue,
        Color::Magenta => crossterm::style::Color::Magenta,
        Color::DarkMagenta => crossterm::style::Color::DarkMagenta,
        Color::Cyan => crossterm::style::Color::Cyan,
        Color::DarkCyan => crossterm::style::Color::DarkCyan,
        Color::White => crossterm::style::Color::White,
        Color::Grey => crossterm::style::Color::Grey,
        Color::Rgb { r, g, b } => crossterm::style::Color::Rgb {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        },
        Color::AnsiValue(value) => crossterm::style::Color::AnsiValue(value as u8),
    }
}

pub fn convert_attributes(attributes: &Attributes) -> crossterm::style::Attributes {
    let mut attrs = crossterm::style::Attributes::none();
    if attributes.has(Attribute::Underlined) {
        attrs.set(crossterm::style::Attribute::Underlined);
    }
    if attributes.has(Attribute::Bold) {
        attrs.set(crossterm::style::Attribute::Bold);
    }
    attrs
}

pub fn convert_content_style(content_style: &ContentStyle) -> crossterm::style::ContentStyle {
    crossterm::style::ContentStyle {
        foreground_color: content_style.foreground_color.map(|c| convert_color(c)),
        background_color: content_style.background_color.map(|c| convert_color(c)),
        underline_color: content_style.underline_color.map(|c| convert_color(c)),
        attributes: convert_attributes(&content_style.attributes),
    }
}

pub fn convert_styled_content(
    styled_content: &StyledContent<char>,
) -> crossterm::style::StyledContent<char> {
    let as_crossterm = crossterm::style::StyledContent::new(
        convert_content_style(&styled_content.style),
        styled_content.content.clone(),
    );
    as_crossterm
}

#[cfg(test)]
mod tests {
    use ansi_commands::{
        comp,
        cursor::{MoveTo, MoveToNextLine},
        style::Print,
    };

    use super::*;

    #[test]
    fn basic() {
        let mut frame = Frame::new();
        comp!(
            frame,
            MoveTo(2, 1),
            Print("Hello,"),
            MoveToNextLine(1),
            Print("world!")
        )
        .unwrap();
        let result = CrosstermRenderer.render(&frame);
        assert_eq!(
            raw_format_ansi::raw_format_ansi(&result.result),
            "\n  Hello,\nworld!"
        );
    }
}
