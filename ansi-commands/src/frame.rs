use crate::{style::StyledContent, Component};

#[derive(Debug, Clone)]
pub enum Printable {
    String(String),
    Char(char),
    StyledContent(StyledContent<String>),
}

impl Printable {
    pub fn raw_text(&self) -> String {
        match self {
            Printable::String(s) => s.clone(),
            Printable::Char(c) => c.to_string(),
            Printable::StyledContent(styled_content) => styled_content.content().to_string(),
        }
    }
}

impl From<&char> for Printable {
    fn from(c: &char) -> Self {
        Printable::Char(*c)
    }
}

impl From<String> for Printable {
    fn from(s: String) -> Self {
        Printable::String(s)
    }
}

impl From<char> for Printable {
    fn from(c: char) -> Self {
        Printable::Char(c)
    }
}

impl From<StyledContent<String>> for Printable {
    fn from(styled_content: StyledContent<String>) -> Self {
        Printable::StyledContent(styled_content)
    }
}

impl From<&str> for Printable {
    fn from(s: &str) -> Self {
        Printable::String(s.to_string())
    }
}

impl From<StyledContent<&str>> for Printable {
    fn from(styled_content: StyledContent<&str>) -> Self {
        Printable::StyledContent(styled_content.into())
    }
}

#[derive(Debug, Clone)]
pub enum Cmd {
    ClearScreen,
    MoveTo(u16 /* column (x) */, u16 /* row (y) */),
    MoveUp(u16),
    MoveDown(u16),
    MoveLeft(u16),
    MoveRight(u16),
    HideCursor,
    ShowCursor,
    MoveToNextLine(u16 /* number of lines */),
    Print(Printable),
}

pub struct Frame {
    pub(crate) commands: Vec<Cmd>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderResult {
    pub result: String,
    pub cursor: (u16, u16),
    pub show_cursor: bool,
}

impl Frame {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn commands(&self) -> &Vec<Cmd> {
        &self.commands
    }

    pub fn render(&mut self, component: &dyn Component) -> Result<(), String> {
        component.render(self)
    }

    pub fn render_raw(&self) -> RenderResult {
        RawRenderer.render(self)
    }
}

impl AsMut<Frame> for Frame {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub struct RawRenderer;

pub trait Renderer {
    fn render(&self, frame: &Frame) -> RenderResult;
}

impl Renderer for RawRenderer {
    fn render(&self, frame: &Frame) -> RenderResult {
        let mut lines: Vec<Vec<char>> = vec![vec![]];
        let mut cursor = (0usize, 0usize);
        let mut show_cursor = true;

        for cmd in frame.commands.iter() {
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
        RenderResult {
            result,
            cursor: (cursor.0 as u16, cursor.1 as u16),
            show_cursor,
        }
    }
}

// #[cfg(feature = "crossterm")]
// impl Frame {
//     pub fn render_crossterm(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
//         for (i, row) in self.rows.iter().enumerate() {
//             use crossterm::cursor::MoveTo;
//             use crossterm::queue;
//             use crossterm::style::Print;

//             for (j, cell) in row.iter().enumerate() {
//                 queue!(f, MoveTo(j as u16, i as u16))?;
//                 if let Some(styled_content) = cell {
//                     queue!(f, Print(styled_content))?;
//                 }
//             }
//             if self.show_cursor {
//                 queue!(f, crossterm::cursor::Show)?;
//             } else {
//                 queue!(f, crossterm::cursor::Hide)?;
//             }
//         }
//         Ok(())
//     }
// }
