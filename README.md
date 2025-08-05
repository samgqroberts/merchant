# Merchant

A clone of the classic terminal UI game Drug Wars, but now set on a merchant ship in the 18th century.

![image](https://github.com/user-attachments/assets/79e77491-9485-4939-aadb-315563fa2cd2)

# How to install and play the game

## Play via Homebrew

```sh
brew install samgqroberts/tap/merchant
```

## Play via NPM

```sh
npx merchant-game
```

## Download Microsoft Installer for Windows

1. On the [latest release](https://github.com/samgqroberts/merchant/releases/latest) page, download the `.msi` File.
  - It will likely have a name like `merchant-x86_64-pc-windows-msvc.msi`
2. Run it to run a Setup Wizard (installer).
  - You will very likely have to tell it to Run Anyway, since it can't verify the publisher. Click "more info" to see this option.
3. Open up the Command Prompt program.
4. Run the `merchant` command

## Download binaries directly

Binary assets for the following targets are uploaded on the [latest release](https://github.com/samgqroberts/merchant/releases/latest) page:
* Apple Darwin aarch64 (ARM)
* Apple Darwin x86_64 (Intel)
* Linux
* Windows

## Run via source

Clone this repository, ensure you have the Rust / Cargo toolchain available, then from the repository root run:

```sh
cargo run --release
```

--- 
---

This project is licensed under the GNU General Public License v3.0 or later.
See the LICENSE file for details.