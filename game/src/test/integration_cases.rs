use rand::{rngs::StdRng, SeedableRng};

use crate::{engine::UpdateResult, state::GameState, test::test_engine::TestEngine};

#[test]
fn splash_screen_into_inventory() -> UpdateResult<()> {
    let mut test_engine = TestEngine::new()?;
    test_engine.expect_full(
        r"
Merchant

Navigate shifting markets and unreliable sources.

By samgqroberts

Press any key to begin
",
    )?;
    test_engine.charpress('a')?;
    test_engine.expect_full(
        "
         March 1782             Hold Size 100
         Gold 1400               Location London

          Home base              Inventory
           Sugar: 0               Sugar: 0
         Tobacco: 0             Tobacco: 0
             Tea: 0                 Tea: 0
          Cotton: 0              Cotton: 0
             Rum: 0                 Rum: 0
          Coffee: 0              Coffee: 0

            Debt: 1400

     Captain, the prices of goods here are:
           Sugar: 49            Tobacco: 106
             Tea: 52             Cotton: 98
             Rum: 48             Coffee: 40

         (1) Buy
         (2) Sell
         (3) Sail
         (4) Stash deposit
         (5) Stash withdraw
         (6) Borrow gold
         (7) Pay down debt
",
    )?;
    Ok(())
}

#[test]
fn buy_good() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state(
        {
            let seed = 42;
            GameState::new(StdRng::seed_from_u64(seed))
        }
        .initialize(),
    )?;
    assert!(e.expect("(1) Buy"));
    e.charpress('1')?;
    assert!(e.expect("Which do you want to buy?"));
    assert!(e.expect("(2) Tobacco"));
    e.charpress('2')?;
    assert!(e.expect("How much Tobacco do you want?"));
    assert!(e.expect("You can afford (13)"));
    e.charpress('1')?;
    assert!(e.expect("How much Tobacco do you want? 1"));
    e.charpress('0')?;
    assert!(e.expect("How much Tobacco do you want? 10"));
    e.enterpress()?;
    assert!(e.expect("Tobacco: 10"));
    Ok(())
}

#[test]
fn sell_good() -> UpdateResult<()> {
    let mut e = TestEngine::from_game_state(
        {
            let mut state = GameState::new(StdRng::seed_from_u64(42));
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
    assert!(e.expect("(4) Cotton"));
    e.charpress('4')?;
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
    assert!(e.expect("Debt: 1400"));
    assert!(e.expect("(3) Sail"));
    e.charpress('3')?;
    assert!(e.expect("Where do you want to sail?"));
    assert!(e.expect("(6) Venice"));
    e.charpress('6')?;
    assert!(e.expect("Location Venice"));
    assert!(e.expect("Debt: 1540"));
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
    assert!(e.expect("(3) Tea"));
    e.charpress('3')?;
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
fn borrow_gold() -> UpdateResult<()> {
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
    assert!(e.expect("(6) Borrow gold"));
    e.charpress('6')?;
    assert!(e.expect("How much gold do you want to borrow?"));
    e.charpress('3')?;
    assert!(e.expect("How much gold do you want to borrow? 3"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want to borrow? 30"));
    e.charpress('0')?;
    assert!(e.expect("How much gold do you want to borrow? 300"));
    e.enterpress()?;
    assert!(e.expect("Gold 1300"));
    assert!(e.expect("Debt: 800"));
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
    assert!(e.expect("(7) Pay down debt"));
    e.charpress('7')?;
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
