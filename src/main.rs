use std::{
    fmt::{self, Display},
    io::{self, stdout, Write},
    time::Duration,
};

use chrono::NaiveDate;
use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    queue,
    style::{style, Attribute, Color, PrintStyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use rand::{rngs::StdRng, RngCore, SeedableRng};

#[derive(Clone, Debug)]
enum Location {
    Savannah,
    London,
    Lisbon,
    Amsterdam,
    CapeTown,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Location::Savannah => "Savannah",
            Location::London => "London",
            Location::Lisbon => "Lisbon",
            Location::Amsterdam => "Amsterdam",
            Location::CapeTown => "Cape Town",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug)]
struct Prices {
    savannah: Inventory,
    london: Inventory,
    lisbon: Inventory,
    amsterdam: Inventory,
    capetown: Inventory,
}

impl Prices {
    fn new(rng: &mut StdRng) -> Prices {
        Prices {
            savannah: Prices::randomized_inventory(rng),
            london: Prices::randomized_inventory(rng),
            lisbon: Prices::randomized_inventory(rng),
            amsterdam: Prices::randomized_inventory(rng),
            capetown: Prices::randomized_inventory(rng),
        }
    }

    fn randomized_inventory(rng: &mut StdRng) -> Inventory {
        // number between 39 and 111
        let mut gen = || rng.next_u32() % (111 - 39) + 39;
        Inventory {
            sugar: gen(),
            tobacco: gen(),
            tea: gen(),
            cotton: gen(),
            rum: gen(),
            coffee: gen(),
        }
    }

    fn location_prices(&self, location: &Location) -> &Inventory {
        match location {
            Location::Savannah => &self.savannah,
            Location::London => &self.london,
            Location::Lisbon => &self.lisbon,
            Location::Amsterdam => &self.amsterdam,
            Location::CapeTown => &self.capetown,
        }
    }
}

#[derive(Clone, Debug)]
struct Inventory {
    sugar: u32,
    tobacco: u32,
    tea: u32,
    cotton: u32,
    rum: u32,
    coffee: u32,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            sugar: 0,
            tobacco: 0,
            tea: 0,
            cotton: 0,
            rum: 0,
            coffee: 0,
        }
    }

    fn good_amount(&self, good_type: &GoodType) -> u32 {
        match good_type {
            GoodType::Sugar => self.sugar,
            GoodType::Tobacco => self.tobacco,
            GoodType::Tea => self.tea,
            GoodType::Cotton => self.cotton,
            GoodType::Rum => self.rum,
            GoodType::Coffee => self.coffee,
        }
    }

    fn total_amount(&self) -> u32 {
        let mut total: u32 = 0;
        for good in GOOD_TYPES {
            total += self.good_amount(good);
        }
        total
    }

    fn add_good(&self, good: &GoodType, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            GoodType::Sugar => new_inventory.sugar += amount,
            GoodType::Tobacco => new_inventory.tobacco += amount,
            GoodType::Tea => new_inventory.tea += amount,
            GoodType::Cotton => new_inventory.cotton += amount,
            GoodType::Rum => new_inventory.rum += amount,
            GoodType::Coffee => new_inventory.coffee += amount,
        }
        new_inventory
    }
}

#[derive(PartialEq, Clone, Debug)]
struct BuyInfo {
    good: GoodType,
    amount: Option<u32>,
}

#[derive(PartialEq, Clone, Debug)]
enum Mode {
    ViewingInventory,
    Buying(Option<BuyInfo>),
    Selling,
    Sailing,
}

#[derive(Clone, Debug)]
struct GameState {
    rng: StdRng,
    initialized: bool,
    date: NaiveDate,
    hold_size: u32,
    gold: u32,
    location: Location,
    inventory: Inventory,
    prices: Prices,
    mode: Mode,
}

impl GameState {
    fn new() -> GameState {
        let mut rng = StdRng::from_entropy();
        let prices = Prices::new(&mut rng);
        GameState {
            rng,
            initialized: false,
            date: NaiveDate::from_ymd_opt(1782, 3, 1).unwrap(),
            hold_size: 100,
            gold: 1400,
            location: Location::London,
            inventory: Inventory::new(),
            prices,
            mode: Mode::ViewingInventory,
        }
    }

    fn initialize(&self) -> GameState {
        let mut game_state = self.clone();
        game_state.initialized = true;
        game_state
    }
}

#[derive(Debug, PartialEq, Clone)]
enum GoodType {
    Sugar,
    Tobacco,
    Tea,
    Cotton,
    Rum,
    Coffee,
}

const GOOD_TYPES: &'static [GoodType] = &[
    GoodType::Sugar,
    GoodType::Tobacco,
    GoodType::Tea,
    GoodType::Cotton,
    GoodType::Rum,
    GoodType::Coffee,
];

