use std::fmt::{self};

use crossterm::{
    cursor::{Hide, MoveDown, MoveRight, MoveTo, MoveToNextLine, Show},
    style::{style, Attribute, Color, PrintStyledContent, Stylize},
    terminal::Clear,
    Command,
};

use crate::{
    comp,
    state::{GameState, Good, GoodsStolenResult, Inventory, Location, Transaction},
};

pub struct SplashScreen();

impl Command for SplashScreen {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All),
            MoveTo(0, 0),
            PrintStyledContent(
                "Merchant"
                    .with(Color::Yellow)
                    .on(Color::Blue)
                    .attribute(Attribute::Bold)
            ),
            MoveTo(0, 2),
            PrintStyledContent(
                "Navigate shifting markets and unreliable sources."
                    .with(Color::Yellow)
                    .on(Color::Blue)
                    .attribute(Attribute::Bold)
            ),
            MoveTo(0, 4),
            PrintStyledContent(
                "By samgqroberts"
                    .with(Color::Yellow)
                    .on(Color::Blue)
                    .attribute(Attribute::Bold)
            ),
            // prompt user
            MoveToNextLine(2),
            PrintStyledContent(
                style("Press any key to begin")
                    .with(Color::Blue)
                    .on(Color::Yellow)
                    .attribute(Attribute::Bold),
            ),
            Hide
        );
        Ok(())
    }
}

pub struct GameEndScreen<'a>(pub &'a GameState);

impl<'a> Command for GameEndScreen<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let state = self.0;
        let final_gold: i64 = ((state.gold + state.bank) as i64) - (state.debt as i64);
        comp!(
            f,
            Clear(crossterm::terminal::ClearType::All),
            Hide,
            MoveTo(0, 0),
            PrintStyledContent(
                "Congratulations!!"
                    .with(Color::White)
                    .attribute(Attribute::Bold)
            ),
            MoveTo(0, 2),
            PrintStyledContent("After three years, you went from being".with(Color::White)),
            MoveTo(0, 3),
            PrintStyledContent(
                format!("1400 gold in debt")
                    .with(Color::White)
                    .attribute(Attribute::Bold)
            ),
            MoveTo(0, 4),
            PrintStyledContent(
                format!("to {}", if final_gold >= 0 { "having" } else { "being" })
                    .with(Color::White)
            ),
            MoveTo(0, 5),
            PrintStyledContent(
                format!(
                    "{}",
                    if final_gold >= 0 {
                        format!("{} gold", final_gold)
                    } else {
                        format!("{} gold in debt", final_gold.abs())
                    }
                )
                .with(Color::White)
                .attribute(Attribute::Bold)
            ),
        );
        Ok(())
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
            PrintStyledContent("How much gold do you".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
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
            PrintStyledContent(
                format!("Tea: {}", Numeric4Digits(inventory.tea)).with(Color::White)
            ),
            MoveTo(offset_x + 1, offset_y + 1),
            PrintStyledContent(
                format!("Coffee: {}", Numeric4Digits(inventory.coffee)).with(Color::White)
            ),
            MoveTo(offset_x + 2, offset_y + 2),
            PrintStyledContent(
                format!("Sugar: {}", Numeric4Digits(inventory.sugar)).with(Color::White)
            ),
            MoveTo(offset_x, offset_y + 3),
            PrintStyledContent(
                format!("Tobacco: {}", Numeric4Digits(inventory.tobacco)).with(Color::White)
            ),
            MoveTo(offset_x + 4, offset_y + 4),
            PrintStyledContent(
                format!("Rum: {}", Numeric4Digits(inventory.rum)).with(Color::White)
            ),
            MoveTo(offset_x + 1, offset_y + 5),
            PrintStyledContent(
                format!("Cotton: {}", Numeric4Digits(inventory.cotton)).with(Color::White)
            ),
        );
        Ok(())
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
            PrintStyledContent("Captain, the prices of goods here are:".with(Color::White)),
            InventoryList(prices, OFFSET_X + 11, OFFSET_Y + 1),
        );
        Ok(())
    }
}

pub struct ViewingInventoryActions<'a> {
    pub location: &'a Location,
    pub debt: u32,
}

