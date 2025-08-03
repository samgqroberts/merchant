use crate::{frame::Cmd, Component, Frame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUp(pub u16);

impl Component for MoveUp {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::MoveUp(self.0));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDown(pub u16);

impl Component for MoveDown {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::MoveDown(self.0));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveLeft(pub u16);

impl Component for MoveLeft {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::MoveLeft(self.0));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveRight(pub u16);

impl Component for MoveRight {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::MoveRight(self.0));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hide;

impl Component for Hide {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::HideCursor);
        Ok(())
    }
}

pub struct MoveToNextLine(pub u16 /* number of lines */);

impl Component for MoveToNextLine {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::MoveToNextLine(self.0));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveTo(pub u16 /* column (x) */, pub u16 /* row (y) */);

impl Component for MoveTo {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::MoveTo(self.0, self.1));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Show;

impl Component for Show {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::ShowCursor);
        Ok(())
    }
}
