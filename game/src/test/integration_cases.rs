use pretty_assertions::assert_eq;
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    engine::UpdateResult,
    state::{GameState, Good, LocationEvent, Mode},
    test::test_engine::TestEngine,
};

#[test]
fn splash_screen_into_inventory() -> UpdateResult<()> {
    let mut e = TestEngine::new()?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r"
Merchant

Navigate shifting markets and unreliable sources.

By samgqroberts

Press any key to begin
",
        )
    );
    e.charpress('a')?;
    assert_eq!(
        e.get_current_formatted(),
        e.expect_full(
            r"
         March 1782             Hold Size 100
         Gold 500                Location London

          Home base                 |     |     |                 
             Tea: 0                )_)   )_)   )_)              
          Coffee: 0               )___) )___) )___)\            
           Sugar: 0              )____))____))_____)\\
         Tobacco: 0            _____|_____|_____|____\\\______
             Rum: 0          \      Tea: 0    Tobacco: 0    /
          Cotton: 0           \  Coffee: 0        Rum: 0   /
                      ---------\  Sugar: 0     Cotton: 0  /---------
            Bank: 0     ^^^^^ ^^^^^^^^^^^^^^^^^^^^^   ^^^
            Debt: 1500    ^^^^      ^^^^     ^^^    ^^
                               ^^^^      ^^^
     Captain, the prices of goods here are:
             Tea: 5626          Tobacco: 102
          Coffee: 2976              Rum: 59
           Sugar: 897            Cotton: 7

         (1) Buy
         (2) Sell
         (3) Sail
         (4) Stash deposit
         (5) Stash withdraw
         (6) Bank deposit
         (7) Bank withdraw
         (8) Pay down debt
",
        )
    );
    Ok(())
}

#[test]
fn end_game_positive() -> UpdateResult<()> {
    let e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
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
Congratulations!!

After three years, you went from being
1400 gold in debt
to having
39900 gold
",
        )
    );
    Ok(())
}

#[test]
fn end_game_negative() -> UpdateResult<()> {
    let e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
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
Congratulations!!

After three years, you went from being
1400 gold in debt
to being
39900 gold in debt
",
        )
    );
    Ok(())
}

#[test]
fn sell_good() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.gold = 1400;
        state.inventory.cotton = 15;
        state.locations.london.prices.cotton = 30;
        state
    })?;
    assert!(e.expect("Gold 1400"));
    assert!(e.expect("Cotton: 15"));
    assert!(e.expect("Cotton: 30"));
    assert!(e.expect("(2) Sell"));
    e.charpress('2')?;
    assert!(e.expect("Which do you want to sell?"));
    assert!(e.expect("(6) Cotton"));
    e.charpress('6')?;
    assert!(e.expect("How much Cotton do you want to sell?"));
    assert!(e.expect("You have (15)"));
    e.charpress('1')?;
    assert!(e.expect("How much Cotton do you want to sell? 1"));
    e.charpress('0')?;
    assert!(e.expect("How much Cotton do you want to sell? 10"));
    e.enterpress()?;
    assert!(e.expect("Cotton: 5"));
    assert!(e.expect("Gold 1700"));
    Ok(())
}

#[test]
fn sail() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state
    })?;
    assert!(e.expect("Location London"));
    assert!(e.expect("Debt: 1500"));
    assert!(e.expect("(3) Sail"));
    e.charpress('3')?;
    assert!(e.expect("Where do you want to sail?"));
    assert!(e.expect("(6) Venice"));
    e.charpress('6')?;
    assert!(e.expect("Location Venice"));
    assert!(e.expect("Debt: 1650"));
    Ok(())
}

#[test]
fn stash_deposit() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.stash.rum = 5;
        state.inventory.rum = 20;
        state
    })?;
    assert!(e.expect("Rum: 5"));
    assert!(e.expect("Rum: 20"));
    assert!(e.expect("(4) Stash deposit"));
    e.charpress('4')?;
    assert!(e.expect("Which do you want to stash?"));
    assert!(e.expect("(5) Rum"));
    e.charpress('5')?;
    assert!(e.expect("How much Rum do you want to stash?"));
    assert!(e.expect("You have (20)"));
    e.charpress('1')?;
    assert!(e.expect("How much Rum do you want to stash? 1"));
    e.charpress('2')?;
    assert!(e.expect("How much Rum do you want to stash? 12"));
    e.enterpress()?;
    assert!(e.expect("Rum: 17"));
    assert!(e.expect("Rum: 8"));
    Ok(())
}

