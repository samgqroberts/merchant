use std::{
    fmt::{self},
    num::Saturating,
};

use chrono::Month;
use crossterm::{
    cursor::{Hide, MoveDown, MoveRight, MoveTo, Show},
    style::{style, Attribute, Print, Stylize},
    terminal::Clear,
    Command,
};

use crate::{
    comp,
    state::{
        GameState, Good, GoodsStolenResult, Inventory, Location, NoEffectEvent,
        PirateEncounterState, Transaction, CANNON_COST,
    },
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
            Frame(true),
            MoveTo(29, 12),
            Print("A tribute to Drug Wars by samgqroberts"),
            MoveTo(38, 14),
            Print("www.samgqroberts.com"),
            MoveTo(1, 23),
            Print(
                CenteredText(
                    "You have three years to make as much gold as you can!".to_string(),
                    (FRAME_WIDTH - 2).into()
                )
                .to_string()
            ),
            MoveTo(37, 25),
            Print(style("Press any key to begin").attribute(Attribute::Bold),),
            Hide
        );
        const OFFSET_X: u16 = 27;
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

const GAME_OVER: &str = r"
  _____                         ____                 
 / ____|                       / __ \                
| |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __ 
| | |_ |/ _` | '_ ` _ \ / _ \ | |  | \ \ / / _ \ '__|
| |__| | (_| | | | | | |  __/ | |__| |\ V /  __/ |   
 \_____|\__,_|_| |_| |_|\___|  \____/  \_/ \___|_|   
";

pub struct GameEndScreen<'a>(pub &'a GameState);

impl<'a> Command for GameEndScreen<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let state = self.0;
        let final_gold: i64 = ((state.gold + state.bank).0 as i64) - (state.debt.0 as i64);
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All),
            Hide,
            Frame(true),
            MoveTo(1, 14),
            Print(
                CenteredText("Congratulations!".to_string(), (FRAME_WIDTH - 2).into())
                    .to_string()
                    .attribute(Attribute::Bold)
            ),
            MoveTo(1, 20),
            Print(CenteredText(
                "After three years, you went from being".to_string(),
                (FRAME_WIDTH - 2).into()
            )),
            MoveTo(1, 22),
            Print(
                CenteredText("1400 gold in debt".to_string(), (FRAME_WIDTH - 2).into())
                    .to_string()
                    .attribute(Attribute::Bold)
            ),
            MoveTo(1, 24),
            Print(
                CenteredText(
                    format!("to {}", if final_gold >= 0 { "having" } else { "being" }),
                    (FRAME_WIDTH - 2).into()
                )
                .to_string()
            ),
            MoveTo(1, 26),
            Print(
                CenteredText(
                    (if final_gold >= 0 {
                        format!("{} gold", final_gold)
                    } else {
                        format!("{} gold in debt", final_gold.abs())
                    })
                    .to_string(),
                    (FRAME_WIDTH - 2).into()
                )
                .to_string()
                .attribute(Attribute::Bold)
            ),
        );
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

pub struct BankWithdrawInput<'a>(pub &'a Option<u32>);

impl<'a> Command for BankWithdrawInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        let prompt = format!(
            "want to withdraw? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("How much gold do you"),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            Print(prompt),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            Print(format!("(b) <- back")),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct Numeric4Digits(u32);

impl std::fmt::Display for Numeric4Digits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = self.0.to_string();
        while value.len() < 4 {
            value.insert(0, ' ')
        }
        write!(f, "{}", value)
    }
}

pub struct Numeric7Digits(u32);

impl std::fmt::Display for Numeric7Digits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = self.0.to_string();
        while value.len() < 7 {
            value.insert(0, ' ')
        }
        write!(f, "{}", value)
    }
}

pub struct InventoryList<'a>(pub &'a Inventory, pub u16, pub u16);

