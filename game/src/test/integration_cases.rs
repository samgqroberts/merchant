use pretty_assertions::assert_eq;

use crate::{
    engine::UpdateResult,
    state::{
        GameState, Good, LocationEvent, Mode, NoEffectEvent, PirateEncounterInfo, Transaction,
    },
    test::{
        rng::{default_location_info, MockRng},
        test_engine::TestEngine,
    },
};

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
|                           __  __               _                 _                              |
|                          |  \/  |             | |               | |                             |
|                          | \  / | ___ _ __ ___| |__   __ _ _ __ | |_                            |
|                          | |\/| |/ _ \ '__/ __| '_ \ / _` | '_ \| __|                           |
|                          | |  | |  __/ | | (__| | | | (_| | | | | |_                            |
|                          |_|  |_|\___|_|  \___|_| |_|\__,_|_| |_|\__|                           |
|                                                                                                 |
|                                                                                                 |
|                            A tribute to Drug Wars by samgqroberts                               |
|                                                                                                 |
|                                     www.samgqroberts.com                                        |
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
|                                    Press any key to begin                                       |
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
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|     _____[LLL]______[LLL]____                                     |                             |
|    /     [LLL]      [LLL]    \                        |          )_)                            |
|   /___________________________\                      )_)        )___)         |                 |
|    )=========================(                      )___)       )____)       )_)\               |
|    '|I .--. I     Tea:    0 I|                      )____)     /)_____)      )__)\              |
|     |I | +| I  Coffee:    0 I|                     )_____)    /)______)\    )___) \             |
|     |I_|_+|_I   Sugar:    0 I|                    )______)  //)_______) \\ )_____) \\           |
|    /_I______I Tobacco:    0 I_\             _____//___|___///_____|______\\\__|_____\\\__=====  |
|     )========     Rum:    0 =(              \      Tea:    0 Coffee:    0  Sugar:    0  /       |
|     |I .--. I  Cotton:    0 I|               \ Tobacco:    0    Rum:    0 Cotton:    0 /        |
|     |I |<>| I               I|                \                                       /____     |
|     |I |~ | I Bank:       0 I|       --------- \ Gold:     500 Hold:  100 Cannons: 1 //.../---  |
|     |I |  | I Debt:    1500 I|          ^^^^^ ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^  ^^^/.../      |
|     |I_|__|_I_______________I|                ^^^^      ^^^    ^^^^^^^^^    ^^^^^  /..../       |
|   ###(______)##################                        ^^^      ^^^^             /...../        |
|    ##(________)   ~"^"^~   ##                                                  /....../         |
|======(_________)========================<------------->======================/......../=========|
|      (__________)                       |   London    |                    /........./          |
|                                         <------------->                                         |
|                                                                                                 |
|         (1) Buy                                    Captain, the prices of goods here are:       |
|         (2) Sell                                                  Tea:    6                     |
|         (3) Sail                                               Coffee:    5                     |
|         (4) Stash deposit                                       Sugar:    4                     |
|         (5) Stash withdraw                                    Tobacco:    3                     |
|         (6) Bank deposit                                          Rum:    2                     |
|         (7) Bank withdraw                                      Cotton:    1                     |
|         (8) Pay down debt                                                                       |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    Ok(())
}

#[test]
fn end_game_positive() -> UpdateResult<()> {
    let e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 40000;
        state.debt = 100;
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
|                                                                                                 |
|                                        Congratulations!                                         |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                             After three years, you went from being                              |
|                                                                                                 |
|                                        1400 gold in debt                                        |
|                                                                                                 |
|                                            to having                                            |
|                                                                                                 |
|                                           39900 gold                                            |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
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
        state.initialize();
        state.gold = 100;
        state.debt = 40000;
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
|                                                                                                 |
|                                        Congratulations!                                         |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                             After three years, you went from being                              |
|                                                                                                 |
|                                        1400 gold in debt                                        |
|                                                                                                 |
|                                            to being                                             |
|                                                                                                 |
|                                       39900 gold in debt                                        |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
",
        )
    );
    Ok(())
}

#[test]
fn buy_good() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 1400;
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

#[test]
fn buy_good_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::Buying(Some(Transaction {
            good: Good::Cotton,
            amount: None,
        }));
        state
    })?;
    assert!(e.expect("How much Cotton do you want?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("Which do you want to buy?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn sell_good() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 1400;
        state.inventory.cotton = 15;
        state.locations.london.prices.cotton = 30;
        state
    })?;
    assert!(e.expect("Gold:    1400"));
    assert!(e.expect("Cotton:   15"));
    assert!(e.expect("Cotton:   30"));
    assert!(e.expect("(2) Sell"));
    e.charpress('2')?;
    assert!(e.expect("Which do you want to sell?"));
    assert!(e.expect("(6) Cotton"));
    e.charpress('6')?;
    assert!(e.expect("How much Cotton do you"));
    assert!(e.expect("want to sell?"));
    assert!(e.expect("You have (15)"));
    e.charpress('1')?;
    assert!(e.expect("How much Cotton do you"));
    assert!(e.expect("want to sell? 1"));
    e.charpress('0')?;
    assert!(e.expect("How much Cotton do you"));
    assert!(e.expect("want to sell? 10"));
    e.enterpress()?;
    assert!(e.expect("Cotton:    5"));
    assert!(e.expect("Gold:    1700"));
    Ok(())
}

#[test]
fn sell_good_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::Selling(Some(Transaction {
            good: Good::Cotton,
            amount: None,
        }));
        state
    })?;
    assert!(e.expect("How much Cotton do you"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("Which do you want to sell?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn sail() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                // need add'l one that we generate when we sail
                .push_location_info(default_location_info())
                .into(),
        );
        state.initialize();
        state
    })?;
    assert!(e.expect("|   London    |"));
    assert!(e.expect("Debt:    1500"));
    assert!(e.expect("(3) Sail"));
    e.charpress('3')?;
    assert!(e.expect("Where do you want to sail?"));
    assert!(e.expect("(6) Venice"));
    e.charpress('6')?;
    assert!(e.expect("|   Venice    |"));
    assert!(e.expect("Debt:    1650"));
    Ok(())
}

#[test]
fn sail_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::Sailing;
        state
    })?;
    assert!(e.expect("Where do you want to sail?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn stash_deposit() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.stash.rum = 5;
        state.inventory.rum = 20;
        state
    })?;
    assert!(e.expect("Rum:    5"));
    assert!(e.expect("Rum:   20"));
    assert!(e.expect("(4) Stash deposit"));
    e.charpress('4')?;
    assert!(e.expect("Which do you want to stash?"));
    assert!(e.expect("(5) Rum"));
    e.charpress('5')?;
    assert!(e.expect("How much Rum do you"));
    assert!(e.expect("want to stash?"));
    assert!(e.expect("You have (20)"));
    e.charpress('1')?;
    assert!(e.expect("How much Rum do you"));
    assert!(e.expect("want to stash? 1"));
    e.charpress('2')?;
    assert!(e.expect("How much Rum do you"));
    assert!(e.expect("want to stash? 12"));
    e.enterpress()?;
    assert!(e.expect("Rum:   17"));
    assert!(e.expect("Rum:    8"));
    Ok(())
}

#[test]
fn stash_deposit_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::StashDeposit(Some(Transaction {
            good: Good::Coffee,
            amount: None,
        }));
        state
    })?;
    assert!(e.expect("How much Coffee do you"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("Which do you want to stash?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn stash_withdraw() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.stash.tea = 30;
        state.inventory.tea = 14;
        state
    })?;
    assert!(e.expect("Tea:   30"));
    assert!(e.expect("Tea:   14"));
    assert!(e.expect("(5) Stash withdraw"));
    e.charpress('5')?;
    assert!(e.expect("Which do you want to withdraw?"));
    assert!(e.expect("(1) Tea"));
    e.charpress('1')?;
    assert!(e.expect("How much Tea do you"));
    assert!(e.expect("want to withdraw?"));
    assert!(e.expect("There are (30)"));
    e.charpress('1')?;
    assert!(e.expect("How much Tea do you"));
    assert!(e.expect("want to withdraw? 1"));
    e.charpress('2')?;
    assert!(e.expect("How much Tea do you"));
    assert!(e.expect("want to withdraw? 12"));
    e.enterpress()?;
    assert!(e.expect("Tea:   18"));
    assert!(e.expect("Tea:   26"));
    Ok(())
}

#[test]
fn stash_withdraw_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::StashWithdraw(Some(Transaction {
            good: Good::Rum,
            amount: None,
        }));
        state
    })?;
    assert!(e.expect("How much Rum do you"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("Which do you want to withdraw?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn pay_debt() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.debt = 500;
        state.gold = 1000;
        state
    })?;
    assert!(e.expect("Gold:    1000"));
    assert!(e.expect("Debt:     500"));
    assert!(e.expect("(8) Pay down debt"));
    e.charpress('8')?;
    assert!(e.expect("How much debt do you"));
    assert!(e.expect("want to pay down?"));
    e.charpress('3')?;
    assert!(e.expect("How much debt do you"));
    assert!(e.expect("want to pay down? 3"));
    e.charpress('0')?;
    assert!(e.expect("How much debt do you"));
    assert!(e.expect("want to pay down? 30"));
    e.charpress('0')?;
    assert!(e.expect("How much debt do you"));
    assert!(e.expect("want to pay down? 300"));
    e.enterpress()?;
    assert!(e.expect("Gold:     700"));
    assert!(e.expect("Debt:     200"));
    assert!(e.expect("(8) Pay down debt"));
    Ok(())
}

#[test]
fn pay_debt_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.debt = 500;
        state.mode = Mode::PayDebt(None);
        state
    })?;
    assert!(e.expect("How much debt do you"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn pay_debt_no_debt_left() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.debt = 0;
        state
    })?;
    assert!(e.expect("Debt:       0"));
    // does not show pay down debt option
    assert!(e.nexpect("Pay down debt"));
    // no effect when pressing the pay down debt option key
    let before = e.get_current_formatted();
    e.charpress('8')?;
    assert_eq!(before, e.expect_full(&before));
    Ok(())
}

#[test]
fn bank_deposit() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 1000;
        state.bank = 500;
        state
    })?;
    assert!(e.expect("Gold:    1000"));
    assert!(e.expect("Bank:     500"));
    assert!(e.expect("(6) Bank deposit"));
    e.charpress('6')?;
    assert!(e.expect("How much gold do you want"));
    assert!(e.expect("to deposit in the bank?"));
    e.charpress('3')?;
    assert!(e.expect("How much gold do you want"));
    assert!(e.expect("to deposit in the bank? 3"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want"));
    assert!(e.expect("to deposit in the bank? 30"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want"));
    assert!(e.expect("to deposit in the bank? 300"));
    e.enterpress()?;
    assert!(e.expect("Gold:     700"));
    assert!(e.expect("Bank:     800"));
    Ok(())
}

#[test]
fn bank_deposit_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::BankDeposit(None);
        state
    })?;
    assert!(e.expect("How much gold do you want"));
    assert!(e.expect("to deposit in the bank?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn bank_withdraw() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 1000;
        state.bank = 500;
        state
    })?;
    assert!(e.expect("Gold:    1000"));
    assert!(e.expect("Bank:     500"));
    assert!(e.expect("(7) Bank withdraw"));
    e.charpress('7')?;
    assert!(e.expect("How much gold do you"));
    assert!(e.expect("want to withdraw?"));
    e.charpress('3')?;
    assert!(e.expect("How much gold do you"));
    assert!(e.expect("want to withdraw? 3"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you"));
    assert!(e.expect("want to withdraw? 30"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you"));
    assert!(e.expect("want to withdraw? 300"));
    e.enterpress()?;
    assert!(e.expect("Gold:    1300"));
    assert!(e.expect("Bank:     200"));
    Ok(())
}

#[test]
fn bank_withdraw_back() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::BankWithdraw(None);
        state
    })?;
    assert!(e.expect("How much gold do you"));
    assert!(e.expect("want to withdraw?"));
    assert!(e.expect("(b) <- back"));
    e.charpress('b')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn arrive_at_cheap_good_event() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::GameEvent(LocationEvent::CheapGood(Good::Coffee));
        state
    })?;
    assert!(e.expect("Cheap Coffee here!"));
    e.charpress('a')?;
    assert!(e.expect("Captain, the prices of goods here are:"));
    Ok(())
}

#[test]
fn arrive_at_expensive_good_event() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::GameEvent(LocationEvent::ExpensiveGood(Good::Coffee));
        state
    })?;
    assert!(e.expect("Expensive Coffee here!"));
    e.charpress('a')?;
    assert!(e.expect("Captain, the prices of goods here are:"));
    Ok(())
}

#[test]
fn arrive_at_find_goods_event() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.inventory.coffee = 4;
        state.mode = Mode::GameEvent(LocationEvent::FindGoods(Good::Coffee, 10));
        state
    })?;
    assert!(e.expect("You randomly find 10 Coffee!"));
    e.charpress('a')?;
    assert!(e.expect("Coffee:   14"));
    Ok(())
}

#[test]
fn arrive_at_find_goods_event_not_enough_hold() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.inventory.coffee = 4;
        state.hold_size = 11;
        state.mode = Mode::GameEvent(LocationEvent::FindGoods(Good::Coffee, 10));
        state
    })?;
    assert!(e.expect("You randomly find 10 Coffee!"));
    assert!(e.expect("You have space for (7)"));
    e.charpress('a')?;
    assert!(e.expect("Coffee:   11"));
    Ok(())
}

#[test]
fn arrive_at_stolen_goods_event_some_stolen() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_good_stolen((Good::Coffee, 4))
                .into(),
        );
        state.initialize();
        state.inventory.coffee = 10;
        state.mode = Mode::GameEvent(LocationEvent::GoodsStolen(None));
        state
    })?;
    assert!(e.expect("Prowling harbor thieves stole"));
    assert!(e.expect("4 Coffee from you!"));
    assert!(e.expect("Coffee:   10"));
    e.charpress('a')?;
    assert!(e.expect("Coffee:    6"));
    Ok(())
}

#[test]
fn arrive_at_stolen_goods_event_nothing_stolen() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.inventory.coffee = 0;
        state.mode = Mode::GameEvent(LocationEvent::GoodsStolen(None));
        state
    })?;
    assert!(e.expect("Thieves were on the prowl, but they"));
    assert!(e.expect("couldn't find anything to steal"));
    e.charpress('a')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn arrive_at_can_buy_cannon_accept() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 5100;
        state.cannons = 3;
        state.mode = Mode::GameEvent(LocationEvent::CanBuyCannon);
        state
    })?;
    assert!(e.expect("Gold:    5100"));
    assert!(e.expect("Cannons: 3"));
    assert!(e.expect("An enterprising gentleman on the docks"));
    assert!(e.expect("offers to outfit your ship with an"));
    assert!(e.expect("additional cannon for 5000 gold."));
    assert!(e.expect("Accept? y/n"));
    e.charpress('e')?; // no effect
    assert!(e.expect("Accept? y/n"));
    e.charpress('y')?;
    assert!(e.expect("(1) Buy"));
    assert!(e.expect("Gold:     100"));
    assert!(e.expect("Cannons: 4"));
    Ok(())
}

#[test]
fn arrive_at_can_buy_cannon_not_enough_gold() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 4999;
        state.cannons = 3;
        state.mode = Mode::GameEvent(LocationEvent::CanBuyCannon);
        state
    })?;
    assert!(e.expect("Gold:    4999"));
    assert!(e.expect("Cannons: 3"));
    assert!(e.expect("Accept? y/n"));
    e.charpress('y')?; // no effect
    assert!(e.expect("Accept? y/n"));
    Ok(())
}

#[test]
fn arrive_at_can_buy_cannon_refuse() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 5100;
        state.cannons = 3;
        state.mode = Mode::GameEvent(LocationEvent::CanBuyCannon);
        state
    })?;
    assert!(e.expect("Gold:    5100"));
    assert!(e.expect("Cannons: 3"));
    assert!(e.expect("Accept? y/n"));
    e.charpress('n')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn pirate_encounter_initial() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_num_pirates_encountered(4)
                .into(),
        );
        state.initialize();
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::Initial,
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                            Pirates have found you on the open seas!                             |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                         (press any key)                                         |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 4, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   Will you (r)un or (f)ight ?                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    Ok(())
}

#[test]
fn pirate_encounter_run_success() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_run_success(true)
                .into(),
        );
        state.initialize();
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::Prompt {
                info: PirateEncounterInfo::new(2),
            },
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   Will you (r)un or (f)ight ?                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('r')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                             You've successfully evaded the pirates!                             |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("Captain, the prices of goods here are:"));
    Ok(())
}

#[test]
fn pirate_encounter_run_failure() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_run_success(false)
                .push_damage_from_pirates(1)
                .into(),
        );
        state.initialize();
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::Prompt {
                info: PirateEncounterInfo::new(2),
            },
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   Will you (r)un or (f)ight ?                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('r')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                          The pirates manouver to cut off your escape!                           |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("The pirates fire their cannons at you"));
    Ok(())
}

#[test]
fn pirate_encounter_pirates_attack_not_destroyed() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::PiratesAttack {
                info: PirateEncounterInfo::new(2),
                damage_this_attack: 2,
            },
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                     The pirates fire their cannons at you, doing 2 damage!                      |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 3, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   Will you (r)un or (f)ight ?                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    Ok(())
}

#[test]
fn pirate_encounter_pirates_attack_is_destroyed() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.inventory.tea = 10;
        state.gold = 500;
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::PiratesAttack {
                info: PirateEncounterInfo {
                    health: 2,
                    cur_pirates: 2,
                    total_pirates: 2,
                },
                damage_this_attack: 3,
            },
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 2, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                     The pirates fire their cannons at you, doing 3 damage!                      |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 The pirates have conquered you!                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                    They get away with half your gold and all of your goods!                     |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("Tea:    0"));
    assert!(e.expect("Gold:     250"));
    Ok(())
}

#[test]
fn pirate_encounter_fight_did_sink_pirate_and_did_not_win() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_did_kill_a_pirate(true)
                .push_damage_from_pirates(1)
                .into(),
        );
        state.initialize();
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::Prompt {
                info: PirateEncounterInfo::new(2),
            },
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   Will you (r)un or (f)ight ?                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('f')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 2, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                 You fire your cannons at the pirates, and you sink one of them!                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("The pirates fire their cannons at you"));
    Ok(())
}

#[test]
fn pirate_encounter_fight_did_sink_pirate_and_did_win() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_did_kill_a_pirate(true)
                .push_gold_recovered_from_pirate_encounter(42)
                .into(),
        );
        state.initialize();
        state.gold = 500;
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::Prompt {
                info: PirateEncounterInfo::new(1),
            },
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 1, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   Will you (r)un or (f)ight ?                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('f')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 1, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                 You fire your cannons at the pirates, and you sink one of them!                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                You have sank all of the pirates!                                |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                             You recover 42 gold from the wreckage!                              |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("Captain, the prices of goods here are:"));
    assert!(e.expect("Gold:     542"));
    Ok(())
}

#[test]
fn pirate_encounter_fight_did_not_sink_pirate() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(
            MockRng::new_with_default_locations()
                .push_did_kill_a_pirate(false)
                .push_damage_from_pirates(1)
                .into(),
        );
        state.initialize();
        state.gold = 500;
        state.cannons = 2;
        state.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
            crate::state::PirateEncounterState::Prompt {
                info: PirateEncounterInfo::new(1),
            },
        ));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 1, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   Will you (r)un or (f)ight ?                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('f')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                 Health 5, Pirates 1, Cannons 2.                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                  You fire your cannons at the pirates, but you only hit water!                  |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                   (press any key to continue)                                   |
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
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("The pirates fire their cannons at you"));
    Ok(())
}

#[test]
fn can_buy_hold_space_accept() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 500;
        state.hold_size = 200;
        state.mode = Mode::GameEvent(LocationEvent::CanBuyHoldSpace {
            price: 410,
            more_hold: 95,
        });
        state
    })?;
    assert!(e.expect("An earnest youth on the docks"));
    assert!(e.expect("offers to clear out the unusable"));
    assert!(e.expect("space in your hold."));
    assert!(e.expect("Pay him 410 gold for 95 more hold space?"));
    assert!(e.expect("(y/n)"));
    e.charpress('y')?;
    assert!(e.expect("Hold:  295"));
    assert!(e.expect("Gold:      90"));
    Ok(())
}

#[test]
fn can_buy_hold_space_refuse() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 500;
        state.hold_size = 200;
        state.mode = Mode::GameEvent(LocationEvent::CanBuyHoldSpace {
            price: 410,
            more_hold: 95,
        });
        state
    })?;
    assert!(e.expect("An earnest youth on the docks"));
    assert!(e.expect("offers to clear out the unusable"));
    assert!(e.expect("space in your hold."));
    assert!(e.expect("Pay him 410 gold for 95 more hold space?"));
    assert!(e.expect("(y/n)"));
    e.charpress('n')?; // no effect
    assert!(e.expect("(1) Buy"));
    assert!(e.expect("Hold:  200"));
    assert!(e.expect("Gold:     500"));
    Ok(())
}

#[test]
fn can_buy_hold_space_not_enough_gold() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.gold = 300;
        state.hold_size = 200;
        state.mode = Mode::GameEvent(LocationEvent::CanBuyHoldSpace {
            price: 410,
            more_hold: 95,
        });
        state
    })?;
    assert!(e.expect("An earnest youth on the docks"));
    assert!(e.expect("offers to clear out the unusable"));
    assert!(e.expect("space in your hold."));
    assert!(e.expect("Pay him 410 gold for 95 more hold space?"));
    assert!(e.expect("(y/n)"));
    e.charpress('y')?; // no effect
    assert!(e.expect("An earnest youth on the docks"));
    Ok(())
}

#[test]
fn no_effect_sunny_day() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::GameEvent(LocationEvent::NoEffect(NoEffectEvent::SunnyDay));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|     _____[LLL]______[LLL]____                                     |                             |
|    /     [LLL]      [LLL]    \                        |          )_)                            |
|   /___________________________\                      )_)        )___)         |                 |
|    )=========================(                      )___)       )____)       )_)\               |
|    '|I .--. I     Tea:    0 I|                      )____)     /)_____)      )__)\              |
|     |I | +| I  Coffee:    0 I|                     )_____)    /)______)\    )___) \             |
|     |I_|_+|_I   Sugar:    0 I|                    )______)  //)_______) \\ )_____) \\           |
|    /_I______I Tobacco:    0 I_\             _____//___|___///_____|______\\\__|_____\\\__=====  |
|     )========     Rum:    0 =(              \      Tea:    0 Coffee:    0  Sugar:    0  /       |
|     |I .--. I  Cotton:    0 I|               \ Tobacco:    0    Rum:    0 Cotton:    0 /        |
|     |I |<>| I               I|                \                                       /____     |
|     |I |~ | I Bank:       0 I|       --------- \ Gold:     500 Hold:  100 Cannons: 1 //.../---  |
|     |I |  | I Debt:    1500 I|          ^^^^^ ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^  ^^^/.../      |
|     |I_|__|_I_______________I|                ^^^^      ^^^    ^^^^^^^^^    ^^^^^  /..../       |
|   ###(______)##################                        ^^^      ^^^^             /...../        |
|    ##(________)   ~"^"^~   ##                                                  /....../         |
|======(_________)========================<------------->======================/......../=========|
|      (__________)                       |   London    |                    /........./          |
|                                         <------------->                                         |
|                                                                                                 |
|         As your ship glides into the harbor        Captain, the prices of goods here are:       |
|         you take a moment and feel the                            Tea:    6                     |
|         comforting warmth of the sun                           Coffee:    5                     |
|         on your face.                                           Sugar:    4                     |
|                                                               Tobacco:    3                     |
|         (press any key to continue)                               Rum:    2                     |
|                                                                Cotton:    1                     |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}

#[test]
fn no_effect_storm() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(MockRng::new_with_default_locations().into());
        state.initialize();
        state.mode = Mode::GameEvent(LocationEvent::NoEffect(NoEffectEvent::StormOnHorizon));
        state
    })?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r###"
----------------------------------------|=================|----------------------------------------
|                                       | March      1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|     _____[LLL]______[LLL]____                                     |                             |
|    /     [LLL]      [LLL]    \                        |          )_)                            |
|   /___________________________\                      )_)        )___)         |                 |
|    )=========================(                      )___)       )____)       )_)\               |
|    '|I .--. I     Tea:    0 I|                      )____)     /)_____)      )__)\              |
|     |I | +| I  Coffee:    0 I|                     )_____)    /)______)\    )___) \             |
|     |I_|_+|_I   Sugar:    0 I|                    )______)  //)_______) \\ )_____) \\           |
|    /_I______I Tobacco:    0 I_\             _____//___|___///_____|______\\\__|_____\\\__=====  |
|     )========     Rum:    0 =(              \      Tea:    0 Coffee:    0  Sugar:    0  /       |
|     |I .--. I  Cotton:    0 I|               \ Tobacco:    0    Rum:    0 Cotton:    0 /        |
|     |I |<>| I               I|                \                                       /____     |
|     |I |~ | I Bank:       0 I|       --------- \ Gold:     500 Hold:  100 Cannons: 1 //.../---  |
|     |I |  | I Debt:    1500 I|          ^^^^^ ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^  ^^^/.../      |
|     |I_|__|_I_______________I|                ^^^^      ^^^    ^^^^^^^^^    ^^^^^  /..../       |
|   ###(______)##################                        ^^^      ^^^^             /...../        |
|    ##(________)   ~"^"^~   ##                                                  /....../         |
|======(_________)========================<------------->======================/......../=========|
|      (__________)                       |   London    |                    /........./          |
|                                         <------------->                                         |
|                                                                                                 |
|         You stand on the docks and look            Captain, the prices of goods here are:       |
|         off to the horizon.                                       Tea:    6                     |
|         You see an ominous storm forming.                      Coffee:    5                     |
|                                                                 Sugar:    4                     |
|                                                               Tobacco:    3                     |
|         (press any key to continue)                               Rum:    2                     |
|                                                                Cotton:    1                     |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------
"###,
        )
    );
    e.charpress('x')?;
    assert!(e.expect("(1) Buy"));
    Ok(())
}
