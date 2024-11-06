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
    state::{GameState, PriceRanges},
};

const GAME_OVER: &str = r"
  _____                         ____                 
 / ____|                       / __ \                
| |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __ 
| | |_ |/ _` | '_ ` _ \ / _ \ | |  | \ \ / / _ \ '__|
| |__| | (_| | | | | | |  __/ | |__| |\ V /  __/ |   
 \_____|\__,_|_| |_| |_|\___|  \____/  \_/ \___|_|   
";

pub struct GameEndScreen<'a>(pub &'a GameState);

pub enum AchievementTier {
    InDebt,
    Poor,
    Ok,
    Good,
    Great,
}

impl AchievementTier {
    pub fn from_net_worth(net_worth: i32, overall_price_ranges: &PriceRanges) -> Self {
        let net_worth = net_worth as i64;
        if net_worth <= 0 {
            AchievementTier::InDebt
        } else if net_worth <= (overall_price_ranges.rum.1 as i64).saturating_mul(50) {
            AchievementTier::Poor
        } else if net_worth <= (overall_price_ranges.sugar.1 as i64).saturating_mul(50) {
            AchievementTier::Ok
        } else if net_worth <= (overall_price_ranges.tea.1 as i64).saturating_mul(50) {
            AchievementTier::Good
        } else {
            AchievementTier::Great
        }
    }
}

impl<'a> Command for GameEndScreen<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let state = self.0;
        let starting_net_worth = state.starting_debt.0 as i64 - state.starting_gold.0 as i64;
        // let starting_gold = state.starting_gold.0;
        // let starting_debt = state.starting_debt.0;
        let final_net_worth = state.net_worth();
        let achievement_tier = AchievementTier::from_net_worth(
            final_net_worth,
            &state.location_config.overall_price_ranges,
        );
        // results
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All),
            Hide,
            Frame(FrameType::SimpleEmptyInside),
            ScreenCenteredText::new(&["After three years, you went from being".to_owned()], 13),
            ScreenCenteredText::new_styleds(
                &[style(format!("{starting_net_worth} gold in debt").as_str())
                    .attribute(Attribute::Bold)],
                15
            ),
            ScreenCenteredText::new(
                &[format!(
                    "to {}",
                    if final_net_worth >= 0 {
                        "having"
                    } else {
                        "being"
                    }
                )],
                17
            ),
            ScreenCenteredText::new_styleds(
                &[style(
                    (if final_net_worth >= 0 {
                        format!("{} gold", final_net_worth)
                    } else {
                        format!("{} gold in debt", final_net_worth.abs())
                    })
                    .as_str()
                )
                .attribute(Attribute::Bold)],
                19
            ),
        );
        match achievement_tier {
            AchievementTier::InDebt => comp!(
                f,
                ScreenCenteredText::new(
                    &["Obviously, your father is disappointed in you.".to_owned()],
                    23
                ),
                ScreenCenteredText::new(
                    &["He has made the decision never to retire.".to_owned()],
                    25
                )
            ),
            AchievementTier::Poor => comp!(
                f,
                ScreenCenteredText::new(&["Your father expected more from you.".to_owned()], 23),
                ScreenCenteredText::new(
                    &["It will likely be a long time before your father retires.".to_owned()],
                    25
                )
            ),
            AchievementTier::Ok => comp!(
                f,
                ScreenCenteredText::new(
                    &[
                        "You showed your father that when you really put your mind to something"
                            .to_owned()
                    ],
                    23
                ),
                ScreenCenteredText::new(
                    &["you can really achieve middling results.".to_owned()],
                    25
                )
            ),
            AchievementTier::Good => comp!(
                f,
                ScreenCenteredText::new(&["You have done well.".to_owned()], 23),
                ScreenCenteredText::new(
                    &["Your father is confident handing over the reins to you.".to_owned()],
                    25
                )
            ),
            AchievementTier::Great => comp!(
                f,
                ScreenCenteredText::new(
                    &["Your father never dreamed his child could be this successful.".to_owned()],
                    23
                ),
                ScreenCenteredText::new(
                    &[
                        "He has decided to retire today, handing the full empire over to you."
                            .to_owned()
                    ],
                    25
                )
            ),
        }
        comp!(
            f,
            ScreenCenteredText::new(&["(q) to quit, (Enter) to play again".to_owned()], 29),
        );
        // "game over" ascii art terxt
        const OFFSET_X: u16 = 23;
        const OFFSET_Y: u16 = 4;
        for (i, line) in GAME_OVER.trim_matches('\n').lines().enumerate() {
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