impl<'a> Command for InventoryList<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let inventory = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        comp!(
            f,
            MoveTo(offset_x + 4, offset_y),
            Print(format!("Tea: {}", Numeric4Digits(inventory.tea))),
            MoveTo(offset_x + 1, offset_y + 1),
            Print(format!("Coffee: {}", Numeric4Digits(inventory.coffee))),
            MoveTo(offset_x + 2, offset_y + 2),
            Print(format!("Sugar: {}", Numeric4Digits(inventory.sugar))),
            MoveTo(offset_x, offset_y + 3),
            Print(format!("Tobacco: {}", Numeric4Digits(inventory.tobacco))),
            MoveTo(offset_x + 4, offset_y + 4),
            Print(format!("Rum: {}", Numeric4Digits(inventory.rum))),
            MoveTo(offset_x + 1, offset_y + 5),
            Print(format!("Cotton: {}", Numeric4Digits(inventory.cotton))),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct CurrentPrices<'a>(pub &'a Inventory);

impl<'a> Command for CurrentPrices<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let prices = self.0;
        const OFFSET_X: u16 = 53;
        const OFFSET_Y: u16 = 23;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("Captain, the prices of goods here are:"),
            InventoryList(prices, OFFSET_X + 11, OFFSET_Y + 1),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct KeyInputAction {
    num_key: u8,
    char_key: char,
    text: String,
}

impl Command for KeyInputAction {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let char_index = self
            .text
            .to_ascii_lowercase()
            .char_indices()
            .find(|x| x.1 == self.char_key)
            .expect("Char for KeyInputAction not found in text")
            .0;
        let segment_before = &self.text[..char_index];
        let char_segment = &self.text[char_index..(char_index + 1)];
        let segment_after = &self.text[(char_index + 1)..];
        comp!(
            f,
            Print(format!("({}) ", self.num_key)),
            Print(segment_before),
            Print(style(char_segment).attribute(Attribute::Underlined)),
            Print(segment_after),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct ViewingInventoryActions<'a> {
    pub location: &'a Location,
    pub home_port: &'a Location,
    pub debt: u32,
}

impl<'a> Command for ViewingInventoryActions<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let location = self.location;
        let home_port = self.home_port;
        let debt = self.debt;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            // actions
            MoveTo(OFFSET_X, OFFSET_Y),
            KeyInputAction {
                num_key: 1,
                char_key: 'b',
                text: "Buy".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            KeyInputAction {
                num_key: 2,
                char_key: 's',
                text: "Sell".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            KeyInputAction {
                num_key: 3,
                char_key: 'a',
                text: "Sail".to_owned()
            },
        );
        if location == home_port {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + 3),
                KeyInputAction {
                    num_key: 4,
                    char_key: 'd',
                    text: "Stash deposit".to_owned()
                },
                MoveTo(OFFSET_X, OFFSET_Y + 4),
                KeyInputAction {
                    num_key: 5,
                    char_key: 'w',
                    text: "Stash withdraw".to_owned()
                },
                MoveTo(OFFSET_X, OFFSET_Y + 5),
                KeyInputAction {
                    num_key: 6,
                    char_key: 'e',
                    text: "Bank deposit".to_owned()
                },
                MoveTo(OFFSET_X, OFFSET_Y + 6),
                KeyInputAction {
                    num_key: 7,
                    char_key: 'i',
                    text: "Bank withdraw".to_owned()
                },
            );
            if debt > 0 {
                comp!(
                    f,
                    MoveTo(OFFSET_X, OFFSET_Y + 7),
                    KeyInputAction {
                        num_key: 8,
                        char_key: 'p',
                        text: "Pay down debt".to_owned()
                    },
                );
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct BankDepositInput<'a>(pub &'a Option<u32>);

impl<'a> Command for BankDepositInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        let prompt = format!(
            "to deposit in the bank? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("How much gold do you want"),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            Print(prompt),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            Print(format!("(b) <- back")),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

const FRAME_WIDTH: u16 = 99;
const FRAME_HEIGHT: u16 = 32;

pub struct HorizontalLine(pub u16 /* y-index */, pub bool /* full-width */);

impl Command for HorizontalLine {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            MoveTo(if self.1 { 0 } else { 1 }, self.0),
            Print("-".repeat(if self.1 { FRAME_WIDTH } else { FRAME_WIDTH - 2 }.into()))
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct Frame(bool);

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

pub struct TopCenterFramed(String);

impl Command for TopCenterFramed {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            HorizontalLine(2, false),
            MoveTo(40, 0),
            Print("|=================|"),
            MoveTo(40, 1),
            Print(format!("|{}|", self.0)),
            MoveTo(40, 2),
            Print("|=================|"),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct Date<'a>(&'a (u16, Month));

impl<'a> Command for Date<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let month_name = self.0 .1.name();
        let mut year = self.0 .0.to_string();
        const TOTAL_NUM_CHARS: u8 = 15;
        while year.len() + month_name.len() < TOTAL_NUM_CHARS.into() {
            year.insert(0, ' ');
        }
        let text = format!(" {}{} ", month_name, year);
        comp!(f, TopCenterFramed(text));
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

impl<'a> From<&'a GameState> for Date<'a> {
    fn from(value: &'a GameState) -> Self {
        Self(&value.date)
    }
}

pub struct HomeBase<'a> {
    stash: &'a Inventory,
    bank: u32,
    debt: u32,
    location: &'a Location,
    home_port: &'a Location,
}

