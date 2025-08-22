pub mod cursor;
pub mod style;
pub mod terminal;
#[macro_use]
mod macros;
mod cmd;
mod commands;
pub mod event;
mod printable;
mod render_raw;

pub use cmd::Cmd;
pub use commands::Commands;
pub use printable::Printable;
pub use render_raw::{render_raw, RawRenderOutput};

pub trait Component {
    fn render(&self, commands: &mut Commands) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use crate::{
        cursor::MoveTo,
        render_raw::render_raw,
        style::{Attribute, Print, Stylize},
    };

    use super::*;

    #[test]
    fn test_frame() -> Result<(), String> {
        let mut commands = Commands::new();
        comp!(commands, MoveTo(1, 1), Print("Hello, world!"))?;
        assert_eq!(render_raw(&commands).result, "\n Hello, world!");
        Ok(())
    }

    #[test]
    fn test_underline() -> Result<(), String> {
        let mut commands = Commands::new();
        comp!(
            commands,
            MoveTo(1, 1),
            Print("H".attribute(Attribute::Underlined)),
            Print("ello, world!")
        )?;
        assert_eq!(render_raw(&commands).result, "\n Hello, world!");
        Ok(())
    }
}
