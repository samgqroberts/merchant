use ansi_commands::frame::{Frame, RenderResult, Renderer};
use ansi_commands::style::Color;

pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(&self, frame: &Frame) -> RenderResult {
        let mut html = String::new();
        let mut current_line = String::new();
        let mut cursor_x = 0;
        let mut cursor_y = 0;
        let mut show_cursor = false;

        for cmd in frame.commands().iter() {
            match cmd {
                ansi_commands::frame::Cmd::ClearScreen => {
                    html.clear();
                    current_line.clear();
                    cursor_x = 0;
                    cursor_y = 0;
                }
                ansi_commands::frame::Cmd::MoveTo(x, y) => {
                    while cursor_y < *y {
                        html.push_str(&current_line);
                        html.push('\n');
                        current_line.clear();
                        cursor_y += 1;
                        cursor_x = 0;
                    }
                    while cursor_x < *x {
                        current_line.push(' ');
                        cursor_x += 1;
                    }
                }
                ansi_commands::frame::Cmd::MoveUp(y) => {
                    cursor_y = cursor_y.saturating_sub(*y);
                }
                ansi_commands::frame::Cmd::MoveDown(y) => {
                    cursor_y += y;
                }
                ansi_commands::frame::Cmd::MoveLeft(x) => {
                    cursor_x = cursor_x.saturating_sub(*x);
                }
                ansi_commands::frame::Cmd::MoveRight(x) => {
                    cursor_x += x;
                }
                ansi_commands::frame::Cmd::HideCursor => {
                    show_cursor = false;
                }
                ansi_commands::frame::Cmd::ShowCursor => {
                    show_cursor = true;
                }
                ansi_commands::frame::Cmd::MoveToNextLine(y) => {
                    for _ in 0..*y {
                        html.push_str(&current_line);
                        html.push('\n');
                        current_line.clear();
                        cursor_y += 1;
                    }
                    cursor_x = 0;
                }
                ansi_commands::frame::Cmd::Print(printable) => {
                    let text = printable.raw_text();
                    current_line.push_str(&text);
                    cursor_x += text.len() as u16;
                }
            }
        }

        // Flush remaining line
        if !current_line.is_empty() {
            html.push_str(&current_line);
        }

        RenderResult {
            result: html,
            cursor: (cursor_x, cursor_y),
            show_cursor,
        }
    }
}

fn color_to_css(color: Color) -> String {
    match color {
        Color::Reset => "inherit".to_string(),
        Color::Black => "black".to_string(),
        Color::DarkGrey => "#555".to_string(),
        Color::Red => "red".to_string(),
        Color::DarkRed => "#800".to_string(),
        Color::Green => "lime".to_string(),
        Color::DarkGreen => "green".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::DarkYellow => "#880".to_string(),
        Color::Blue => "#00f".to_string(),
        Color::DarkBlue => "#008".to_string(),
        Color::Magenta => "magenta".to_string(),
        Color::DarkMagenta => "#808".to_string(),
        Color::Cyan => "cyan".to_string(),
        Color::DarkCyan => "#088".to_string(),
        Color::White => "white".to_string(),
        Color::Grey => "#aaa".to_string(),
        Color::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
        Color::AnsiValue(value) => format!("color-{}", value),
    }
}