impl<'a> Command for ViewingInventoryActions<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let location = self.location;
        let debt = self.debt;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            // actions
            MoveTo(OFFSET_X, OFFSET_Y),
            PrintStyledContent("(1) Buy".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent("(2) Sell".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            PrintStyledContent("(3) Sail".with(Color::White)),
        );
        if location == &Location::London {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + 3),
                PrintStyledContent("(4) Stash deposit".with(Color::White)),
                MoveTo(OFFSET_X, OFFSET_Y + 4),
                PrintStyledContent("(5) Stash withdraw".with(Color::White)),
                MoveTo(OFFSET_X, OFFSET_Y + 5),
                PrintStyledContent("(6) Bank deposit".with(Color::White)),
                MoveTo(OFFSET_X, OFFSET_Y + 6),
                PrintStyledContent("(7) Bank withdraw".with(Color::White)),
            );
            if debt > 0 {
                comp!(
                    f,
                    MoveTo(OFFSET_X, OFFSET_Y + 7),
                    PrintStyledContent("(8) Pay down debt".with(Color::White)),
                );
            }
        }
        Ok(())
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
            PrintStyledContent("How much gold do you want".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }
}

const FRAME_WIDTH: u16 = 99;
const FRAME_HEIGHT: u16 = 32;

pub struct Frame;

impl Command for Frame {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        // 2 horizontal lines at top and bottom ends
        for i in 0..(FRAME_WIDTH) {
            comp!(f, MoveTo(i, 0), PrintStyledContent("-".with(Color::White)));
        }
        for i in 0..(FRAME_WIDTH) {
            comp!(
                f,
                MoveTo(i, FRAME_HEIGHT),
                PrintStyledContent("-".with(Color::White)),
                MoveRight(1)
            );
        }
        // additional horizontal line under date
        for i in 0..(FRAME_WIDTH) {
            comp!(
                f,
                MoveTo(i, 2),
                PrintStyledContent("-".with(Color::White)),
                MoveRight(1)
            );
        }
        // additional thick horizontal line near location
        for i in 0..(FRAME_WIDTH) {
            comp!(
                f,
                MoveTo(i, 19),
                PrintStyledContent("=".with(Color::White)),
                MoveRight(1)
            );
        }
        // 2 vertical lines at left and right ends
        for i in 0..(FRAME_HEIGHT - 1) {
            comp!(
                f,
                MoveTo(0, 1 + i),
                PrintStyledContent("|".with(Color::White)),
                MoveDown(1)
            );
        }
        for i in 0..(FRAME_HEIGHT - 1) {
            comp!(
                f,
                MoveTo(FRAME_WIDTH - 1, 1 + i),
                PrintStyledContent("|".with(Color::White)),
                MoveDown(1)
            );
        }
        Ok(())
    }
}

pub struct Date;

impl Command for Date {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        comp!(
            f,
            MoveTo(40, 0),
            PrintStyledContent("|=================|".with(Color::White)),
            MoveTo(40, 1),
            PrintStyledContent("| March      1782 |".with(Color::White)),
            MoveTo(40, 2),
            PrintStyledContent("|=================|".with(Color::White)),
        );
        Ok(())
    }
}

pub struct HomeBase<'a> {
    stash: &'a Inventory,
    bank: u32,
    debt: u32,
    location: &'a Location,
}

impl<'a> From<&'a GameState> for HomeBase<'a> {
    fn from(value: &'a GameState) -> Self {
        HomeBase {
            stash: &value.stash,
            bank: value.bank,
            debt: value.debt,
            location: &value.location,
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
        for (i, line) in HOME.trim_matches('\n').lines().into_iter().enumerate() {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + (i as u16)),
                PrintStyledContent(format!("{}", line).with(Color::Grey)),
            );
        }
        comp!(
            f,
            InventoryList(self.stash, OFFSET_X + 12, OFFSET_Y + 4),
            MoveTo(OFFSET_X + 12, OFFSET_Y + 11),
            PrintStyledContent(format!("Bank: {}", Numeric7Digits(self.bank)).with(Color::White)),
            MoveTo(OFFSET_X + 12, OFFSET_Y + 12),
            PrintStyledContent(format!("Debt: {}", Numeric7Digits(self.debt)).with(Color::White)),
        );
        const PATH_CONTINUATION: &str = r###"
(_________)
(__________)
"###;
        if self.location == &Location::London {
            for (i, line) in PATH_CONTINUATION
                .trim_matches('\n')
                .lines()
                .into_iter()
                .enumerate()
            {
                comp!(
                    f,
                    MoveTo(OFFSET_X + 3, OFFSET_Y + 16 + (i as u16)),
                    PrintStyledContent(format!("{}", line).with(Color::Grey)),
                );
            }
        }
        Ok(())
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
            PrintStyledContent("<------------->".with(Color::White)),
            MoveTo(42, 20),
            PrintStyledContent(
                format!("|{}|", CenteredText(self.location.to_string(), 13)).with(Color::White)
            ),
            MoveTo(42, 21),
            PrintStyledContent("<------------->".with(Color::White)),
        );
        Ok(())
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
            Frame,
            Date,
            HomeBase::from(state),
            Ship::from(state),
            CurrentLocation::from(state),
            CurrentPrices(&state.locations.location_info(&state.location).prices)
        );
        Ok(())
    }
}

