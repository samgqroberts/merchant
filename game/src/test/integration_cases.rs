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
    e.expect("(1) Buy")?;
    e.charpress('1')?;
    e.expect("Which do you want to buy?")?;
    e.expect("(2) Tobacco")?;
    e.charpress('2')?;
    e.expect("How much Tobacco do you want?")?;
    e.expect("You can afford (13)")?;
    e.charpress('1')?;
    e.expect("How much Tobacco do you want? 1")?;
    e.charpress('0')?;
    e.expect("How much Tobacco do you want? 10")?;
    e.enterpress()?;
    e.expect("Tobacco: 10")?;
    Ok(())
}
