use std::num::Saturating;

use pretty_assertions::assert_eq;

use crate::test::test_engine::TestEngine;
use merchant_core::engine::UpdateResult;
use merchant_core::state::GameState;
use merchant_core::test::rng::{default_location_config, default_location_infos, MockRng};

#[test]
fn splash_screen_into_inventory() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        GameState::new(MockRng::new_with_default_locations().into())
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r"
---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                            __  __               _                 _                             |
|                           |  \/  |             | |               | |                            |
|                           | \  / | ___ _ __ ___| |__   __ _ _ __ | |_                           |
|                           | |\/| |/ _ \ '__/ __| '_ \ / _` | '_ \| __|                          |
|                           | |  | |  __/ | | (__| | | | (_| | | | | |_                           |
|                           |_|  |_|\___|_|  \___|_| |_|\__,_|_| |_|\__|                          |
|                                                                                                 |
|                                                                                                 |
|                              A tribute to Drug Wars by samgqroberts                             |
|                                                                                                 |
|                                       www.samgqroberts.com                                      |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                      Press any key to begin                                     |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
",
        )
    );
    e.charpress('a')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
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
"###,
        )
    );
    e.charpress('a')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~|=================|~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.
)                                       | March      1782 |                                       (
|                                       |=================|                                       |
(     _____[LLL]______[LLL]____                                     |                             )
|    /     [LLL]      [LLL]    \                        |          )_)                            |
)   /___________________________\                      )_)        )___)         |                 (
|    )=========================(                      )___)       )____)       )_)\               |
(    '|I .--. I     Tea:    0 I|                      )____)     /)_____)      )__)\              )
|     |I | +| I  Coffee:    0 I|                     )_____)    /)______)\    )___) \             |
)     |I_|_+|_I   Sugar:    0 I|                    )______)  //)_______) \\ )_____) \\           (
|    /_I______I Tobacco:    0 I_\             _____//___|___///_____|______\\\__|_____\\\__=====  |
(     )========     Rum:    0 =(              \      Tea:    0 Coffee:    0  Sugar:    0  /       )
|     |I .--. I  Cotton:    0 I|               \ Tobacco:    0    Rum:    0 Cotton:    0 /        |
)     |I |<>| I               I|                \                                       /____     (
|     |I |~ | I Bank:       0 I|       --------- \ Gold:     500 Hold:  100 Cannons: 1 //.../---  |
(     |I |  | I Debt:    1500 I|          ^^^^^ ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^  ^^^/.../      )
|     |I_|__|_I_______________I|                ^^^^      ^^^    ^^^^^^^^^    ^^^^^  /..../       |
)   ###(______)##################                        ^^^      ^^^^             /...../        (
|    ##(________)   ~"^"^~   ##                                                  /....../         |
('~~.~~(_________)~'~~.~~'~~.~~'~~.~~'~~.~<------------->~.~~'~~.~~'~~.~~'~~.~~/......../~~'~~.~~')
|      (__________)                       |   London    |                    /........./          |
)                                         <------------->                                         (
|                                                                                                 |
(         (1) Buy                                    Captain, the prices of goods here are:       )
|         (2) Sell                                                  Tea:    6                     |
)         (3) Sail                                               Coffee:    5                     (
|         (4) Stash deposit                                       Sugar:    4                     |
(         (5) Stash withdraw                                    Tobacco:    3                     )
|         (6) Bank deposit                                          Rum:    2                     |
)         (7) Bank withdraw                                      Cotton:    1                     (
|         (8) Pay down debt                                                                       |
(                                                                                                 )
.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'~.~'
"###,
        )
    );
    Ok(())
}

#[test]
fn end_game_positive() -> UpdateResult<()> {
    let e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.introduction_to_game();
        state.gold = Saturating(40000);
        state.debt = Saturating(100);
        state.game_end = true;
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r"
---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                        _____                         ____                                       |
|                       / ____|                       / __ \                                      |
|                      | |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __                       |
|                      | | |_ |/ _` | '_ ` _ \ / _ \ | |  | \ \ / / _ \ '__|                      |
|                      | |__| | (_| | | | | | |  __/ | |__| |\ V /  __/ |                         |
|                       \_____|\__,_|_| |_| |_|\___|  \____/  \_/ \___|_|                         |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                              After three years, you went from being                             |
|                                                                                                 |
|                                        1000 gold in debt                                        |
|                                                                                                 |
|                                            to having                                            |
|                                                                                                 |
|                                            39900 gold                                           |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                               Your father expected more from you.                               |
|                                                                                                 |
|                    It will likely be a long time before your father retires.                    |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                      (Enter) to play again                                      |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
",
        )
    );
    Ok(())
}

#[test]
fn end_game_negative() -> UpdateResult<()> {
    let e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.introduction_to_game();
        state.gold = Saturating(100);
        state.debt = Saturating(40000);
        state.game_end = true;
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r"
---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                        _____                         ____                                       |
|                       / ____|                       / __ \                                      |
|                      | |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __                       |
|                      | | |_ |/ _` | '_ ` _ \ / _ \ | |  | \ \ / / _ \ '__|                      |
|                      | |__| | (_| | | | | | |  __/ | |__| |\ V /  __/ |                         |
|                       \_____|\__,_|_| |_| |_|\___|  \____/  \_/ \___|_|                         |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                              After three years, you went from being                             |
|                                                                                                 |
|                                        1000 gold in debt                                        |
|                                                                                                 |
|                                             to being                                            |
|                                                                                                 |
|                                        39900 gold in debt                                       |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                          Obviously, your father is disappointed in you.                         |
|                                                                                                 |
|                            He has made the decision never to retire.                            |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                      (Enter) to play again                                      |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
",
        )
    );
    Ok(())
}

#[test]
fn end_game_restart() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_location_config(default_location_config())
                .push_location_infos(&default_location_infos())
                .into(),
        );
        state.introduction_to_game();
        state.gold = Saturating(100);
        state.debt = Saturating(40000);
        state.game_end = true;
        state
    })?;
    assert!(e.expect("(Enter) to play again"));
    e.enterpress()?;
    assert!(e.expect("Press any key to begin"));
    Ok(())
}

#[test]
fn buy_good_text_input_works() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.introduction_to_game();
        state.gold = Saturating(1400);
        state.inventory.cotton = 15;
        state.locations.london.prices.cotton = 30;
        state
    })?;
    assert!(e.expect("Gold:    1400"));
    assert!(e.expect("Cotton:   15"));
    assert!(e.expect("Cotton:   30"));
    assert!(e.expect("(1) Buy"));
    e.charpress('1')?;
    assert!(e.expect("Which do you want to buy?"));
    assert!(e.expect("(6) Cotton"));
    e.charpress('6')?;
    assert!(e.expect("How much Cotton do you want?"));
    assert!(e.expect("You can afford (46)"));
    e.charpress('1')?;
    assert!(e.expect("How much Cotton do you want? 1"));
    assert!(e.expect("You can afford (46)"));
    e.charpress('0')?;
    assert!(e.expect("How much Cotton do you want? 10"));
    assert!(e.expect("You can afford (46)"));
    e.enterpress()?;
    assert!(e.expect("Cotton:   25"));
    assert!(e.expect("Gold:    1100"));
    Ok(())
}
