use std::fmt::{self};

use crossterm::{
    cursor::{MoveDown, MoveRight, MoveTo},
    style::Print,
    Command,
};

use crate::{
    comp,
    components::{HorizontalLine, FRAME_HEIGHT, FRAME_WIDTH},
};

pub struct Frame(pub bool);

impl Command for Frame {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        // 2 horizontal lines at top and bottom ends
        comp!(
            f,
            HorizontalLine(0, true),
            HorizontalLine(FRAME_HEIGHT, true)
        );
        if !self.0 {
            // additional thick horizontal line near location
            for i in 0..(FRAME_WIDTH) {
                comp!(f, MoveTo(i, 19), Print("="), MoveRight(1));
            }
        }
        // 2 vertical lines at left and right ends
        for i in 0..(FRAME_HEIGHT - 1) {
            comp!(f, MoveTo(0, 1 + i), Print("|"), MoveDown(1));
        }
        for i in 0..(FRAME_HEIGHT - 1) {
            comp!(f, MoveTo(FRAME_WIDTH - 1, 1 + i), Print("|"), MoveDown(1));
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use captured_write::CapturedWrite;
    use crossterm::execute;
    use pretty_assertions::assert_eq;

    use super::*;

    fn render_component<T: Command>(x: T) -> String {
        let mut writer = CapturedWrite::new();
        execute!(&mut writer, x).unwrap();
        raw_format_ansi::raw_format_ansi(&writer.buffer)
    }

    #[test]
    fn basic() {
        assert_eq!(
            render_component(Frame(true)),
            r#"
---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"#
            .trim()
        );
    }
}
