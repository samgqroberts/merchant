use std::fmt::{self};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveTo},
    style::Print,
    Command,
};

use crate::{
    comp,
    components::{FRAME_HEIGHT, FRAME_WIDTH},
    state::Location,
};

#[derive(PartialEq)]
pub enum FrameType {
    SimpleEmptyInside,
    Location(Location),
}

struct VerticalSequence {
    pub char_sequence: Vec<char>,
    pub len: u16,
}

impl Command for VerticalSequence {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let mut iter = self.char_sequence.iter().cycle();
        for _ in 0..(self.len) {
            let Some(symbol) = iter.next() else {
                continue;
            };
            comp!(f, Print(symbol), MoveDown(1), MoveLeft(1));
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct Frame(pub FrameType);

const LOCATION_DIVIDER_Y: u16 = 19;

pub const SIMPLE_HORIZONTAL_FULL: &str = "---------------------------------------------------------------------------------------------------";

pub const LONDON_HORIZONTAL_TOP: &str = "'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.";
pub const LONDON_HORIZONTAL_BOT: &str = ".~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'";
pub const LONDON_HORIZONTAL_MID: &str = "'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'";

pub const SAVANNAH_HORIZONTAL_TOP: &str = "┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴";
pub const SAVANNAH_HORIZONTAL_BOT: &str = "┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬";
pub const SAVANNAH_HORIZONTAL_MID: &str = "─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─";

pub const LISBON_HORIZONTAL_TOP: &str = "▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚";
pub const LISBON_HORIZONTAL_BOT: &str = "▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞";
pub const LISBON_HORIZONTAL_MID: &str = "▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄";

pub const AMSTERDAM_HORIZONTAL_TOP: &str = "▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷";
pub const AMSTERDAM_HORIZONTAL_BOT: &str = "◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁";
pub const AMSTERDAM_HORIZONTAL_MID: &str = "◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊";

pub const CAPETOWN_HORIZONTAL_TOP: &str = "◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖";
pub const CAPETOWN_HORIZONTAL_BOT: &str = "◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●";
pub const CAPETOWN_HORIZONTAL_MID: &str = "◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐";

pub const VENICE_HORIZONTAL_TOP: &str = "╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲";
pub const VENICE_HORIZONTAL_BOT: &str = "╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱";
pub const VENICE_HORIZONTAL_MID: &str = "╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲";

impl Command for Frame {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let (top, bot, mid, left_char_seq, right_char_seq) = match self.0 {
            FrameType::SimpleEmptyInside => (
                SIMPLE_HORIZONTAL_FULL,
                SIMPLE_HORIZONTAL_FULL,
                None,
                vec!['|'],
                vec!['|'],
            ),
            FrameType::Location(location) => match location {
                Location::London => (
                    LONDON_HORIZONTAL_TOP,
                    LONDON_HORIZONTAL_BOT,
                    Some(LONDON_HORIZONTAL_MID),
                    vec![')', '|', '(', '|'],
                    vec!['(', '|', ')', '|'],
                ),
                Location::Savannah => (
                    SAVANNAH_HORIZONTAL_TOP,
                    SAVANNAH_HORIZONTAL_BOT,
                    Some(SAVANNAH_HORIZONTAL_MID),
                    vec!['├', '┤'],
                    vec!['┤', '├'],
                ),
                Location::Lisbon => (
                    LISBON_HORIZONTAL_TOP,
                    LISBON_HORIZONTAL_BOT,
                    Some(LISBON_HORIZONTAL_MID),
                    vec!['▚', '▞'],
                    vec!['▞', '▚'],
                ),
                Location::Amsterdam => (
                    AMSTERDAM_HORIZONTAL_TOP,
                    AMSTERDAM_HORIZONTAL_BOT,
                    Some(AMSTERDAM_HORIZONTAL_MID),
                    vec!['◊'],
                    vec!['◊'],
                ),
                Location::CapeTown => (
                    CAPETOWN_HORIZONTAL_TOP,
                    CAPETOWN_HORIZONTAL_BOT,
                    Some(CAPETOWN_HORIZONTAL_MID),
                    vec!['○', '●'],
                    vec!['○', '●'],
                ),
                Location::Venice => (
                    VENICE_HORIZONTAL_TOP,
                    VENICE_HORIZONTAL_BOT,
                    Some(VENICE_HORIZONTAL_MID),
                    vec!['╳'],
                    vec!['╳'],
                ),
            },
        };
        comp!(
            f,
            MoveTo(0, 0),
            Print(top),
            MoveTo(0, FRAME_HEIGHT),
            Print(bot),
            MoveTo(0, 1),
            VerticalSequence {
                char_sequence: left_char_seq,
                len: FRAME_HEIGHT - 1
            },
            MoveTo(FRAME_WIDTH - 1, 1),
            VerticalSequence {
                char_sequence: right_char_seq,
                len: FRAME_HEIGHT - 1
            },
        );
        if let Some(mid) = mid {
            comp!(f, MoveTo(1, LOCATION_DIVIDER_Y), Print(mid),);
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
    fn simple_empty_inside() {
        assert_eq!(
            render_component(Frame(FrameType::SimpleEmptyInside)),
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

    #[test]
    fn location_london() {
        assert_eq!(
            render_component(Frame(FrameType::Location(Location::London))),
            r#"
'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
('~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~')
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'
"#
            .trim()
        );
    }

    #[test]
    fn location_savannah() {
        assert_eq!(
            render_component(Frame(FrameType::Location(Location::Savannah))),
            r#"
┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┤                                                                                                 ├
├                                                                                                 ┤
┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬┴┬
"#
            .trim()
        );
    }

    #[test]
    fn location_lisbon() {
        assert_eq!(
            render_component(Frame(FrameType::Location(Location::Lisbon))),
            r#"
▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄█▀█▄▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞                                                                                                 ▚
▚                                                                                                 ▞
▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞
"#
            .trim()
        );
    }

    #[test]
    fn location_amsterdam() {
        assert_eq!(
            render_component(Frame(FrameType::Location(Location::Amsterdam))),
            r#"
▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◁▷◊◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◊                                                                                                 ◊
◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁▷◁
"#
            .trim()
        );
    }

    #[test]
    fn location_capetown() {
        assert_eq!(
            render_component(Frame(FrameType::Location(Location::CapeTown))),
            r#"
◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐◓◑◒◐○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
●                                                                                                 ●
○                                                                                                 ○
◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●◖◗●
"#
            .trim()
        );
    }

    #[test]
    fn location_venice() {
        assert_eq!(
            render_component(Frame(FrameType::Location(Location::Venice))),
            r#"
╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱
"#
            .trim()
        );
    }
}