pub struct Ship<'a> {
    inventory: &'a Inventory,
    gold: u32,
    hold_size: u32,
}

impl<'a> From<&'a GameState> for Ship<'a> {
    fn from(value: &'a GameState) -> Self {
        Ship {
            inventory: &value.inventory,
            gold: value.gold,
            hold_size: value.hold_size,
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
        for (i, line) in SHIP.trim_matches('\n').lines().into_iter().enumerate() {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + (i as u16)),
                PrintStyledContent(format!("{}", line).with(Color::Grey)),
            );
        }
        let inventory = self.inventory;
        comp!(
            f,
            MoveTo(OFFSET_X + 14, OFFSET_Y + 8),
            PrintStyledContent(
                format!("Tea: {}", Numeric4Digits(inventory.tea)).with(Color::White)
            ),
            MoveTo(OFFSET_X + 24, OFFSET_Y + 8),
            PrintStyledContent(
                format!("Coffee: {}", Numeric4Digits(inventory.coffee)).with(Color::White)
            ),
            MoveTo(OFFSET_X + 38, OFFSET_Y + 8),
            PrintStyledContent(
                format!("Sugar: {}", Numeric4Digits(inventory.sugar)).with(Color::White)
            ),
            MoveTo(OFFSET_X + 10, OFFSET_Y + 9),
            PrintStyledContent(
                format!("Tobacco: {}", Numeric4Digits(inventory.tobacco)).with(Color::White)
            ),
            MoveTo(OFFSET_X + 27, OFFSET_Y + 9),
            PrintStyledContent(
                format!("Rum: {}", Numeric4Digits(inventory.rum)).with(Color::White)
            ),
            MoveTo(OFFSET_X + 37, OFFSET_Y + 9),
            PrintStyledContent(
                format!("Cotton: {}", Numeric4Digits(inventory.cotton)).with(Color::White)
            ),
            MoveTo(OFFSET_X + 13, OFFSET_Y + 11),
            PrintStyledContent(format!("Gold: {}", Numeric7Digits(self.gold)).with(Color::White)),
            MoveTo(OFFSET_X + 27, OFFSET_Y + 11),
            PrintStyledContent(
                format!("Hold: {}", Numeric4Digits(self.hold_size)).with(Color::White)
            ),
        );
        const DOCK_CONTINUATION_1: &str = r###"
/......../
"###;
        const DOCK_CONTINUATION_2: &str = r###"
