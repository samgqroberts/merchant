use std::{
    fmt::{self, Display},
    io::{self, stdout, Write},
    time::Duration,
};

use chrono::NaiveDate;
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{style, Attribute, Color, PrintStyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    QueueableCommand,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};

#[derive(Clone)]
enum Location {
    SAVANNAH,
    LONDON,
    LISBON,
    AMSTERDAM,
    CAPETOWN,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Location::SAVANNAH => "Savannah",
            Location::LONDON => "London",
            Location::LISBON => "Lisbon",
            Location::AMSTERDAM => "Amsterdam",
            Location::CAPETOWN => "Cape Town",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}

#[derive(Clone)]
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
            Location::SAVANNAH => &self.savannah,
            Location::LONDON => &self.london,
            Location::LISBON => &self.lisbon,
            Location::AMSTERDAM => &self.amsterdam,
            Location::CAPETOWN => &self.capetown,
        }
    }
}

#[derive(Clone)]
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
            GoodType::SUGAR => self.sugar,
            GoodType::TOBACCO => self.tobacco,
            GoodType::TEA => self.tea,
            GoodType::COTTON => self.cotton,
            GoodType::RUM => self.rum,
            GoodType::COFFEE => self.coffee,
        }
    }
}

#[derive(Clone)]
struct GameState {
    rng: StdRng,
    initialized: bool,
    date: NaiveDate,
    hold_size: u16,
    gold: u32,
    location: Location,
    inventory: Inventory,
    prices: Prices,
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
            location: Location::LONDON,
            inventory: Inventory::new(),
            prices,
        }
    }

    fn initialize(&self) -> GameState {
        let mut game_state = self.clone();
        game_state.initialized = true;
        game_state
    }
}

#[derive(Debug)]
enum GoodType {
    SUGAR,
    TOBACCO,
    TEA,
    COTTON,
    RUM,
    COFFEE,
}

const GOOD_TYPES: &'static [GoodType] = &[
    GoodType::SUGAR,
    GoodType::TOBACCO,
    GoodType::TEA,
    GoodType::COTTON,
    GoodType::RUM,
    GoodType::COFFEE,
];

impl Display for GoodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            GoodType::SUGAR => "Sugar",
            GoodType::TOBACCO => "Tobacco",
            GoodType::TEA => "Tea",
            GoodType::COTTON => "Cotton",
            GoodType::RUM => "Rum",
            GoodType::COFFEE => "Coffee",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}

fn draw_inventory_scene(game_state: &GameState) -> io::Result<()> {
    let mut stdout = stdout();
    queue!(
        stdout,
        // clear terminal
        Clear(crossterm::terminal::ClearType::All),
        // write out new game state
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
        PrintStyledContent(format!("Tobacco: {}", game_state.inventory.sugar).with(Color::White)),
        MoveTo(13, 6),
        PrintStyledContent(format!("Tea: {}", game_state.inventory.sugar).with(Color::White)),
        MoveTo(10, 7),
        PrintStyledContent(format!("Cotton: {}", game_state.inventory.sugar).with(Color::White)),
        MoveTo(13, 8),
        PrintStyledContent(format!("Rum: {}", game_state.inventory.sugar).with(Color::White)),
        MoveTo(10, 9),
        PrintStyledContent(format!("Coffee: {}", game_state.inventory.sugar).with(Color::White)),
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
    stdout.flush()?;
    Ok(())
}

fn main_loop(game_state: &GameState) -> io::Result<(bool, Option<GameState>)> {
    // Wait up to 1s for some user event
    if poll(Duration::from_millis(1_000))? {
        // Read what even happened from the poll
        // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
        match read()? {
            Event::Key(event) => {
                // detect exit request
                if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
                    return Ok((true, None));
                }
                // move forward game state
                if !game_state.initialized {
                    // initialize game
                    draw_inventory_scene(game_state)?;
                    return Ok((false, Some(game_state.initialize())));
                }
                // user event had no effect
                Ok((false, None))
            }
            _ => Ok((false, None)),
        }
    } else {
        // Timeout expired, no event for 1s
        Ok((false, None))
    }
}

fn main() -> io::Result<()> {
    // initial splash screen
    execute!(
        io::stdout(),
        // clear terminal
        Clear(crossterm::terminal::ClearType::All),
        // reset cursor position to top left
        MoveTo(0, 0),
        // write out splash screen
        PrintStyledContent(
            "Merchant\n\nNavigate shifting markets and unreliable sources.\n\nBy samgqroberts"
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
    )?;
    // set terminal into "non-canonical" mode so inputs are captured raw with no interpretation
    // https://docs.rs/crossterm/0.26.1/crossterm/terminal/index.html#raw-mode
    enable_raw_mode()?;
    // start main game loop, with fresh game state
    let mut game_state = GameState::new();
    loop {
        // perform main loop logic, detect and handle potential io error
        match main_loop(&mut game_state) {
            Err(e) => {
                // an error was encountered in main game loop
                println!("Error: {:?}\r", e);
            }
            Ok((should_exit, new_game_state)) => {
                // main loop may have told us user requested an exit
                if should_exit {
                    break;
                }
                // main loop may have given back a new game state
                if let Some(new_game_state) = new_game_state {
                    game_state = new_game_state
                }
            }
        }
    }
    // set terminal back to canonical mode before exiting
    disable_raw_mode()
}