impl<'a> From<&'a GameState> for HomeBase<'a> {
    fn from(value: &'a GameState) -> Self {
        HomeBase {
            stash: &value.stash,
            bank: value.bank.0,
            debt: value.debt.0,
            location: &value.location,
            home_port: &value.location_config.home_port,
        }
    }
}

impl<'a> Command for HomeBase<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        const HOME: &str = r###"
  _____[LLL]______[LLL]____
 /     [LLL]      [LLL]    \
/___________________________\
 )=========================(
 '|I .--. I               I|
  |I | +| I               I|
  |I_|_+|_I               I|
 /_I______I               I_\
  )========               =(
  |I .--. I               I|
  |I |<>| I               I|
  |I |~ | I               I|
  |I |  | I               I|
  |I_|__|_I_______________I|
###(______)##################
 ##(________)   ~"^"^~   ## 
"###;
        const OFFSET_X: u16 = 4;
        const OFFSET_Y: u16 = 3;
        for (i, line) in HOME.trim_matches('\n').lines().enumerate() {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + (i as u16)),
                Print(line.to_string()),
            );
        }
        comp!(
            f,
            InventoryList(self.stash, OFFSET_X + 12, OFFSET_Y + 4),
            MoveTo(OFFSET_X + 12, OFFSET_Y + 11),
            Print(format!("Bank: {}", Numeric7Digits(self.bank))),
            MoveTo(OFFSET_X + 12, OFFSET_Y + 12),
            Print(format!("Debt: {}", Numeric7Digits(self.debt))),
        );
        const PATH_CONTINUATION: &str = r###"
(_________)
(__________)
"###;
        if self.location == self.home_port {
            for (i, line) in PATH_CONTINUATION.trim_matches('\n').lines().enumerate() {
                comp!(
                    f,
                    MoveTo(OFFSET_X + 3, OFFSET_Y + 16 + (i as u16)),
                    Print(line.to_string()),
                );
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

struct CenteredText(String, u32);

impl std::fmt::Display for CenteredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = self.0.clone();
        let mut add_before = value.len() % 2 == 1;
        while value.len() < self.1.try_into().unwrap() {
            let position = if add_before { 0 } else { value.len() };
            value.insert(position, ' ');
            add_before = !add_before
        }
        write!(f, "{}", value)
    }
}

pub struct CurrentLocation<'a> {
    location: &'a Location,
}

impl<'a> From<&'a GameState> for CurrentLocation<'a> {
    fn from(value: &'a GameState) -> Self {
        CurrentLocation {
            location: &value.location,
        }
    }
}

impl<'a> Command for CurrentLocation<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            MoveTo(42, 19),
            Print("<------------->"),
            MoveTo(42, 20),
            Print(format!("|{}|", CenteredText(self.location.to_string(), 13))),
            MoveTo(42, 21),
            Print("<------------->"),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct ViewingInventoryBase<'a>(pub &'a GameState);

