use std::fmt::{self};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    style::{style, Attribute, Color, PrintStyledContent, Stylize},
    terminal::Clear,
    Command,
};

use crate::{
    comp,
    state::{GameState, GoodType, Inventory, Location, Transaction},
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

pub struct BankWithdrawInput<'a>(pub &'a Option<u32>, pub u16, pub u16);

impl<'a> Command for BankWithdrawInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        let prompt = format!(
            "How much gold do you want to withdraw? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
        );
        Ok(())
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
            PrintStyledContent(format!("Tea: {}", inventory.tea).with(Color::White)),
            MoveTo(offset_x + 1, offset_y + 1),
            PrintStyledContent(format!("Coffee: {}", inventory.coffee).with(Color::White)),
            MoveTo(offset_x + 2, offset_y + 2),
            PrintStyledContent(format!("Sugar: {}", inventory.sugar).with(Color::White)),
            MoveTo(offset_x, offset_y + 3),
            PrintStyledContent(format!("Tobacco: {}", inventory.tobacco).with(Color::White)),
            MoveTo(offset_x + 4, offset_y + 4),
            PrintStyledContent(format!("Rum: {}", inventory.rum).with(Color::White)),
            MoveTo(offset_x + 1, offset_y + 5),
            PrintStyledContent(format!("Cotton: {}", inventory.cotton).with(Color::White)),
        );
        Ok(())
    }
}

pub struct CurrentPrices<'a>(pub &'a Inventory, pub u16, pub u16);

impl<'a> Command for CurrentPrices<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let prices = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent("Captain, the prices of goods here are:".with(Color::White)),
            MoveTo(offset_x + 8, offset_y + 1),
            PrintStyledContent(format!("Tea: {}", prices.tea).with(Color::White)),
            MoveTo(offset_x + 5, offset_y + 2),
            PrintStyledContent(format!("Coffee: {}", prices.coffee).with(Color::White)),
            MoveTo(offset_x + 6, offset_y + 3),
            PrintStyledContent(format!("Sugar: {}", prices.sugar).with(Color::White)),
            MoveTo(offset_x + 27, offset_y + 1),
            PrintStyledContent(format!("Tobacco: {}", prices.tobacco).with(Color::White)),
            MoveTo(offset_x + 31, offset_y + 2),
            PrintStyledContent(format!("Rum: {}", prices.rum).with(Color::White)),
            MoveTo(offset_x + 28, offset_y + 3),
            PrintStyledContent(format!("Cotton: {}", prices.cotton).with(Color::White)),
        );
        Ok(())
    }
}

pub struct ViewingInventoryActions<'a>(pub &'a Location, pub u16, pub u16);

impl<'a> Command for ViewingInventoryActions<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let location = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        comp!(
            f,
            // actions
            MoveTo(offset_x, offset_y),
            PrintStyledContent("(1) Buy".with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent("(2) Sell".with(Color::White)),
            MoveTo(offset_x, offset_y + 2),
            PrintStyledContent("(3) Sail".with(Color::White)),
        );
        if location == &Location::London {
            comp!(
                f,
                MoveTo(offset_x, offset_y + 3),
                PrintStyledContent("(4) Stash deposit".with(Color::White)),
                MoveTo(offset_x, offset_y + 4),
                PrintStyledContent("(5) Stash withdraw".with(Color::White)),
                MoveTo(offset_x, offset_y + 5),
                PrintStyledContent("(6) Pay down debt".with(Color::White)),
                MoveTo(offset_x, offset_y + 6),
                PrintStyledContent("(7) Bank deposit".with(Color::White)),
                MoveTo(offset_x, offset_y + 7),
                PrintStyledContent("(8) Bank withdraw".with(Color::White)),
            );
        }
        Ok(())
    }
}

pub struct BankDepositInput<'a>(pub &'a Option<u32>, pub u16, pub u16);

impl<'a> Command for BankDepositInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        let prompt = format!(
            "How much gold do you want to deposit in the bank? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
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
            // clear terminal
            Clear(crossterm::terminal::ClearType::All),
            // hide cursor
            Hide,
            // date
            MoveTo(9, 0),
            PrintStyledContent(
                format!("{} {}", state.date.1.name(), state.date.0.to_string()).with(Color::White)
            ),
            // hold size
            MoveTo(32, 0),
            PrintStyledContent(format!("Hold Size {}", state.hold_size).with(Color::White)),
            // gold
            MoveTo(9, 1),
            PrintStyledContent(format!("Gold {}", state.gold).with(Color::White)),
            // location
            MoveTo(33, 1),
            PrintStyledContent(format!("Location {}", state.location).with(Color::White)),
            // home base
            MoveTo(10, 3),
            PrintStyledContent("Home base".with(Color::White)),
            InventoryList(&state.stash, 9, 4),
            MoveTo(12, 11),
            PrintStyledContent(format!("Bank: {}", state.bank).with(Color::White)),
            MoveTo(12, 12),
            PrintStyledContent(format!("Debt: {}", state.debt).with(Color::White)),
            // inventory
            MoveTo(33, 3),
            PrintStyledContent("Inventory".with(Color::White)),
            InventoryList(&state.inventory, 32, 4),
            // current prices
            CurrentPrices(state.prices.location_prices(&state.location), 5, 14),
        );
        Ok(())
    }
}

