use std::io;

use crossterm::event::{KeyCode, KeyEvent};

use crate::state::{BuyInfo, GameState, GoodType, Mode};

pub fn update(event: KeyEvent, game_state: &GameState) -> io::Result<Option<GameState>> {
    // any key event initializes the game if game is not already initialized
    if !game_state.initialized {
        return Ok(Some(game_state.initialize()));
    }
    // other updates depend on what the viewer is seeing currently
    match &game_state.mode {
        Mode::ViewingInventory => {
            if event.code == KeyCode::Char('1') {
                // user is now in buying mode
                let mut new_state = game_state.clone();
                new_state.mode = Mode::Buying(None);
                return Ok(Some(new_state));
            }
        }
        Mode::Buying(buy_info) => {
            if let Some(buy_info) = buy_info {
                // user has chosen a good to buy
                if let KeyCode::Char(c) = event.code {
                    if let Some(digit) = c.to_digit(10) {
                        let mut new_state = game_state.clone();
                        let mut new_buy_info = buy_info.clone();
                        new_buy_info.amount = Some(
                            new_buy_info
                                .amount
                                .map_or(digit, |amount| amount * 10 + digit),
                        );
                        new_state.mode = Mode::Buying(Some(new_buy_info));
                        return Ok(Some(new_state));
                    }
                }
                if event.code == KeyCode::Backspace {
                    let mut new_state = game_state.clone();
                    let mut new_buy_info = buy_info.clone();
                    new_buy_info.amount = new_buy_info.amount.and_then(|amount| {
                        if amount <= 9 {
                            None
                        } else {
                            Some(amount / 10)
                        }
                    });
                    new_state.mode = Mode::Buying(Some(new_buy_info));
                    return Ok(Some(new_state));
                }
                if event.code == KeyCode::Enter {
                    // let amount = buy_info.amount.unwrap_or(0);
                    match buy_info.amount.unwrap_or(0) {
                        0 => {
                            let mut new_state = game_state.clone();
                            new_state.mode = Mode::ViewingInventory;
                            return Ok(Some(new_state));
                        }
                        amount => {
                            let good_price = game_state
                                .prices
                                .location_prices(&game_state.location)
                                .good_amount(&buy_info.good);
                            let can_afford = game_state.gold / good_price;
                            println!("{} {}", good_price, can_afford);
                            if amount > can_afford {
                                // user cannot make this purchase because not enough gold
                            } else {
                                let hold_size = game_state.hold_size;
                                let current_hold = game_state.inventory.total_amount();
                                if current_hold + amount > hold_size {
                                    // user cannot make this purchase because not enough hold space
                                } else {
                                    let mut new_state = game_state.clone();
                                    new_state.inventory =
                                        new_state.inventory.add_good(&buy_info.good, amount);
                                    new_state.gold -= good_price * amount;
                                    new_state.mode = Mode::ViewingInventory;
                                    return Ok(Some(new_state));
                                }
                            }
                        }
                    }
                }
            } else {
                let mut good: Option<GoodType> = None;
                if let KeyCode::Char(c) = event.code {
                    match c {
                        '1' => {
                            good = Some(GoodType::Sugar);
                        }
                        '2' => {
                            good = Some(GoodType::Tobacco);
                        }
                        '3' => {
                            good = Some(GoodType::Tea);
                        }
                        '4' => {
                            good = Some(GoodType::Cotton);
                        }
                        '5' => {
                            good = Some(GoodType::Rum);
                        }
                        '6' => {
                            good = Some(GoodType::Coffee);
                        }
                        _ => (),
                    }
                }
                if let Some(good) = good {
                    let mut new_state = game_state.clone();
                    new_state.mode = Mode::Buying(Some(BuyInfo { good, amount: None }));
                    return Ok(Some(new_state));
                }
            }
        }
        Mode::Selling => todo!(),
        Mode::Sailing => todo!(),
    }
    Ok(None)
}
