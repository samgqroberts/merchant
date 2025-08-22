use crate::Printable;

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
