use ansi_commands::{
    comp,
    cursor::{Hide, MoveTo},
    style::Print,
    terminal::Clear,
    Component,
};

use crate::components::{FRAME_HEIGHT, FRAME_WIDTH};

pub struct RequireResize {
    pub current_x_cols: u16,
    pub current_y_cols: u16,
}

impl Component for RequireResize {
    fn render(&self, f: &mut ansi_commands::frame::Frame) -> Result<(), String> {
        let RequireResize {
            current_x_cols,
            current_y_cols,
        } = self;
        let more_x_needed = FRAME_WIDTH as i32 - *current_x_cols as i32;
        let more_y_needed = FRAME_HEIGHT as i32 - *current_y_cols as i32;
        let msg = if more_x_needed > 0 && more_y_needed > 0 {
            format!("Please resize terminal to be at least {FRAME_WIDTH} columns wide (currently {current_x_cols}) and {FRAME_HEIGHT} columns tall (currently {current_y_cols}).")
        } else if more_x_needed > 0 {
            format!("Please resize terminal to be at least {FRAME_WIDTH} columns wide (currently {current_x_cols}).")
        } else {
            format!("Please resize terminal to be at least {FRAME_HEIGHT} columns tall (currently {current_y_cols}).")
        };
        comp!(
            f,
            Clear(ansi_commands::terminal::ClearType::All),
            MoveTo(0, 0),
            Print(msg),
            Hide
        )?;
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test::render_component;

    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            render_component(RequireResize {
                current_x_cols: 5,
                current_y_cols: 5
            }),
            "Please resize terminal to be at least 99 columns wide (currently 5) and 32 columns tall (currently 5)."
        );
    }
}