#[test]
fn stash_withdraw() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.stash.tea = 30;
        state.inventory.tea = 14;
        state
    })?;
    assert!(e.expect("Tea: 30"));
    assert!(e.expect("Tea: 14"));
    assert!(e.expect("(5) Stash withdraw"));
    e.charpress('5')?;
    assert!(e.expect("Which do you want to withdraw?"));
    assert!(e.expect("(1) Tea"));
    e.charpress('1')?;
    assert!(e.expect("How much Tea do you want to withdraw?"));
    assert!(e.expect("There are (30)"));
    e.charpress('1')?;
    assert!(e.expect("How much Tea do you want to withdraw? 1"));
    e.charpress('2')?;
    assert!(e.expect("How much Tea do you want to withdraw? 12"));
    e.enterpress()?;
    assert!(e.expect("Tea: 18"));
    assert!(e.expect("Tea: 26"));
    Ok(())
}

#[test]
fn pay_debt() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.debt = 500;
        state.gold = 1000;
        state
    })?;
    assert!(e.expect("Gold 1000"));
    assert!(e.expect("Debt: 500"));
    assert!(e.expect("(8) Pay down debt"));
    e.charpress('8')?;
    assert!(e.expect("How much debt do you want to pay down?"));
    e.charpress('3')?;
    assert!(e.expect("How much debt do you want to pay down? 3"));
    e.charpress('0')?;
    assert!(e.expect("How much debt do you want to pay down? 30"));
    e.charpress('0')?;
    assert!(e.expect("How much debt do you want to pay down? 300"));
    e.enterpress()?;
    assert!(e.expect("Gold 700"));
    assert!(e.expect("Debt: 200"));
    assert!(e.expect("(8) Pay down debt"));
    Ok(())
}

#[test]
fn pay_debt_no_debt_left() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.debt = 0;
        state
    })?;
    assert!(e.expect("Debt: 0"));
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
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.gold = 1000;
        state.bank = 500;
        state
    })?;
    assert!(e.expect("Gold 1000"));
    assert!(e.expect("Bank: 500"));
    assert!(e.expect("(6) Bank deposit"));
    e.charpress('6')?;
    assert!(e.expect("How much gold do you want to deposit in the bank?"));
    e.charpress('3')?;
    assert!(e.expect("How much gold do you want to deposit in the bank? 3"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want to deposit in the bank? 30"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want to deposit in the bank? 300"));
    e.enterpress()?;
    assert!(e.expect("Gold 700"));
    assert!(e.expect("Bank: 800"));
    Ok(())
}

#[test]
fn bank_withdraw() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.gold = 1000;
        state.bank = 500;
        state
    })?;
    assert!(e.expect("Gold 1000"));
    assert!(e.expect("Bank: 500"));
    assert!(e.expect("(7) Bank withdraw"));
    e.charpress('7')?;
    assert!(e.expect("How much gold do you want to withdraw?"));
    e.charpress('3')?;
    assert!(e.expect("How much gold do you want to withdraw? 3"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want to withdraw? 30"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want to withdraw? 300"));
    e.enterpress()?;
    assert!(e.expect("Gold 1300"));
    assert!(e.expect("Bank: 200"));
    Ok(())
}

#[test]
fn arrive_at_cheap_good_event() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
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
        let mut state = GameState::new(StdRng::seed_from_u64(42));
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
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.inventory.coffee = 4;
        state.mode = Mode::GameEvent(LocationEvent::FindGoods(Good::Coffee, 10));
        state
    })?;
    assert!(e.expect("You randomly find 10 Coffee!"));
    e.charpress('a')?;
    assert!(e.expect("Coffee: 14"));
    Ok(())
}

#[test]
fn arrive_at_find_goods_event_not_enough_hold() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state({
        let mut state = GameState::new(StdRng::seed_from_u64(42));
        state.initialize();
        state.inventory.coffee = 4;
        state.hold_size = 11;
        state.mode = Mode::GameEvent(LocationEvent::FindGoods(Good::Coffee, 10));
        state
    })?;
    assert!(e.expect("You randomly find 10 Coffee!"));
    assert!(e.expect("You have space for (7)"));
    e.charpress('a')?;
    assert!(e.expect("Coffee: 11"));
    Ok(())
}
