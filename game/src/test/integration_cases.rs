use crate::{engine::UpdateResult, state::GameState, test::test_engine::TestEngine};

#[test]
fn splash_screen_into_inventory() -> UpdateResult<()> {
    let mut test_engine = TestEngine::new()?;
    test_engine.expect(
        "Merchant

Navigate shifting markets and unreliable sources.

By samgqroberts

Press any key to begin",
    )?;
    test_engine.charpress('a')?;
    test_engine.expect(
        "         Date 1782-03-01        Hold Size 100
         Gold 1400               Location London

         Inventory
           Sugar: 0
         Tobacco: 0
             Tea: 0
          Cotton: 0
             Rum: 0
          Coffee: 0

     Captain, the prices of goods here are:
           Sugar: 57       Tobacco: 39
             Tea: 97        Cotton: 102
             Rum: 95        Coffee: 42

         (1) Buy
         (2) Sell
         (3) Sail",
    )?;
    Ok(())
}

#[test]
fn buy_good() -> UpdateResult<()> {
    let mut test_engine = TestEngine::from_game_state(GameState::from_u64_seed(42).initialize())?;
    test_engine.expect(
        "         Date 1782-03-01        Hold Size 100
         Gold 1400               Location London

         Inventory
           Sugar: 0
         Tobacco: 0
             Tea: 0
          Cotton: 0
             Rum: 0
          Coffee: 0

     Captain, the prices of goods here are:
           Sugar: 57       Tobacco: 39
             Tea: 97        Cotton: 102
             Rum: 95        Coffee: 42

         (1) Buy
         (2) Sell
         (3) Sail",
    )?;
    test_engine.charpress('1')?;
    test_engine.expect(
        "         Date 1782-03-01        Hold Size 100
         Gold 1400               Location London

         Inventory
           Sugar: 0
         Tobacco: 0
             Tea: 0
          Cotton: 0
             Rum: 0
          Coffee: 0

     Captain, the prices of goods here are:
           Sugar: 57       Tobacco: 39
             Tea: 97        Cotton: 102
             Rum: 95        Coffee: 42

         Which do you want to buy?
         (1) Sugar
         (2) Tobacco
         (3) Tea
         (4) Cotton
         (5) Rum
         (6) Coffee",
    )?;
    test_engine.charpress('2')?;
    test_engine.expect(
        "         Date 1782-03-01        Hold Size 100
         Gold 1400               Location London

         Inventory
           Sugar: 0
         Tobacco: 0
             Tea: 0
          Cotton: 0
             Rum: 0
          Coffee: 0

     Captain, the prices of goods here are:
           Sugar: 57       Tobacco: 39
             Tea: 97        Cotton: 102
             Rum: 95        Coffee: 42

         How much Tobacco do you want? 
         You can afford (35)",
    )?;
    test_engine.charpress('1')?;
    test_engine.expect(
        "         Date 1782-03-01        Hold Size 100
         Gold 1400               Location London

         Inventory
           Sugar: 0
         Tobacco: 0
             Tea: 0
          Cotton: 0
             Rum: 0
          Coffee: 0

     Captain, the prices of goods here are:
           Sugar: 57       Tobacco: 39
             Tea: 97        Cotton: 102
             Rum: 95        Coffee: 42

         How much Tobacco do you want? 1
         You can afford (35)",
    )?;
    test_engine.charpress('0')?;
    test_engine.expect(
        "         Date 1782-03-01        Hold Size 100
         Gold 1400               Location London

         Inventory
           Sugar: 0
         Tobacco: 0
             Tea: 0
          Cotton: 0
             Rum: 0
          Coffee: 0

     Captain, the prices of goods here are:
           Sugar: 57       Tobacco: 39
             Tea: 97        Cotton: 102
             Rum: 95        Coffee: 42

         How much Tobacco do you want? 10
         You can afford (35)",
    )?;
    test_engine.enterpress()?;
    test_engine.expect(
        "         Date 1782-03-01        Hold Size 100
         Gold 1010               Location London

         Inventory
           Sugar: 0
         Tobacco: 10
             Tea: 0
          Cotton: 0
             Rum: 0
          Coffee: 0

     Captain, the prices of goods here are:
           Sugar: 57       Tobacco: 39
             Tea: 97        Cotton: 102
             Rum: 95        Coffee: 42

         (1) Buy
         (2) Sell
         (3) Sail",
    )?;
    Ok(())
}