impl Display for GoodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            GoodType::Sugar => "Sugar",
            GoodType::Tobacco => "Tobacco",
            GoodType::Tea => "Tea",
            GoodType::Cotton => "Cotton",
            GoodType::Rum => "Rum",
            GoodType::Coffee => "Coffee",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}

fn draw_scene(game_state: &GameState) -> io::Result<()> {
    let mut stdout = stdout();
    if !game_state.initialized {
        // initial splash screen
        queue!(
            stdout,
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
            stdout,
            // clear terminal
            Clear(crossterm::terminal::ClearType::All),
            // date
            MoveTo(9, 0),
            PrintStyledContent(format!("Date {}", game_state.date.to_string()).with(Color::White)),
            // hold size
            MoveTo(32, 0),
            PrintStyledContent(format!("Hold Size {}", game_state.hold_size).with(Color::White)),
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
            PrintStyledContent(format!("Sugar: {}", game_state.inventory.sugar).with(Color::White)),
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
                    stdout,
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
                        stdout,
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
                        stdout,
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
    stdout.flush()?;
    Ok(())
}

fn main_loop(game_state: &GameState) -> io::Result<(bool, Option<GameState>)> {
    // draw the game state
    draw_scene(game_state)?;
    // Wait for any user event
    loop {
        // Wait up to 1s for some user event per loop iteration
        if poll(Duration::from_millis(1_000))? {
            // Read what even happened from the poll
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            match read()? {
                Event::Key(event) => {
                    // detect exit request
                    if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')
                    {
                        return Ok((true, None));
                    }
                    // move forward game state
                    if !game_state.initialized {
                        // initialize game
                        return Ok((false, Some(game_state.initialize())));
                    } else {
                        match &game_state.mode {
                            Mode::ViewingInventory => {
                                if event.code == KeyCode::Char('1') {
                                    // user is now in buying mode
                                    let mut new_state = game_state.clone();
                                    new_state.mode = Mode::Buying(None);
                                    return Ok((false, Some(new_state)));
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
                                            return Ok((false, Some(new_state)));
                                        }
                                    }
                                    if event.code == KeyCode::Backspace {
                                        let mut new_state = game_state.clone();
                                        let mut new_buy_info = buy_info.clone();
                                        new_buy_info.amount =
                                            new_buy_info.amount.and_then(|amount| {
                                                if amount <= 9 {
                                                    None
                                                } else {
                                                    Some(amount / 10)
                                                }
                                            });
                                        new_state.mode = Mode::Buying(Some(new_buy_info));
                                        return Ok((false, Some(new_state)));
                                    }
                                    if event.code == KeyCode::Enter {
                                        // let amount = buy_info.amount.unwrap_or(0);
                                        match buy_info.amount.unwrap_or(0) {
                                            0 => {
                                                let mut new_state = game_state.clone();
                                                new_state.mode = Mode::ViewingInventory;
                                                return Ok((false, Some(new_state)));
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
                                                    let current_hold =
                                                        game_state.inventory.total_amount();
                                                    if current_hold + amount > hold_size {
                                                        // user cannot make this purchase because not enough hold space
                                                    } else {
                                                        let mut new_state = game_state.clone();
                                                        new_state.inventory = new_state
                                                            .inventory
                                                            .add_good(&buy_info.good, amount);
                                                        new_state.gold -= good_price * amount;
                                                        new_state.mode = Mode::ViewingInventory;
                                                        return Ok((false, Some(new_state)));
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
                                        new_state.mode =
                                            Mode::Buying(Some(BuyInfo { good, amount: None }));
                                        return Ok((false, Some(new_state)));
                                    }
                                }
                            }
                            Mode::Selling => todo!(),
                            Mode::Sailing => todo!(),
                        }
                    }
                    // user event had no effect
                    continue;
                }
                _ => continue,
            }
        } else {
            // Timeout expired, no event for 1s, wait for user input again
            continue;
        }
    }
}

fn main() -> io::Result<()> {
    // set terminal into "non-canonical" mode so inputs are captured raw with no interpretation
    // https://docs.rs/crossterm/0.26.1/crossterm/terminal/index.html#raw-mode
    enable_raw_mode()?;
    // start main game loop, which draws -> reads input -> updates state, with fresh game state
    let mut game_state = GameState::new();
    loop {
        match main_loop(&mut game_state) {
            Err(e) => {
                // an io error was encountered in main game loop
                println!("Error: {:?}\r", e);
            }
            Ok((should_exit, new_game_state)) => {
                if should_exit {
                    // main loop told us user requested an exit
                    break;
                }
                if let Some(new_game_state) = new_game_state {
                    // main loop gave back a new game state
                    game_state = new_game_state
                }
            }
        }
    }
    // set terminal back to canonical mode before exiting
    disable_raw_mode()
}