/........./
"###;
        for (i, line) in DOCK_CONTINUATION_1
            .trim_matches('\n')
            .lines()
            .into_iter()
            .enumerate()
        {
            comp!(
                f,
                MoveTo(OFFSET_X + 40, OFFSET_Y + 16 + (i as u16)),
                PrintStyledContent(format!("{}", line).with(Color::Grey)),
            );
        }
        for (i, line) in DOCK_CONTINUATION_2
            .trim_matches('\n')
            .lines()
            .into_iter()
            .enumerate()
        {
            comp!(
                f,
                MoveTo(OFFSET_X + 38, OFFSET_Y + 17 + (i as u16)),
                PrintStyledContent(format!("{}", line).with(Color::Grey)),
            );
        }
        Ok(())
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
            .get_good(&good);
        let prompt = format!(
            "How much {} do you want? {}",
            good,
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        let can_afford = gold / good_price;
        comp!(
            f,
            // prompt what to buy
            MoveTo(OFFSET_X, OFFSET_Y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent(format!("You can afford ({})", can_afford).with(Color::White)),
        );
        let remaining_hold = state.remaining_hold();
        if remaining_hold < can_afford {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + 2),
                PrintStyledContent(
                    format!("You have space for ({})", remaining_hold).with(Color::White)
                ),
            )
        }
        comp!(f, MoveTo(OFFSET_X + prompt_len, OFFSET_Y), Show);
        Ok(())
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
            PrintStyledContent("Which do you want to buy?".with(Color::White)),
            GoodOptions(OFFSET_X, OFFSET_Y + 1)
        );
        Ok(())
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
            PrintStyledContent("(1) Tea".with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent("(2) Coffee".with(Color::White)),
            MoveTo(offset_x, offset_y + 2),
            PrintStyledContent("(3) Sugar".with(Color::White)),
            MoveTo(offset_x, offset_y + 3),
            PrintStyledContent("(4) Tobacco".with(Color::White)),
            MoveTo(offset_x, offset_y + 4),
            PrintStyledContent("(5) Rum".with(Color::White)),
            MoveTo(offset_x, offset_y + 5),
            PrintStyledContent("(6) Cotton".with(Color::White)),
        );
        Ok(())
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
            PrintStyledContent(format!("How much {} do you", good).with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            PrintStyledContent(format!("You have ({})", current_amount).with(Color::White)),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
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
            PrintStyledContent("Which do you want to sell?".with(Color::White)),
            GoodOptions(OFFSET_X, OFFSET_Y + 1),
        );
        Ok(())
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
            PrintStyledContent("Where do you want to sail?".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent("(1) London".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            PrintStyledContent("(2) Savannah".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 3),
            PrintStyledContent("(3) Lisbon".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 4),
            PrintStyledContent("(4) Amsterdam".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 5),
            PrintStyledContent("(5) Cape Town".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 6),
            PrintStyledContent("(6) Venice".with(Color::White)),
        );
        Ok(())
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
            PrintStyledContent(format!("How much {} do you", good).with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            PrintStyledContent(format!("You have ({})", current_amount).with(Color::White)),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
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
            PrintStyledContent("Which do you want to stash?".with(Color::White)),
            GoodOptions(offset_x, offset_y + 1),
        );
        Ok(())
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
            PrintStyledContent(format!("How much {} do you", good).with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 2),
            PrintStyledContent(format!("There are ({})", current_amount).with(Color::White)),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
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
            PrintStyledContent("Which do you want to withdraw?".with(Color::White)),
            GoodOptions(OFFSET_X, OFFSET_Y + 1),
        );
        Ok(())
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
            PrintStyledContent("How much debt do you".with(Color::White)),
            MoveTo(OFFSET_X, OFFSET_Y + 1),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(OFFSET_X + prompt_len, OFFSET_Y + 1),
            Show
        );
        Ok(())
    }
}

pub struct CheapGoodDialog<'a>(pub &'a Good);

impl<'a> Command for CheapGoodDialog<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let good = self.0;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            PrintStyledContent(format!("Cheap {} here!", good).with(Color::White)),
        );
        Ok(())
    }
}

pub struct ExpensiveGoodDialog<'a>(pub &'a Good);

impl<'a> Command for ExpensiveGoodDialog<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let good = self.0;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            PrintStyledContent(format!("Expensive {} here!", good).with(Color::White)),
        );
        Ok(())
    }
}

pub struct FindGoodsDialog<'a>(pub &'a Good, pub &'a u32, pub &'a GameState);

impl<'a> Command for FindGoodsDialog<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let good = self.0;
        let amount = self.1;
        let state = self.2;
        const OFFSET_X: u16 = PROMPT_OFFSET_X;
        const OFFSET_Y: u16 = PROMPT_OFFSET_Y;
        comp!(
            f,
            MoveTo(OFFSET_X, OFFSET_Y),
            PrintStyledContent(
                format!("You randomly find {} {}!", amount, good).with(Color::White)
            ),
        );
        let remaining_hold = state.remaining_hold();
        if &remaining_hold < amount {
            comp!(
                f,
                MoveTo(OFFSET_X, OFFSET_Y + 1),
                PrintStyledContent(
                    format!("You have space for ({})", remaining_hold).with(Color::White)
                ),
            )
        }
        Ok(())
    }
}

pub struct GoodsStolenDialog(pub GoodsStolenResult);

impl Command for GoodsStolenDialog {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self.0 {
            GoodsStolenResult::NothingStolen => comp!(
                f,
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
                PrintStyledContent("Thieves were on the prowl, but they".with(Color::White)),
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
                PrintStyledContent("couldn't find anything to steal".with(Color::White)),
            ),
            GoodsStolenResult::WasStolen { good, amount } => comp!(
                f,
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y),
                PrintStyledContent("Prowling harbor thieves stole".with(Color::White)),
                MoveTo(PROMPT_OFFSET_X, PROMPT_OFFSET_Y + 1),
                PrintStyledContent(format!("{} {} from you!", amount, good).with(Color::White)),
            ),
        }
        Ok(())
    }
}
