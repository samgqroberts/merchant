extern crate ansi_parser;
#[cfg(test)]
extern crate captured_write;
#[cfg(test)]
extern crate crossterm;
extern crate regex;

use ansi_parser::{
    AnsiParser, AnsiSequence,
    AnsiSequence::{CursorBackward, CursorDown, CursorForward, CursorPos, CursorUp},
    Output,
    Output::*,
};
use regex::Regex;
use std::convert::TryInto;
use std::str;

/// Takes a string slice and parses it into ansi escape sequences and text blocks.
///
/// The bulk of the work is done by the crate ansi_parser.
/// See: <https://docs.rs/ansi-parser/latest/ansi_parser/>
///
/// This function wraps that work in order to fix a bug where certain escape sequences
/// are mistakenly passed through as text blocks.
/// See: <https://gitlab.com/davidbittner/ansi-parser/-/issues/9>
fn tokenize_ansi(s: &str) -> Vec<Output<'_>> {
    let move_cursor_down_regex = Regex::new(r"\u{1b}\[\d+E").unwrap();
    let parsed = s.ansi_parse().flat_map(|token| {
        let mut broken_out: Vec<Output> = Vec::new();
        match token {
            Output::TextBlock(mut text) => {
                while let Some(match_) = move_cursor_down_regex.find(text) {
                    let num_to_move = text
                        .split('[')
                        .skip(1)
                        .collect::<String>()
                        .split('E')
                        .take(1)
                        .collect::<String>();
                    let num_to_move = num_to_move.parse().unwrap();
                    broken_out.push(Output::Escape(AnsiSequence::CursorDown(num_to_move)));
                    broken_out.push(Output::Escape(AnsiSequence::CursorBackward(999999999)));
                    text = &text[match_.end()..];
                }
                if !text.is_empty() {
                    broken_out.push(TextBlock(text));
                }
            }
            Escape(AnsiSequence::SetGraphicsMode(_)) => {}
            _ => {
                broken_out.push(token);
            }
        };
        broken_out
    });
    parsed.collect()
}

