use crate::frame::Frame;

pub mod cursor;
pub mod style;
pub mod terminal;
#[macro_use]
pub mod macros;
pub mod event;
pub mod frame;

pub trait Component {
    fn render(&self, frame: &mut Frame) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use crate::{
        cursor::MoveTo,
        style::{Attribute, Print, Stylize},
    };

    use super::*;

    #[test]
    fn test_frame() -> Result<(), String> {
        let mut frame = Frame::new();
        comp!(frame, MoveTo(1, 1), Print("Hello, world!"))?;
        assert_eq!(frame.render_raw().result, "\n Hello, world!");
        Ok(())
    }

    #[test]
    fn test_underline() -> Result<(), String> {
        let mut frame = Frame::new();
        comp!(
            frame,
            MoveTo(1, 1),
            Print("H".attribute(Attribute::Underlined)),
            Print("ello, world!")
        )?;
        assert_eq!(frame.render_raw().result, "\n Hello, world!");
        Ok(())
    }

    // #[cfg(feature = "crossterm")]
    // #[test]
    // fn test_frame_crossterm() -> Result<(), String> {
    //     use crate::style::Stylize;
    //     use raw_format_ansi::raw_format_ansi;

    //     use crate::style::Attribute;

    //     let mut frame = Frame::new();
    //     comp!(
    //         frame,
    //         MoveTo(1, 1),
    //         Print("Hello, world!".attribute(Attribute::Bold)),
    //     )?;

    //     let mut captured_write = captured_write::CapturedWrite::new();
    //     frame.render_crossterm(&mut captured_write).unwrap();
    //     let buffer = captured_write.buffer;
    //     let formatted = raw_format_ansi(&buffer);
    //     assert_eq!(formatted, "\n Hello, world!");
    //     Ok(())
    // }
}