impl<'a> Command for ViewingInventoryBase<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let state = self.0;
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All), // clear the terminal
            Hide,                                       // hide the cursor
            Frame(false),
            Date::from(state),
            HomeBase::from(state),
            Ship::from(state),
            CurrentLocation::from(state),
            CurrentPrices(&state.locations.location_info(&state.location).prices)
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct Ship<'a> {
    inventory: &'a Inventory,
    gold: u32,
    hold_size: u32,
    cannons: u8,
}

impl<'a> From<&'a GameState> for Ship<'a> {
    fn from(value: &'a GameState) -> Self {
        Ship {
            inventory: &value.inventory,
            gold: value.gold.0,
            hold_size: value.hold_size.0,
            cannons: value.cannons.0,
        }
    }
}

impl<'a> Command for Ship<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        const SHIP: &str = r###"
                             |                            
                 |          )_)                           
                )_)        )___)         |                
               )___)       )____)       )_)\              
               )____)     /)_____)      )__)\             
              )_____)    /)______)\    )___) \            
             )______)  //)_______) \\ )_____) \\          
       _____//___|___///_____|______\\\__|_____\\\__=====
       \                                           /     
        \                                         /      
         \                                       /____   
--------- \                                     //.../---
   ^^^^^ ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^  ^^^/.../    
         ^^^^      ^^^    ^^^^^^^^^    ^^^^^  /..../     
                  ^^^      ^^^^             /...../      
                                          /....../       
"###;
        const OFFSET_X: u16 = 39;
        const OFFSET_Y: u16 = 3;
        for (i, line) in SHIP.trim_matches('\n').lines().enumerate() {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + (i as u16)),
                Print(line.to_string()),
            );
        }
        let inventory = self.inventory;
        comp!(
            f,
            MoveTo(OFFSET_X + 14, OFFSET_Y + 8),
            Print(format!("Tea: {}", Numeric4Digits(inventory.tea))),
            MoveTo(OFFSET_X + 24, OFFSET_Y + 8),
            Print(format!("Coffee: {}", Numeric4Digits(inventory.coffee))),
            MoveTo(OFFSET_X + 38, OFFSET_Y + 8),
            Print(format!("Sugar: {}", Numeric4Digits(inventory.sugar))),
            MoveTo(OFFSET_X + 10, OFFSET_Y + 9),
            Print(format!("Tobacco: {}", Numeric4Digits(inventory.tobacco))),
            MoveTo(OFFSET_X + 27, OFFSET_Y + 9),
            Print(format!("Rum: {}", Numeric4Digits(inventory.rum))),
            MoveTo(OFFSET_X + 37, OFFSET_Y + 9),
            Print(format!("Cotton: {}", Numeric4Digits(inventory.cotton))),
            MoveTo(OFFSET_X + 12, OFFSET_Y + 11),
            Print(format!("Gold: {}", Numeric7Digits(self.gold))),
            MoveTo(OFFSET_X + 26, OFFSET_Y + 11),
            Print(format!("Hold: {}", Numeric4Digits(self.hold_size))),
            MoveTo(OFFSET_X + 37, OFFSET_Y + 11),
            Print(format!("Cannons: {}", self.cannons)),
        );
        const DOCK_CONTINUATION_1: &str = r###"
/......../
"###;
        const DOCK_CONTINUATION_2: &str = r###"
/........./
"###;
        for (i, line) in DOCK_CONTINUATION_1.trim_matches('\n').lines().enumerate() {
            comp!(
                f,
                MoveTo(OFFSET_X + 40, OFFSET_Y + 16 + (i as u16)),
                Print(line.to_string()),
            );
        }
        for (i, line) in DOCK_CONTINUATION_2.trim_matches('\n').lines().enumerate() {
            comp!(
                f,
                MoveTo(OFFSET_X + 38, OFFSET_Y + 17 + (i as u16)),
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

pub struct BuyInput<'a> {
    pub info: &'a Transaction,
    pub state: &'a GameState,
}

impl<'a> Command for BuyInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.info;
        let state = self.state;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        // user has indicated which good they want to buy
        let gold = state.gold;
        let good = &info.good;
        let good_price = state
            .locations
            .location_info(&state.location)
            .prices
            .get_good(good);
        let prompt = format!(
            "How much {} do you want? {}",
            good,
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        let can_afford = gold / Saturating(*good_price);
        comp!(
            f,
            // prompt what to buy
            MoveTo(OFFSET_X, OFFSET_Y),
            Print(prompt),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            Print(format!("You can afford ({})", can_afford)),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            Print(format!("(b) <- back")),
        );
        let remaining_hold = state.remaining_hold();
        if remaining_hold < can_afford.0 {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + 2),
                Print(format!("You have space for ({})", remaining_hold)),
            )
        }
        comp!(f, MoveTo(OFFSET_X + prompt_len, OFFSET_Y), Show);
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