pub struct BuyInput<'a>(pub &'a Transaction, pub &'a GameState, pub u16, pub u16);

impl<'a> Command for BuyInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.0;
        let state = self.1;
        let offset_x = self.2;
        let offset_y = self.3;
        // user has indicated which good they want to buy
        let gold = state.gold;
        let good = &info.good;
        let good_price = state
            .prices
            .location_prices(&state.location)
            .good_amount(&good);
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
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent(format!("You can afford ({})", can_afford).with(Color::White)),
        );
        let remaining_hold = state.hold_size - state.inventory.total_amount();
        if remaining_hold < can_afford {
            comp!(
                f,
                MoveTo(offset_x, offset_y + 2),
                PrintStyledContent(
                    format!("You have space for ({})", remaining_hold).with(Color::White)
                ),
            )
        }
        comp!(f, MoveTo(offset_x + prompt_len, offset_y), Show);
        Ok(())
    }
}

pub struct BuyPrompt(pub u16, pub u16);

impl Command for BuyPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let offset_x = self.0;
        let offset_y = self.1;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent("Which do you want to buy?".with(Color::White)),
            GoodOptions(offset_x, offset_y + 1)
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

pub struct SellInput<'a>(pub &'a Transaction, pub u32, pub u16, pub u16);

impl<'a> Command for SellInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.0;
        let current_amount = self.1;
        let offset_x = self.2;
        let offset_y = self.3;
        let good = &info.good;
        let prompt = format!(
            "How much {} do you want to sell? {}",
            good,
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent(format!("You have ({})", current_amount).with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
        );
        Ok(())
    }
}

pub struct SellPrompt(pub u16, pub u16);

impl Command for SellPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let offset_x = self.0;
        let offset_y = self.1;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent("Which do you want to sell?".with(Color::White)),
            GoodOptions(offset_x, offset_y + 1),
        );
        Ok(())
    }
}

pub struct SailPrompt(pub u16, pub u16);

impl Command for SailPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let offset_x = self.0;
        let offset_y = self.1;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent("Where do you want to sail?".with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent("(1) London".with(Color::White)),
            MoveTo(offset_x, offset_y + 2),
            PrintStyledContent("(2) Savannah".with(Color::White)),
            MoveTo(offset_x, offset_y + 3),
            PrintStyledContent("(3) Lisbon".with(Color::White)),
            MoveTo(offset_x, offset_y + 4),
            PrintStyledContent("(4) Amsterdam".with(Color::White)),
            MoveTo(offset_x, offset_y + 5),
            PrintStyledContent("(5) Cape Town".with(Color::White)),
            MoveTo(offset_x, offset_y + 6),
            PrintStyledContent("(6) Venice".with(Color::White)),
        );
        Ok(())
    }
}

pub struct StashDepositInput<'a>(pub &'a Transaction, pub u32, pub u16, pub u16);

impl<'a> Command for StashDepositInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.0;
        let current_amount = self.1;
        let offset_x = self.2;
        let offset_y = self.3;
        // user has indicated which good they want to stash
        let good = &info.good;
        let prompt = format!(
            "How much {} do you want to stash? {}",
            good,
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent(format!("You have ({})", current_amount).with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
        );
        Ok(())
    }
}

pub struct StashDepositPrompt(pub u16, pub u16);

impl Command for StashDepositPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let offset_x = self.0;
        let offset_y = self.1;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent("Which do you want to stash?".with(Color::White)),
            GoodOptions(offset_x, offset_y + 1),
        );
        Ok(())
    }
}

pub struct StashWithdrawInput<'a>(pub &'a Transaction, pub u32, pub u16, pub u16);

impl<'a> Command for StashWithdrawInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let info = self.0;
        let current_amount = self.1;
        let offset_x = self.2;
        let offset_y = self.3;
        // user has indicated which good they want to stash
        let good = &info.good;
        let prompt = format!(
            "How much {} do you want to withdraw? {}",
            good,
            info.amount
                .map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent(format!("There are ({})", current_amount).with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
        );
        Ok(())
    }
}

pub struct StashWithdrawPrompt(pub u16, pub u16);

impl Command for StashWithdrawPrompt {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let offset_x = self.0;
        let offset_y = self.1;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent("Which do you want to withdraw?".with(Color::White)),
            GoodOptions(offset_x, offset_y + 1),
        );
        Ok(())
    }
}

pub struct PayDebtInput<'a>(pub &'a Option<u32>, pub u16, pub u16);

impl<'a> Command for PayDebtInput<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        let prompt = format!(
            "How much debt do you want to pay down? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
        );
        Ok(())
    }
}

pub struct CheapGoodDialog<'a>(pub &'a GoodType, pub u16, pub u16);

impl<'a> Command for CheapGoodDialog<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let good = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(format!("Cheap {} here!", good).with(Color::White)),
        );
        Ok(())
    }
}
