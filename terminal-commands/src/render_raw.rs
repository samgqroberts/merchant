use crate::{commands::Commands, Cmd};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawRenderOutput {
    pub result: String,
    pub cursor: (u16, u16),
    pub show_cursor: bool,
}

/// Processes a series of [Cmd]s into a basic [String] similar to if
/// these commands were executed in a terminal, and then ran through [raw_format_ansi::raw_format_ansi].
pub fn render_raw(commands: &Commands) -> RawRenderOutput {
    let mut lines: Vec<Vec<char>> = vec![vec![]];
    let mut cursor = (0usize, 0usize);
    let mut show_cursor = true;

    for cmd in commands.iter() {
        match cmd {
            Cmd::Print(printable) => {
                let text = printable.raw_text();
                if text.is_empty() {
                    continue;
                }
                let (row, col) = &cursor;
                let row = *row;
                let col = *col;
                while lines.len() < row as usize + 1 {
                    // pad with empty lines
                    lines.push(vec![]);
                }
                let line: &mut Vec<char> = lines.get_mut(row as usize).unwrap();
                while line.len() < col as usize {
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
                    cursor.1 += 1;
                }
            }
            Cmd::ClearScreen => {
                lines.clear();
                cursor = (0, 0);
            }
            Cmd::MoveTo(x, y) => {
                cursor = (*y as usize, *x as usize);
            }
            Cmd::MoveUp(y) => {
                cursor.0 -= *y as usize;
            }
            Cmd::MoveDown(y) => {
                cursor.0 += *y as usize;
            }
            Cmd::MoveLeft(x) => {
                cursor.1 -= *x as usize;
            }
            Cmd::MoveRight(x) => {
                cursor.1 += *x as usize;
            }
            Cmd::HideCursor => {
                show_cursor = false;
            }
            Cmd::ShowCursor => {
                show_cursor = true;
            }
            Cmd::MoveToNextLine(y) => {
                cursor.0 += *y as usize;
                cursor.1 = 0;
            }
        }
    }
    let result = lines
        .into_iter()
        .map(|l| l.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    RawRenderOutput {
        result,
        cursor: (cursor.0 as u16, cursor.1 as u16),
        show_cursor,
    }
}