const PROMPT_OFFSET_X: u16 = 10;
const PROMPT_OFFSET_Y: u16 = 23;

pub struct BuyPrompt;

impl Command for BuyPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("Which do you want to buy?"),
            GoodOptions(OFFSET_X, OFFSET_Y + 1)
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct GoodOptions(pub u16, pub u16);

impl Command for GoodOptions {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let offset_x = self.0;
        let offset_y = self.1;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            KeyInputAction {
                num_key: 1,
                char_key: 't',
                text: "Tea".to_owned()
            },
            MoveTo(offset_x, offset_y + 1),
            KeyInputAction {
                num_key: 2,
                char_key: 'c',
                text: "Coffee".to_owned()
            },
            MoveTo(offset_x, offset_y + 2),
            KeyInputAction {
                num_key: 3,
                char_key: 's',
                text: "Sugar".to_owned()
            },
            MoveTo(offset_x, offset_y + 3),
            KeyInputAction {
                num_key: 4,
                char_key: 'a',
                text: "Tobacco".to_owned()
            },
            MoveTo(offset_x, offset_y + 4),
            KeyInputAction {
                num_key: 5,
                char_key: 'r',
                text: "Rum".to_owned()
            },
            MoveTo(offset_x, offset_y + 5),
            KeyInputAction {
                num_key: 6,
                char_key: 'o',
                text: "Cotton".to_owned()
            },
            MoveTo(offset_x, offset_y + 6),
            Print("(b) <- back"),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct SellInput<'a>(pub &'a Transaction, pub &'a u32);

impl<'a> Command for SellInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.0;
        let current_amount = self.1;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        let good = &info.good;
        let prompt = format!(
            "want to sell? {}",
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print(format!("How much {} do you", good)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            Print(prompt),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            Print(format!("You have ({})", current_amount)),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            Print(format!("(b) <- back")),
            // position cursor for input
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct SellPrompt;

impl Command for SellPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("Which do you want to sell?"),
            GoodOptions(OFFSET_X, OFFSET_Y + 1),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct SailPrompt;

impl Command for SailPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("Where do you want to sail?"),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            KeyInputAction {
                num_key: 1,
                char_key: 'l',
                text: "London".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            KeyInputAction {
                num_key: 2,
                char_key: 's',
                text: "Savannah".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 3),
            KeyInputAction {
                num_key: 3,
                char_key: 'i',
                text: "Lisbon".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 4),
            KeyInputAction {
                num_key: 4,
                char_key: 'a',
                text: "Amsterdam".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            KeyInputAction {
                num_key: 5,
                char_key: 'c',
                text: "Cape Town".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 6),
            KeyInputAction {
                num_key: 6,
                char_key: 'v',
                text: "Venice".to_owned()
            },
            MoveTo(OFFSET_X, OFFSET_Y + 7),
            Print("(b) <- back"),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct StashDepositInput<'a>(pub &'a Transaction, pub &'a u32);

impl<'a> Command for StashDepositInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.0;
        let current_amount = self.1;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        // user has indicated which good they want to stash
        let good = &info.good;
        let prompt = format!(
            "want to stash? {}",
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print(format!("How much {} do you", good)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            Print(prompt),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            Print(format!("You have ({})", current_amount)),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            Print(format!("(b) <- back")),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct StashDepositPrompt;

impl Command for StashDepositPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let offset_x: u16 = PROMPT_OFFSET_X;
        let offset_y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            Print("Which do you want to stash?"),
            GoodOptions(offset_x, offset_y + 1),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct StashWithdrawInput<'a>(pub &'a Transaction, pub &'a u32);

impl<'a> Command for StashWithdrawInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.0;
        let current_amount = self.1;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        // user has indicated which good they want to stash
        let good = &info.good;
        let prompt = format!(
            "want to withdraw? {}",
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print(format!("How much {} do you", good)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            Print(prompt),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            Print(format!("There are ({})", current_amount)),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            Print(format!("(b) <- back")),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct StashWithdrawPrompt;

impl Command for StashWithdrawPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("Which do you want to withdraw?"),
            GoodOptions(OFFSET_X, OFFSET_Y + 1),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct PayDebtInput<'a>(pub &'a Option<u32>);

impl<'a> Command for PayDebtInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        let prompt = format!(
            "want to pay down? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            Print("How much debt do you"),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            Print(prompt),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            Print(format!("(b) <- back")),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct CheapGoodDialog<'a>(pub &'a Good);

impl<'a> Command for CheapGoodDialog<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let good = self.0;
        comp!(
            f,
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
            Print("The market is bursting with".to_owned()),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
            Print(format!("a surplus of {}, lowering", good)),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 2),
            Print("the price significantly!".to_owned()),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 5),
            Print("(press any key to continue)".to_string())
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct ExpensiveGoodDialog<'a>(pub &'a Good);

impl<'a> Command for ExpensiveGoodDialog<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let good = self.0;
        comp!(
            f,
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
            Print("A trade ship that was delivering".to_owned()),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
            Print(format!("a large shipment of {} has", good)),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 2),
            Print("wrecked at sea, increasing the local".to_owned()),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 3),
            Print("price significantly!".to_owned()),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 5),
            Print("(press any key to continue)".to_string())
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct FindGoodsDialog<'a>(pub &'a Good, pub &'a u32, pub &'a GameState);

impl<'a> Command for FindGoodsDialog<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let good = self.0;
        let amount = self.1;
        let state = self.2;
        comp!(
            f,
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
            Print(format!("You notice a crate on the docks")),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
            Print(format!("that seems to be abandoned.")),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 2),
            Print(format!("After eyeing it for a while, you decide")),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 3),
            Print(format!("to claim it. Inside you find {} {}!", amount, good)),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 6),
            Print("(press any key to continue)".to_string())
        );
        let remaining_hold = state.remaining_hold();
        if &remaining_hold < amount {
            comp!(
                f,
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 4),
                Print(format!("You have space for ({})", remaining_hold)),
            )
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}
pub struct GoodsStolenDialog(pub GoodsStolenResult);

