TODO

- [x] loan shark
- [x] bank
- [x] home base stash
- [x] end of game (30 month deadline?)
- [x] start with less cash than debt
- [x] different good prices (which of these goods is most expensive?)
- [x] stop showing pay down debt option when debt is 0
- events
  - [] buy guns (cannons?)
  - [] buy more hold space
    - in drug wars: "Will you buy a new trench coat with more pockets for 212 ?"
    - "rival drug dealers raded (sic) a pharmacy and are selling C H E A P  L U D E S !!!"
  - [] confrontations (pirates?)
    - choices: get captured, run, fight, ignore
    - in drug wars:
      - "Officer Hardass and 2 of his deupties are chasing you !!!!! (Press Space Bar)"
      - Damage 0, Cops 3, Guns 2. Will you run or fight ?
        - if run:
          - You can't lose them !!
          - They are firing on you man !!  They missed !! (or You've been hit !!, then Damage goes up by 1)
        - if fire:
          - You're Firing on them !!!
          - You Killed One !! (Cops value minus 1)
          - then they are firing on you man
  - [x] good sales (cheaper goods)
    - in drug wars: "Pigs are selling cheap heroin from last weeks raid !!"
  - [x] increased good prices
  - [] theft
  - [x] find random goods
    - [ ] find random goods is rarer for more valuable goods
  - [] no-effect events. "it's a sunny day"?
    - or maybe these are GPT-generated events, and indicate something
  - [] multiple events at single location?
  - [] in general: make things more in-universe
    - eg. instead of "you find x tobacco" how about "as you dock your ship, you see a crate and nobody in sight. inside you find x tobacco!." 
- [] enliven UI
  - [x] big ASCII ship
  - [] ASCII home base
  - [] put like with like – eg. hold size should be near (about) ship
  - [] borders around top info
  - UI overhaul
    - Split screen into 4 quadrants: top right is Ship / hold, Bottom right is market at the port you're at, Top left is home base info (including stash, bank and debt). Bottom left is choices (and event info?)
    - Maybe across whole top is date and gold
    - maybe event info is across whole bottom similarly
  - use colors
    - highlight choices like (1) (and/or bold?).
    - inactive (flavor-only) elements like the boat and the house should be duller / faded
- [] window resize / size awareness
- [] make merchant metaphors / style better


Design:
----------------------------------------|=================|----------------------------------------
|                                       | September  1782 |                                       |
|---------------------------------------|=================|---------------------------------------|
|     _____[LLL]______[LLL]____                                     |                             |
|    /     [LLL]      [LLL]    \                        |          )_)                            |
|   /___________________________\                      )_)        )___)         |                 |
|    )=========================(                      )___)       )____)       )_)\               |
|    '|I .--. I     Tea: 0000 I|                      )____)     /)_____)      )__)\              |
|     |I | +| I  Coffee: 0000 I|                     )_____)    /)______)\    )___) \             |
|     |I_|_+|_I   Sugar: 0000 I|                    )______)  //)_______) \\ )_____) \\           |
|    /_I______I Tobacco: 0000 I_\             _____//___|___///_____|______\\\__|_____\\\__=====  |
|     )========     Rum: 0000 =(              \      Tea: 0000 Coffee: 0000  Sugar: 0000  /       |
|     |I .--. I  Cotton: 0000 I|               \ Tobacco: 0000    Rum: 0000 Cotton: 0000 /        |
|     |I |<>| I               I|                \                                       /____     |
|     |I |~ | I Bank: 0000000 I|       --------- \  Gold: 0000000 Hold: 0000 Guns: 00  //.../---  |
|     |I |  | I Debt: 0000000 I|          ^^^^^ ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^  ^^^/.../      |
|     |I_|__|_I_______________I|                ^^^^      ^^^    ^^^^^^^^^    ^^^^^  /..../       |
|   ###(______)##################                        ^^^      ^^^^             /...../        |
|    ##(________)   ~"^"^~   ##                                                  /....../         |
|======(_________)========================<------------->======================/......../=========|
|      (__________)                       |  Amsterdam  |                    /........./          |
|                                         <------------->                                         |
|                                                                                                 |
|                              Captain, the prices of goods here are:                             |
|                                      Tea: 6858    Tobacco: 76                                   |
|                                   Coffee: 2514        Rum: 38                                   |
|                                    Sugar: 975      Cotton: 8                                    |
|                                                                                                 |
|                        (1) Buy    (4) Stash deposit    (6) Bank deposit                         |
|                        (2) Sell   (5) Stash withdraw   (7) Bank withdraw                        |
|                        (3) Sail                        (8) Pay down debt                        |
|                                                                                                 |
---------------------------------------------------------------------------------------------------