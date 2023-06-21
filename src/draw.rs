use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    execute, queue,
    style::{style, Attribute, Color, Print, PrintStyledContent, Stylize},
    terminal::Clear,
};
use std::io::{self, Write};

use crate::state::{GameState, Mode};

pub struct Drawer<'a, Writer: Write> {
    pub writer: &'a mut Box<Writer>,
}

impl<'a, Writer: Write> Drawer<'a, Writer> {
    pub fn draw_scene(&mut self, game_state: &GameState) -> io::Result<()> {
        let writer = self.writer.by_ref();
        if !game_state.initialized {
            // initial splash screen
            queue!(
                writer,
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
            )?;
        } else {
            queue!(
                writer,
                // clear terminal
                Clear(crossterm::terminal::ClearType::All),
                // date
                MoveTo(9, 0),
                PrintStyledContent(
                    format!("Date {}", game_state.date.to_string()).with(Color::White)
                ),
                // hold size
                MoveTo(32, 0),
                PrintStyledContent(
                    format!("Hold Size {}", game_state.hold_size).with(Color::White)
                ),
                // gold
                MoveTo(9, 1),
                PrintStyledContent(format!("Gold {}", game_state.gold).with(Color::White)),
                // location
                MoveTo(33, 1),
                PrintStyledContent(format!("Location {}", game_state.location).with(Color::White)),
                // inventory
                MoveTo(9, 3),
                PrintStyledContent("Inventory".with(Color::White)),
                MoveTo(11, 4),
                PrintStyledContent(
                    format!("Sugar: {}", game_state.inventory.sugar).with(Color::White)
                ),
                MoveTo(9, 5),
                PrintStyledContent(
                    format!("Tobacco: {}", game_state.inventory.tobacco).with(Color::White)
                ),
                MoveTo(13, 6),
                PrintStyledContent(format!("Tea: {}", game_state.inventory.tea).with(Color::White)),
                MoveTo(10, 7),
                PrintStyledContent(
                    format!("Cotton: {}", game_state.inventory.cotton).with(Color::White)
                ),
                MoveTo(13, 8),
                PrintStyledContent(format!("Rum: {}", game_state.inventory.rum).with(Color::White)),
                MoveTo(10, 9),
                PrintStyledContent(
                    format!("Coffee: {}", game_state.inventory.coffee).with(Color::White)
                ),
                // current prices
                MoveTo(5, 11),
                PrintStyledContent("Captain, the prices of goods here are:".with(Color::White)),
                MoveTo(11, 12),
                PrintStyledContent(
                    format!(
                        "Sugar: {}",
                        game_state
                            .prices
                            .location_prices(&game_state.location)
                            .sugar
                    )
                    .with(Color::White)
                ),
                MoveTo(27, 12),
                PrintStyledContent(
                    format!(
                        "Tobacco: {}",
                        game_state
                            .prices
                            .location_prices(&game_state.location)
                            .tobacco
                    )
                    .with(Color::White)
                ),
                MoveTo(13, 13),
                PrintStyledContent(
                    format!(
                        "Tea: {}",
                        game_state.prices.location_prices(&game_state.location).tea
                    )
                    .with(Color::White)
                ),
                MoveTo(28, 13),
                PrintStyledContent(
                    format!(
                        "Cotton: {}",
                        game_state
                            .prices
                            .location_prices(&game_state.location)
                            .cotton
                    )
                    .with(Color::White)
                ),
                MoveTo(13, 14),
                PrintStyledContent(
                    format!(
                        "Rum: {}",
                        game_state.prices.location_prices(&game_state.location).rum
                    )
                    .with(Color::White)
                ),
                MoveTo(28, 14),
                PrintStyledContent(
                    format!(
                        "Coffee: {}",
                        game_state
                            .prices
                            .location_prices(&game_state.location)
                            .coffee
                    )
                    .with(Color::White)
                ),
            )?;
            match &game_state.mode {
                Mode::ViewingInventory => {
                    queue!(
                        writer,
                        // actions
                        MoveTo(9, 16),
                        PrintStyledContent("(1) Buy".with(Color::White)),
                        MoveTo(9, 17),
                        PrintStyledContent("(2) Sell".with(Color::White)),
                        MoveTo(9, 18),
                        PrintStyledContent("(3) Sail".with(Color::White)),
                    )?;
                }
                Mode::Buying(good) => {
                    if let Some(buy_info) = good {
                        // user has indicated which good they want to buy
                        let good = &buy_info.good;
                        let prompt = format!(
                            "How much {} do you want? {}",
                            good,
                            buy_info
                                .amount
                                .map_or("".to_owned(), |amount| amount.to_string())
                        );
                        let prompt_len: u16 = prompt.len().try_into().unwrap();
                        let good_price = game_state
                            .prices
                            .location_prices(&game_state.location)
                            .good_amount(&good);
                        let can_afford = game_state.gold / good_price;
                        queue!(
                            writer,
                            // prompt what to buy
                            MoveTo(9, 16),
                            PrintStyledContent(prompt.with(Color::White)),
                            MoveTo(9, 17),
                            PrintStyledContent(
                                format!("You can afford ({})", can_afford).with(Color::White)
                            ),
                            MoveTo(9 + prompt_len, 16),
                            Show
                        )?;
                    } else {
                        // user is choosing which good to buy
                        queue!(
                            writer,
                            // prompt what to buy
                            MoveTo(9, 16),
                            PrintStyledContent("Which do you want to buy?".with(Color::White)),
                            MoveTo(9, 17),
                            PrintStyledContent("(1) Sugar".with(Color::White)),
                            MoveTo(9, 18),
                            PrintStyledContent("(2) Tobacco".with(Color::White)),
                            MoveTo(9, 19),
                            PrintStyledContent("(3) Tea".with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent("(4) Cotton".with(Color::White)),
                            MoveTo(9, 21),
                            PrintStyledContent("(5) Rum".with(Color::White)),
                            MoveTo(9, 22),
                            PrintStyledContent("(6) Coffee".with(Color::White)),
                        )?;
                    }
                }
                Mode::Selling => todo!(),
                Mode::Sailing => todo!(),
            }
        }
        self.writer.flush()?;
        Ok(())
    }

    pub fn exit_message(&mut self) -> io::Result<()> {
        let writer = self.writer.by_ref();
        execute!(
            writer,
            Show,
            MoveToNextLine(2),
            Print("Thank you for playing!"),
            MoveToNextLine(1)
        )?;
        Ok(())
    }
}