impl Command for GoodsStolenDialog {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self.0 {
            GoodsStolenResult::NothingStolen => comp!(
                f,
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
                Print("Thieves were on the prowl, but they"),
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
                Print("couldn't find anything to steal"),
            ),
            GoodsStolenResult::WasStolen { good, amount } => comp!(
                f,
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
                Print("Prowling harbor thieves stole"),
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
                Print(format!("{} {} from you!", amount, good)),
            ),
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct CanBuyCannon;

impl Command for CanBuyCannon {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
            Print("An enterprising gentleman on the docks"),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
            Print("offers to outfit your ship with an"),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 2),
            Print(format!("additional cannon for {CANNON_COST} gold.")),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 4),
            Print("Accept? y/n"),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct CanBuyHoldSpace {
    pub price: u32,
    pub more_hold: u32,
}

impl Command for CanBuyHoldSpace {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
            Print("An earnest youth on the docks"),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
            Print("offers to clear out the unusable"),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 2),
            Print("space in your hold."),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 3),
            Print(format!(
                "Pay him {} gold for {} more hold space?",
                self.price, self.more_hold
            )),
            MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 5),
            Print("(y/n)"),
        );
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct PirateEncounter {
    pub pirate_encounter_state: PirateEncounterState,
    pub cannons: u8,
    pub date: (u16, Month),
}

