use pretty_assertions::assert_eq;
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    engine::UpdateResult,
    state::{GameEvent, GameState, GoodType, Mode},
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

          Home base              Inventory
             Tea: 0                 Tea: 0
          Coffee: 0              Coffee: 0
           Sugar: 0               Sugar: 0
         Tobacco: 0             Tobacco: 0
             Rum: 0                 Rum: 0
          Cotton: 0              Cotton: 0

            Bank: 0
            Debt: 1500

     Captain, the prices of goods here are:
             Tea: 5626          Tobacco: 102
          Coffee: 2976              Rum: 59
           Sugar: 897            Cotton: 7

         (1) Buy
         (2) Sell
         (3) Sail
         (4) Stash deposit
         (5) Stash withdraw
         (6) Pay down debt
         (7) Bank deposit
         (8) Bank withdraw
",
        )
    );
    Ok(())
}

#[test]
fn end_game_positive() -> UpdateResult<()> {
    let e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.gold = 40000;
            state.debt = 100;
            state.game_end = true;
            state
        }
        .initialize(),
    )?;
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
    let e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.gold = 100;
            state.debt = 40000;
            state.game_end = true;
            state
        }
        .initialize(),
    )?;
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
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.gold = 1400;
            state.inventory.cotton = 15;
            state.prices.london.cotton = 30;
            state
        }
        .initialize(),
    )?;
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
    let mut e =
        TestEngine::from_game_state(GameState::new(StdRng::seed_from_u64(42)).initialize())?;
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
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.stash.rum = 5;
            state.inventory.rum = 20;
            state
        }
        .initialize(),
    )?;
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
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.stash.tea = 30;
            state.inventory.tea = 14;
            state
        }
        .initialize(),
    )?;
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
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.debt = 500;
            state.gold = 1000;
            state
        }
        .initialize(),
    )?;
    assert!(e.expect("Gold 1000"));
    assert!(e.expect("Debt: 500"));
    assert!(e.expect("(6) Pay down debt"));
    e.charpress('6')?;
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
    Ok(())
}

#[test]
fn bank_deposit() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.gold = 1000;
            state.bank = 500;
            state
        }
        .initialize(),
    )?;
    assert!(e.expect("Gold 1000"));
    assert!(e.expect("Bank: 500"));
    assert!(e.expect("(7) Bank deposit"));
    e.charpress('7')?;
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
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.gold = 1000;
            state.bank = 500;
            state
        }
        .initialize(),
    )?;
    assert!(e.expect("Gold 1000"));
    assert!(e.expect("Bank: 500"));
    assert!(e.expect("(8) Bank withdraw"));
    e.charpress('8')?;
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
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
            state.mode = Mode::GameEvent(GameEvent::CheapGood(GoodType::Coffee));
            state
        }
        .initialize(),
    )?;
    assert!(e.expect("Cheap Coffee here!"));
    e.charpress('a')?;
    assert!(e.expect("Captain, the prices of goods here are:"));
    Ok(())
}
