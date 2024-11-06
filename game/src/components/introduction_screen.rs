use std::fmt::{self};

use crossterm::{
    cursor::Hide,
    style::{style, Attribute, Stylize},
    terminal::Clear,
    Command,
};

use crate::{
    comp,
    components::{Frame, FrameType, ScreenCenteredText},
    state::Location,
};

pub struct IntroductionScreen {
    pub home: Location,
    pub starting_year: u16,
}

impl Command for IntroductionScreen {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let IntroductionScreen {
            home,
            starting_year,
        } = self;
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All),
            Frame(FrameType::Location(self.home)),
            ScreenCenteredText::new_styleds(
                &[
                    style("The year is "),
                    style(format!("{starting_year}").as_str()).attribute(Attribute::Bold),
                    style("."),
                ],
                5
            ),
            ScreenCenteredText::new_styleds(
                &[
                    style("Your father, a rich merchant captain from "),
                    style(format!("{home}").as_str()).attribute(Attribute::Bold),
                    style(","),
                ],
                7
            ),
            ScreenCenteredText::new_styleds(
                &[style(
                    "is preparing to retire and he is looking for a successor."
                ),],
                8
            ),
            ScreenCenteredText::new_styleds(
                &[style(
                    "He has issued a challenge to you: build a merchant empire"
                ),],
                10
            ),
            ScreenCenteredText::new_styleds(
                &[style(
                    "of your own to prove that you are worthy to carry on his legacy."
                ),],
                11
            ),
            ScreenCenteredText::new_styleds(
                &[
                    style("You have "),
                    style("three years").attribute(Attribute::Bold),
                    style(" to make as much money as possible."),
                ],
                14
            ),
            ScreenCenteredText::new_styleds(
                &[style("Fair winds and following seas, captain."),],
                24
            ),
            ScreenCenteredText::new_styleds(
                &[style("Press any key to continue").attribute(Attribute::Bold)],
                27
            ),
            Hide
        );
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
    fn location_london() {
        assert_eq!(
            render_component(IntroductionScreen {
                home: Location::London,
                starting_year: 1782
            }),
            r#"
'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                                                                                                 |
)                                        The year is 1782.                                        (
|                                                                                                 |
(                        Your father, a rich merchant captain from London,                        )
|                    is preparing to retire and he is looking for a successor.                    |
)                                                                                                 (
|                    He has issued a challenge to you: build a merchant empire                    |
(                 of your own to prove that you are worthy to carry on his legacy.                )
|                                                                                                 |
)                                                                                                 (
|                     You have three years to make as much money as possible.                     |
(                                                                                                 )
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
('~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~'~~.~~')
|                                                                                                 |
)                                                                                                 (
|                                                                                                 |
(                                                                                                 )
|                             Fair winds and following seas, captain.                             |
)                                                                                                 (
|                                                                                                 |
(                                    Press any key to continue                                    )
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
    fn location_venice() {
        assert_eq!(
            render_component(IntroductionScreen {
                home: Location::Venice,
                starting_year: 1785
            }),
            r#"
╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲╳╱╳╲
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                        The year is 1785.                                        ╳
╳                                                                                                 ╳
╳                        Your father, a rich merchant captain from Venice,                        ╳
╳                    is preparing to retire and he is looking for a successor.                    ╳
╳                                                                                                 ╳
╳                    He has issued a challenge to you: build a merchant empire                    ╳
╳                 of your own to prove that you are worthy to carry on his legacy.                ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                     You have three years to make as much money as possible.                     ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╱╱╲╲╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                             Fair winds and following seas, captain.                             ╳
╳                                                                                                 ╳
╳                                                                                                 ╳
╳                                    Press any key to continue                                    ╳
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
