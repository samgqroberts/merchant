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
struct Inventory {
    sugar: u32,
    tobacco: u32,
    tea: u32,
    cotton: u32,
    silk: u32,
    coffee: u32,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            sugar: 0,
            tobacco: 0,
            tea: 0,
            cotton: 0,
            silk: 0,
            coffee: 0,
        }
    }

    fn good_amount(&self, good_type: &GoodType) -> u32 {
        match good_type {
            GoodType::SUGAR => self.sugar,
            GoodType::TOBACCO => self.tobacco,
            GoodType::TEA => self.tea,
            GoodType::COTTON => self.cotton,
            GoodType::SILK => self.silk,
            GoodType::COFFEE => self.coffee,
        }
    }
}

#[derive(Clone)]
struct GameState {
    initialized: bool,
    date: NaiveDate,
    hold_size: u16,
    gold: u32,
    location: Location,
    inventory: Inventory,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            initialized: false,
            date: NaiveDate::from_ymd_opt(1782, 3, 1).unwrap(),
            hold_size: 100,
            gold: 1400,
            location: Location::LONDON,
            inventory: Inventory::new(),
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
    SILK,
    COFFEE,
}

const GOOD_TYPES: &'static [GoodType] = &[
    GoodType::SUGAR,
    GoodType::TOBACCO,
    GoodType::TEA,
    GoodType::COTTON,
    GoodType::SILK,
    GoodType::COFFEE,
];

impl Display for GoodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            GoodType::SUGAR => "Sugar",
            GoodType::TOBACCO => "Tobacco",
            GoodType::TEA => "Tea",
            GoodType::COTTON => "Cotton",
            GoodType::SILK => "Silk",
            GoodType::COFFEE => "Coffee",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}

fn print_good_inventory(kind: &GoodType, amount: u32) -> io::Result<()> {
    stdout()
        .queue(PrintStyledContent(
            format!("{}: ", kind)
                .with(Color::White)
                .on(Color::Green)
                .attribute(Attribute::Bold),
        ))?
        .queue(PrintStyledContent(
            amount.to_string().with(Color::White).on(Color::Black),
        ))?;
    Ok(())
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
    )?;
    for (i, good_type) in GOOD_TYPES.iter().enumerate() {
        queue!(stdout, MoveTo(9, (i as u16) + 4))?;
        print_good_inventory(good_type, game_state.inventory.good_amount(good_type))?;
    }
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
