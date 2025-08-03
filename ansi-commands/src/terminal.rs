use crate::{frame::Cmd, Component};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ClearType {
    /// All cells.
    All,
    /// All plus history
    Purge,
    /// All cells from the cursor position downwards.
    FromCursorDown,
    /// All cells from the cursor position upwards.
    FromCursorUp,
    /// All cells at the cursor row.
    CurrentLine,
    /// All cells from the cursor position until the new line.
    UntilNewLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clear(pub ClearType);

impl Component for Clear {
    fn render(&self, frame: &mut crate::frame::Frame) -> Result<(), String> {
        match self.0 {
            ClearType::All => {
                frame.commands.push(Cmd::ClearScreen);
            }
            ClearType::Purge => todo!(),
            ClearType::FromCursorDown => todo!(),
            ClearType::FromCursorUp => todo!(),
            ClearType::CurrentLine => todo!(),
            ClearType::UntilNewLine => todo!(),
        }
        Ok(())
    }
}