impl From<(PirateEncounterState, &mut GameState)> for PirateEncounter {
    fn from(value: (PirateEncounterState, &mut GameState)) -> Self {
        PirateEncounter {
            pirate_encounter_state: value.0,
            cannons: value.1.cannons.0,
            date: value.1.date,
        }
    }
}

impl Command for PirateEncounter {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All), // clear the terminal
            Frame(true),
            Date(&self.date),
        );
        match self.pirate_encounter_state {
            PirateEncounterState::Initial => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        "Pirates have found you on the open seas!".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 15),
                    Print(CenteredText(
                        "(press any key)".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                );
            }
            PirateEncounterState::Prompt { info } => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        format!(
                            "Health {}, Pirates {}, Cannons {}.",
                            info.health, info.cur_pirates, self.cannons
                        ),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 15),
                    Print(CenteredText(
                        "Will you (r)un or (f)ight ?".to_owned(),
                        (FRAME_WIDTH - 2).into()
                    )),
                );
            }
            PirateEncounterState::RunSuccess => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        "You've successfully evaded the pirates!".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 16),
                    Print(CenteredText(
                        "(press any key to continue)".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                );
            }
            PirateEncounterState::RunFailure { info } => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        format!(
                            "Health {}, Pirates {}, Cannons {}.",
                            info.health, info.cur_pirates, self.cannons
                        ),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 12),
                    Print(CenteredText(
                        "The pirates manouver to cut off your escape!".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 16),
                    Print(CenteredText(
                        "(press any key to continue)".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                )
            }
            PirateEncounterState::PiratesAttack {
                info,
                damage_this_attack,
            } => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        format!(
                            "Health {}, Pirates {}, Cannons {}.",
                            info.health, info.cur_pirates, self.cannons
                        ),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 12),
                    Print(CenteredText(
                        format!("The pirates fire their cannons at you, doing {damage_this_attack} damage!"),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 16),
                    Print(CenteredText(
                        "(press any key to continue)".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                )
            }
            PirateEncounterState::Destroyed => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        "The pirates have conquered you!".to_owned(),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 12),
                    Print(CenteredText(
                        "They get away with half your gold and all of your goods!".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 16),
                    Print(CenteredText(
                        "(press any key to continue)".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                )
            }
            PirateEncounterState::AttackResult {
                info,
                did_kill_a_pirate,
            } => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        format!(
                            "Health {}, Pirates {}, Cannons {}.",
                            info.health, info.cur_pirates, self.cannons
                        ),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 12),
                    Print(CenteredText(
                        format!(
                            "You fire your cannons at the pirates, {}",
                            if did_kill_a_pirate {
                                "and you sink one of them!"
                            } else {
                                "but you only hit water!"
                            }
                        ),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 16),
                    Print(CenteredText(
                        "(press any key to continue)".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                )
            }
            PirateEncounterState::Victory { gold_recovered } => {
                comp!(
                    f,
                    MoveTo(1, 8),
                    Print(CenteredText(
                        "You have sank all of the pirates!".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 12),
                    Print(CenteredText(
                        format!("You recover {} gold from the wreckage!", gold_recovered),
                        (FRAME_WIDTH - 2).into()
                    )),
                    MoveTo(1, 16),
                    Print(CenteredText(
                        "(press any key to continue)".to_string(),
                        (FRAME_WIDTH - 2).into()
                    )),
                )
            }
        };
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct NoEffect {
    pub variant: NoEffectEvent,
}

impl Command for NoEffect {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self.variant {
            NoEffectEvent::SunnyDay => {
                comp!(
                    f,
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
                    Print("As your ship glides into the harbor"),
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
                    Print("you take a moment and feel the"),
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 2),
                    Print("comforting warmth of the sun"),
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 3),
                    Print("on your face."),
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 5),
                    Print("(press any key to continue)".to_string())
                );
            }
            NoEffectEvent::StormOnHorizon => {
                comp!(
                    f,
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
                    Print("You stand on the docks and look"),
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
                    Print("off to the horizon."),
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 2),
                    Print("You see an ominous storm forming."),
                    MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 5),
                    Print("(press any key to continue)".to_string())
                );
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}