/// Takes a string slice containing ANSI escape sequences and returns a
/// String that looks like the terminal formatted it, but without any visual styling.
///
/// The important part of the terminal formatting that this emulates is cursor position.
/// In the resulting string, ANSI cursor position has been taken into account to produce simple
/// text in the right location.
///
/// # Examples
///
/// If an ANSI sequence directed the terminal to move the cursor right 4 characters and
/// then write the text "Hello", the resulting string from this function would be `"    Hello"`.
///
/// ```rust
/// # use raw_format_ansi::raw_format_ansi;
/// let input = "\u{1b}[4CHello";
/// let result = raw_format_ansi(input);
/// assert_eq!(result, "    Hello".to_owned());
/// ```
///
/// If the input string continued on to have a sequence directing the cursor move to line 3
/// and column 2 then write "World", the resulting string would be `"    Hello\n\n World"`
/// ```rust
/// # use raw_format_ansi::raw_format_ansi;
/// let input = "\u{1b}[4CHello\u{1b}[3;2HWorld";
/// let result = raw_format_ansi(input);
/// assert_eq!(result, "    Hello\n\n World".to_owned());
/// ```
///
/// If the input string continued on to move the cursor back to line 1 column 5 and write "J",
/// the resulting string would be:
/// ```rust
/// # use raw_format_ansi::raw_format_ansi;
/// let input = "\u{1b}[4CHello\u{1b}[3;2HWorld\u{1b}[1;5HJ";
/// let result = raw_format_ansi(input);
/// assert_eq!(result, "    Jello\n\n World".to_owned());
/// ```
pub fn raw_format_ansi(s: &str) -> String {
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut cursor_pos: (usize, usize) = (0, 0);
    let tokens = tokenize_ansi(s);
    for token in tokens {
        if let Escape(sequence) = token {
            if let CursorPos(row, col) = sequence {
                let row: usize = row.try_into().unwrap();
                let col: usize = col.try_into().unwrap();
                cursor_pos = (
                    if row > 0 { row - 1 } else { 0 },
                    if col > 0 { col - 1 } else { 0 },
                ) // ANSI escapes use 1-indexing
            } else if let CursorUp(num_lines) = sequence {
                let num_lines: usize = num_lines.try_into().unwrap();
                cursor_pos.0 = cursor_pos.0.saturating_sub(num_lines);
            } else if let CursorDown(num_lines) = sequence {
                let num_lines: usize = num_lines.try_into().unwrap();
                cursor_pos.0 += num_lines;
            } else if let CursorBackward(num_cols) = sequence {
                let num_cols: usize = num_cols.try_into().unwrap();
                cursor_pos.1 = cursor_pos.1.saturating_sub(num_cols)
            } else if let CursorForward(num_cols) = sequence {
                let num_cols: usize = num_cols.try_into().unwrap();
                cursor_pos.1 = cursor_pos.1.checked_add(num_cols).unwrap_or(0)
            }
        } else if let TextBlock(text) = token {
            // incorporate text at position
            if text.is_empty() {
                continue;
            }
            let (row, col) = &cursor_pos;
            let row = *row;
            let col = *col;
            while lines.len() < row + 1 {
                // pad with empty lines
                lines.push(vec![]);
            }
            let line: &mut Vec<char> = lines.get_mut(row).unwrap();
            while line.len() < col {
                // pad with empty spaces
                line.push(' ');
            }
            // append actual text
            let mut index = col;
            for text_char in text.chars() {
                if line.len() > index {
                    let _ = std::mem::replace(&mut line[index], text_char);
                } else {
                    line.push(text_char);
                }
                index += 1;
                cursor_pos.1 += 1;
            }
        } else {
            continue;
        }
    }
    lines
        .into_iter()
        .map(|l| l.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use ansi_parser::AnsiSequence::{CursorBackward, CursorDown, CursorPos};
    use captured_write::CapturedWrite;
    use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveToNextLine, MoveUp};
    use crossterm::execute;
    use crossterm::style::{style, Attribute, Color, Print, PrintStyledContent, Stylize};
    use crossterm::terminal::Clear;
    use std::io;

    #[test]
    fn test_tokenize_ansi() -> io::Result<()> {
        let mut fake = CapturedWrite::new();
        execute!(
            fake,
            MoveTo(1, 5),
            Print("hi"),
            MoveToNextLine(2),
            Print("bye")
        )?;
        let tokens = tokenize_ansi(&fake.buffer);
        assert_eq!(
            tokens,
            vec![
                Escape(CursorPos(6, 2)),
                TextBlock("hi"),
                Escape(CursorDown(2)),
                Escape(CursorBackward(999999999)),
                TextBlock("bye")
            ]
        );
        Ok(())
    }

    #[test]
    fn basic() -> io::Result<()> {
        let mut fake = CapturedWrite::new();
        execute!(
            fake,
            Clear(crossterm::terminal::ClearType::All),
            PrintStyledContent(
                "First line."
                    .with(Color::Green)
                    .on(Color::Red)
                    .attribute(Attribute::Bold)
            ),
            MoveTo(5, 1),
            PrintStyledContent(
                "Indented second line."
                    .with(Color::Blue)
                    .on(Color::White)
                    .attribute(Attribute::NoBold)
            ),
            MoveToNextLine(2),
            PrintStyledContent("Third line after blank line.".with(Color::White)),
        )?;
        let stripped = raw_format_ansi(&fake.buffer);
        assert_eq!(
            stripped,
            "First line.\n     Indented second line.\n\nThird line after blank line.".to_owned()
        );
        Ok(())
    }

    #[test]
    fn skipping_line_with_move_to() -> io::Result<()> {
        let mut fake = CapturedWrite::new();
        execute!(
            fake,
            Clear(crossterm::terminal::ClearType::All),
            MoveTo(0, 0),
            PrintStyledContent("First line.".with(Color::Green)),
            MoveTo(0, 2),
            PrintStyledContent("Skipped line.".with(Color::Blue)),
        )?;
        let stripped = raw_format_ansi(&fake.buffer);
        assert_eq!(stripped, "First line.\n\nSkipped line.".to_owned());
        Ok(())
    }

    #[test]
    fn relative_moves() -> io::Result<()> {
        let mut fake = CapturedWrite::new();
        execute!(
            fake,
            Print("1"),
            MoveTo(3, 3),
            Print("2"),
            MoveRight(1),
            Print("3"),
            MoveDown(2),
            Print("4"),
            MoveLeft(3),
            Print("5"),
            MoveUp(4),
            Print("6"),
        )?;
        let stripped = raw_format_ansi(&fake.buffer);
        assert_eq!(
            stripped,
            r#"1
     6

   2 3

    5 4"#
        );
        Ok(())
    }

    #[test]
    fn text_with_underline() -> io::Result<()> {
        let mut fake = CapturedWrite::new();
        execute!(
            fake,
            Print("1"),
            Print(style("2").attribute(Attribute::Underlined)),
            Print("3"),
        )?;
        let stripped = raw_format_ansi(&fake.buffer);
        assert_eq!(stripped, "123");
        Ok(())
    }

    #[test]
    fn unicode() {
        let mut writer = CapturedWrite::new();
        execute!(
            writer,
            Print("╔╗╚╝"),
            Print('┼'),
            MoveLeft(2),
            Print('┉'),
            MoveRight(2),
            Print('╿')
        )
        .unwrap();
        assert_eq!(raw_format_ansi(&writer.buffer), "╔╗╚┉┼ ╿");
    }
}
