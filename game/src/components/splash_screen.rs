use std::fmt::{self};

use crossterm::{
    cursor::{Hide, MoveTo},
    style::{style, Attribute, Print, Stylize},
    terminal::Clear,
    Command,
};

use crate::{
    comp,
    components::{Frame, FrameType, ScreenCenteredText},
};

pub struct SplashScreen();

const LOGO: &str = r#"
 __  __               _                 _   
|  \/  |             | |               | |  
| \  / | ___ _ __ ___| |__   __ _ _ __ | |_ 
| |\/| |/ _ \ '__/ __| '_ \ / _` | '_ \| __|
| |  | |  __/ | | (__| | | | (_| | | | | |_ 
|_|  |_|\___|_|  \___|_| |_|\__,_|_| |_|\__|
"#;

impl Command for SplashScreen {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All),
            Frame(FrameType::SimpleEmptyInside),
            ScreenCenteredText::new(&["A tribute to Drug Wars by samgqroberts".to_owned()], 12),
            ScreenCenteredText::new(&["www.samgqroberts.com".to_owned()], 14),
            ScreenCenteredText::new_styleds(
                &[style("Press any key to begin").attribute(Attribute::Bold)],
                25
            ),
            ScreenCenteredText::new(&["ctrl-c to quit at any time".to_owned()], 29),
            Hide
        );
        const OFFSET_X: u16 = 28;
        const OFFSET_Y: u16 = 4;
        for (i, line) in LOGO.trim_matches('\n').lines().enumerate() {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + (i as u16)),
                Print(line.to_string()),
            );
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
    use pretty_assertions::assert_eq;

    use crate::test::render_component;

    use super::*;

    #[test]
    fn basic() {
        println!("{}", render_component(SplashScreen()));
        assert_eq!(
            render_component(SplashScreen()),
            r#"
---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                            __  __               _                 _                             |
|                           |  \/  |             | |               | |                            |
|                           | \  / | ___ _ __ ___| |__   __ _ _ __ | |_                           |
|                           | |\/| |/ _ \ '__/ __| '_ \ / _` | '_ \| __|                          |
|                           | |  | |  __/ | | (__| | | | (_| | | | | |_                           |
|                           |_|  |_|\___|_|  \___|_| |_|\__,_|_| |_|\__|                          |
|                                                                                                 |
|                                                                                                 |
|                              A tribute to Drug Wars by samgqroberts                             |
|                                                                                                 |
|                                       www.samgqroberts.com                                      |
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
|                                      Press any key to begin                                     |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                    ctrl-c to quit at any time                                   |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"#
            .trim()
        );
    }
}
