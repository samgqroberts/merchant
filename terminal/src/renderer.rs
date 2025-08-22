use std::io::{self, Write};

use crossterm::queue;
use terminal_commands::style::{Attribute, Attributes, Color, ContentStyle};
use terminal_commands::Printable;
use terminal_commands::{Cmd, Commands};

/// Parses a series of [Cmd]s and translates them into
/// crossterm commands, which are written to the provided writer.
/// The writer is flushed after all commands have been executed.
pub fn execute_commands(commands: &Commands, writer: &mut impl Write) -> io::Result<()> {
    for cmd in commands.iter() {
        match cmd {
            Cmd::ClearScreen => queue!(
                writer,
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
            ),
            Cmd::MoveTo(x, y) => {
                queue!(writer, crossterm::cursor::MoveTo(*x as u16, *y as u16))
            }
            Cmd::MoveUp(y) => {
                queue!(writer, crossterm::cursor::MoveUp(*y as u16))
            }
            Cmd::MoveDown(y) => {
                queue!(writer, crossterm::cursor::MoveDown(*y as u16))
            }
            Cmd::MoveLeft(x) => {
                queue!(writer, crossterm::cursor::MoveLeft(*x as u16))
            }
            Cmd::MoveRight(x) => {
                queue!(writer, crossterm::cursor::MoveRight(*x as u16))
            }
            Cmd::HideCursor => {
                queue!(writer, crossterm::cursor::Hide)
            }
            Cmd::ShowCursor => {
                queue!(writer, crossterm::cursor::Show)
            }
            Cmd::MoveToNextLine(y) => {
                queue!(writer, crossterm::cursor::MoveToNextLine(*y as u16))
            }
            Cmd::Print(printable) => match printable {
                Printable::String(x) => queue!(writer, crossterm::style::Print(x)),
                Printable::Char(x) => queue!(writer, crossterm::style::Print(x)),
                Printable::StyledContent(styled_content) => {
                    let style = convert_content_style(&styled_content.style);
                    let content = styled_content.content();
                    let styled_content = crossterm::style::StyledContent::new(style, content);
                    queue!(writer, crossterm::style::Print(styled_content))
                }
            },
        }?
    }

    writer.flush()?;

    Ok(())
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

#[cfg(test)]
mod tests {
    use terminal_commands::{
        comp,
        cursor::{MoveTo, MoveToNextLine},
        style::Print,
    };

    use super::*;

    #[test]
    fn basic() {
        let mut commands = Commands::new();
        comp!(
            commands,
            MoveTo(2, 1),
            Print("Hello,"),
            MoveToNextLine(1),
            Print("world!")
        )
        .unwrap();
        let mut writer = Vec::new();
        execute_commands(&commands, &mut writer).unwrap();
        assert_eq!(
            raw_format_ansi::raw_format_ansi(&String::from_utf8(writer).unwrap()),
            "\n  Hello,\nworld!"
        );
    }
}
