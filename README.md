# NumNum

A text editor that does math. Type what you're thinking, get answers as you type.

No buttons, no keypad. Just a blank page that understands numbers, units, currencies, and percentages in plain language. Results appear on the right, lined up with what you wrote. Click any result to copy it.

Blazingly fast. GPU-rendered UI powered by Zed's [GPUI](https://github.com/zed-industries/zed) framework, the same engine behind the fastest code editor on the planet. Written in Rust. Open source. Cross-platform.

![NumNum](assets/screenshot.png)

## Changes in this fork

This is a modified version of [NumNum](https://github.com/rudrabhoj/numnum) by Rudrabhoj Bhati, licensed under the same GPLv2 license. The git history records every change.

### Sums, subtotals and blocks

A block is the lines up to a blank line.

- `---` (three or more dashes) shows the running balance of the block so far, as `subtotal: x`. Useful for a list that counts down from an amount.
- `sum` (or a lone `=`) totals the block, shown as `= x` on a highlighted row.
- A `sum` after a blank line, with no block of its own, adds up the block sums above it, for a grand total.
- Sum and subtotal convert currencies and units, so `100 SEK` and `10 EUR` add up correctly.
- Plain numbers inherit the block's unit or currency. In a block that starts with `147.77 SEK`, a later `- 4.43` counts as kronor, and the result column shows it.
- Click a subtotal or sum to copy its value without the label.

```
147.77 SEK
- 29.55 SEK
---                          subtotal: 118.22 kr
- 4.43
- 25.65
---                          subtotal: 88.14 kr
- 66.10
sum                          = 22.04 kr
```

A grand total over several blocks, in mixed currencies:

```
rent = 9500 SEK                              9,500 kr
power = 650 SEK                              650 kr
sum                                          = 10,150 kr

fuel = 120 EUR                               € 120
food = 3200 SEK                              3,200 kr
sum                                          = 4,520 kr

sum                                          = 14,670 kr
```

### Prices per unit

A currency divided by a unit is a price per unit. Multiply it by an amount and you get the cost.

```
talk = 120 min
price = 2 SEK/min                            2 kr/min
talk * price                                 240 kr
1 TB * 0.12 SEK per GB                       120 kr
2 SEK/min in SEK/h                           120 kr/h
0.12 SEK/GB in USD/TB                        $11.43/TB
5% of price                                  0.1 kr/min
```

Prices can be scaled, added together and converted to another currency or unit.

### Stricter units and currencies

```
5 kg + 10 EUR                                Error: Incompatible units
120 min * 2 SEK                              Error: Incompatible units
6191.81 hours as a % of 600000 min           61.92 %
130 SEK as a % on 100 USD                    -87.62 %
100 Mbit/s in MB/s                           12.5 MB/s
1 GB / 100 Mbps                              80 s
9 Mbit/s * 3 h in GB                         12.15 GB
```

- A unit and a currency cannot be combined (`5 kg + 10 EUR`, `120 min * 2 SEK`). This gives an error instead of silently dropping the currency.
- `x as a % of/on/off y` converts x to y's unit or currency first, so `90 min as a % on 1 h` is 50 %.
- Data units are case-sensitive, since case tells bytes from bits: `MB` is megabyte and `Mb` is megabit. Lowercase forms such as `mb` and `gb` give an error. Added `kbit`, `Mbit`, `Gbit` and `Tbit`.
- Data rates convert (`100 Mbit/s in MB/s`), and matching units cancel (`9 Mbit/s * 3 h in GB`, `1 GB / 100 Mbps` gives seconds).

### Currency rates in the document

A line like `1 EUR = 10.95 SEK` sets that pair's rate, in both directions, for the lines below it. Other pairs keep the fetched rates.

```
1 EUR = 10.95 SEK                            10.95 kr
100 EUR in SEK                               1,095 kr
10 EUR + 50 SEK                              € 14.57
```

### Swedish number format

- New number format option: Swedish (`1 234 567,89`).
- Space-grouped numbers such as `3 594 272,5` are read as one number, so results can be pasted back in.
- A number that cannot be parsed in the active format, such as `3,594,272` with comma decimals, is an error instead of silently becoming 0.
- Numbers between 0.000001 and 0.01 are shown as plain decimals (`0.007`), not `7e-3`.

```
3 594 272,5 * 2                              7 188 545
1 234,5 + 0,5                                1 235
0,14 SEK * 0,05                              0,007 kr
```

### Files and multiple windows

- Open and save documents as plain-text `.numnum` files (Ctrl+O, Ctrl+S, Ctrl+Shift+S). Files autosave like sessions.
- Unsaved work is a scratchpad session. Save As moves it into the file.
- `numnum summary.numnum` opens a file from the command line, creating it if needed:

  ```sh
  numnum ~/budget/2027.numnum
  ```
- Several windows can run at once. A document open in one window is locked and cannot be opened in another, so two windows never overwrite each other.
- Opened files appear in the session menu next to the sessions.
- The Linux install script registers `.numnum` as a file type that opens in NumNum.

### Status bar

- Shows the total of the document, converted to the unit of the last result line. Sum lines are left out, so values are not counted twice.
- Shows the total of the selected lines next to it.
- A button copies the document, or the selected lines, as plain text with the results in a right-aligned column, for pasting into chat or email.

### Editor and window

- Title bar shows the document name, or "Scratchpad" for unsaved work.
- New title bar mode, Info: the NumNum title bar without window buttons.
- The status bar menu button is hidden when the title bar has one.
- Autocomplete lists data units with full names (`MB (Megabyte)`, `Mb (Megabit)`) and needs only one character after a number or `in`.
- The current-line highlight is hidden while text is selected.

Keyboard shortcuts added:

| Keys | Action |
|---|---|
| Ctrl+Left / Right | Previous / next word |
| Ctrl+Home / End | Start / end of document |
| Page Up / Down | Move one page |
| Shift + any of the above, Shift+Home / End, Shift+Up / Down | Extend the selection |
| Ctrl+N | New scratchpad |
| Ctrl+O / Ctrl+S / Ctrl+Shift+S | Open / Save / Save As |
| Ctrl+Plus / Minus / 0 | Larger / smaller / reset font size |
| Ctrl+Shift+C / V | Copy / paste |
| Super+W | Close window |

## How it works

```
rent = 45000 INR                              ₹ 45,000
groceries = 12000 INR                         ₹ 12,000
utilities = rent * 5%                         ₹ 2,250
total = rent + groceries + utilities          ₹ 59,250
total in USD                                  $709.28

5 m * 3 m                                    15 m²
60 km / 2 hours                               30 km/h
220 V * 10 A                                  2,200 W

500 kcal in kJ                                2,092 kJ
20% of what is 11.68 cm                       58.4 cm
0xff in decimal                               255
```

You type on the left. Results show up on the right. Change a variable and everything downstream updates. That's it.

## What it understands

**Math.** Full arithmetic with operator precedence, parentheses, and nested functions (`sin`, `cos`, `sqrt`, `log`, `abs`, `round`, 18 total). Variables persist across lines. Compound assignments (`tax += 5`) work.

**170+ currencies.** Symbols (`$`, `₹`, `€`, `£`, `¥`), ISO codes (`USD`, `INR`, `GBP`), and full names (`indian rupee`, `swiss franc`). Rates pulled live on startup, cached in SQLite, with a hardcoded fallback for offline use.

**100+ units.** Length, mass, time, temperature, area, volume, data, power, energy, voltage, current, resistance, frequency. Write `60 km / 2 hours` and get `30 km/h`. Write `220 V * 10 A` and get `2,200 W`. Shorthands like `mph`, `kmh`, `kbps` work. So does the word `per` (`100 km per hour`).

**Percentages in plain English.** `20% of 500`, `20% on 500`, `20% off 500`, `15% of what is 75`, or inline: `500 + 10%`.

**Number formats.** US (`1,234,567.89`), Indian (`12,34,567.89`), European (`1.234.567,89`). Paste formatted numbers from a spreadsheet and they parse correctly in the active locale.

**Representations.** `255 in hex`, `10 in binary`, `0xff in decimal`, scientific notation.

**Aggregation.** `sum`, `average`, `prev` reference results from earlier lines.

## The editor

NumNum is not a CLI tool or a widget. It's a proper text editor with:

- Syntax highlighting tuned for math expressions
- Autocomplete that knows every function, unit, currency, and variable in scope
- Double-click to select a word, triple-click for a line
- Undo/redo, cut/copy/paste, soft line wrapping
- Ctrl+scroll to change font size on the fly

Results on the right scroll with the editor and stay aligned, even when lines wrap. Click any result to copy it to the clipboard.

## Session persistence

Your work is automatically saved. Every calculation lives in a named session that persists across restarts. No manual save, no dialogs.

- Sessions are stored as JSON in your platform's data directory (`~/.local/share/numnum/sessions/` on Linux/FreeBSD)
- The most recent session is restored automatically when you reopen NumNum
- Click the burger menu in the top-right to switch between sessions or start a new one
- Hover over a session in the list to reveal a close button
- Empty sessions are cleaned up automatically so they don't clutter the list

![Session Menu](assets/sessions.png)

## Looks

8 color themes ship out of the box: Catppuccin Mocha, Catppuccin Latte, Tokyo Night, Tokyo Night Day, Rose Pine Moon, Rose Pine Dawn, Zed One Dark, Zed One Light.

Drop a `.toml` file in `~/.config/numnum/themes/` to add your own. Dark, light, and auto (follows system) appearance modes.

Optional custom titlebar with macOS-style traffic light buttons, or use your system's native one, or go with no titlebar at all.

## Font

NumNum looks best with [Maple Mono NF](https://github.com/subframe7536/maple-font) (the default). Install it before running:

```sh
# macOS / Linux (Homebrew)
brew install --cask font-maple-mono-nf

# Arch Linux
paru -S ttf-maplemono-nf-unhinted

# Windows (Scoop)
scoop bucket add nerd-fonts
scoop install Maple-Mono-NF
```

Or grab it from the [releases page](https://github.com/subframe7536/maple-font/releases). Any monospace font works, but Maple Mono NF is what NumNum is designed around. You can change the font in settings.

## Install

Prebuilt packages for every platform are attached to each [release](https://github.com/rudrabhoj/numnum/releases). Every file is named the same way: `NumNum-<version>-<os>-<arch>.<ext>`.

### Linux

Linux packages are built for both `x86_64` and `aarch64`. Pick the file matching your machine.

```sh
# Debian / Ubuntu
sudo apt install ./NumNum-*-linux-x86_64.deb

# Fedora / RHEL / openSUSE
sudo dnf install ./NumNum-*-linux-x86_64.rpm

# Arch / Arch Linux ARM
sudo pacman -U ./NumNum-*-archlinux-x86_64.pkg.tar.zst
```

Or download `NumNum-<version>-linux-<arch>.tar.xz` and put the `numnum` binary on your `PATH`.

### macOS

Download `NumNum-<version>-macos.dmg`, open it, and drag NumNum to Applications. The binary is universal, so one download runs on both Intel and Apple Silicon.

NumNum is not signed with an Apple Developer ID, and macOS Sequoia (15) and later removed the old Control-click bypass. First launch: double-click NumNum, dismiss the warning, then open **System Settings -> Privacy & Security**, scroll to the bottom and click **Open Anyway** next to the NumNum entry. You only do this once.

If you prefer the terminal, strip the quarantine attribute and the app opens normally:

```sh
xattr -d com.apple.quarantine /Applications/NumNum.app
```

### Windows

Download `NumNum-<version>-windows-x86_64.exe` and run it. It installs to Program Files with a Start Menu shortcut and an uninstaller. A portable `NumNum-<version>-windows-x86_64.zip` is also attached if you would rather not install.

The installer is unsigned, so on first launch Windows Defender SmartScreen shows **Windows protected your PC**. Click **More info** then **Run anyway**. You can also unblock the file in PowerShell before running it:

```powershell
Unblock-File .\NumNum-*-windows-x86_64.exe
```

If you have **Smart App Control** turned on (off by default on most systems, enabled only on some clean Windows 11 installs), it has no per-app override; you would have to disable it system-wide to install.

### FreeBSD

```sh
sudo pkg install ./NumNum-*-freebsd-x86_64.pkg
```

Or use the `NumNum-<version>-freebsd-x86_64.tar.xz` archive.

## Window manager tips (Hyprland, Niri)

NumNum is a small utility window. On a tiling Wayland compositor it feels best floating, borderless, and pinned, so it acts like a quick scratchpad instead of claiming a tile. It sets its Wayland `app-id` (and X11 `WM_CLASS`) to `numnum`, which is what these rules match on.

### Hyprland

Add to `~/.config/hypr/hyprland.conf`:

```
# NumNum calculator
windowrule = float on, match:class numnum
windowrule = pin on, match:class numnum
windowrule = border_size 0, match:class numnum
```

### Niri

Add to `~/.config/niri/config.kdl`:

```kdl
// NumNum calculator
window-rule {
    match app-id="numnum"
    open-floating true
    border {
        off
    }
    focus-ring {
        off
    }
}
```

## Building from source

### You'll need

- Rust 1.85+ (edition 2024)
- A C/C++ compiler
- CMake

Install Rust via [rustup](https://rustup.rs) if you don't have it:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Plus platform-specific libraries:

### Linux (Debian/Ubuntu)

```sh
sudo apt install \
  build-essential cmake clang mold \
  libasound2-dev libfontconfig-dev libssl-dev \
  libwayland-dev libx11-xcb-dev libxkbcommon-x11-dev \
  libzstd-dev libsqlite3-dev libvulkan1 libva-dev \
  libglib2.0-dev
```

```sh
./build_install_linux_bsd.sh
```

### Linux (Fedora)

```sh
sudo dnf install \
  gcc g++ cmake clang mold \
  alsa-lib-devel fontconfig-devel openssl-devel \
  wayland-devel libxcb-devel libxkbcommon-x11-devel \
  libzstd-devel sqlite-devel vulkan-loader libva-devel \
  glib2-devel
```

```sh
./build_install_linux_bsd.sh
```

### Linux (Arch)

```sh
sudo pacman -S \
  gcc clang cmake mold \
  alsa-lib fontconfig openssl \
  wayland libxcb libxkbcommon-x11 \
  zstd sqlite vulkan-icd-loader libva \
  glib2
```

```sh
./build_install_linux_bsd.sh
```

### FreeBSD

```sh
sudo pkg install cmake llvm git alsa-lib libX11 sqlite3
```

```sh
./build_install_linux_bsd.sh
```

NumNum sets `RUST_LIB_BACKTRACE=0` itself on FreeBSD to work around a GPUI performance issue, so no shell setup is needed.

The install script builds the release binary, copies it to `~/.local/bin/`, sets up the desktop entry and icon, and optionally configures Hyprland or Niri window rules.

### macOS

Xcode is required for Metal shader compilation and system frameworks.

```sh
xcode-select --install
brew install cmake
cargo build --release
```

### Windows

Visual Studio 2022 (or Build Tools) with the "Desktop development with C++" workload and a Windows 10/11 SDK.

```sh
cargo build --release
```

If the build can't find `fxc.exe` (HLSL shader compiler), point `GPUI_FXC_PATH` at your Windows SDK bin directory.

### Run it

```sh
cargo run --release
```

Binary lands at `target/release/numnum`.

## Configuration

Settings file: `~/.config/numnum/settings.toml` (Linux/FreeBSD), `~/Library/Application Support/numnum/settings.toml` (macOS), `%APPDATA%/numnum/settings.toml` (Windows).

Everything is configurable from the in-app settings pane (click the gear icon): font family and size, decimal precision, number format, color theme, appearance mode, titlebar style, and more.

## Built with

- [GPUI](https://github.com/zed-industries/zed) for GPU-accelerated rendering (from the Zed editor)
- [cosmic-text](https://github.com/pop-os/cosmic-text) for text shaping
- Pratt parser for expression evaluation
- SQLite for exchange rate caching

## License

NumNum is licensed under [GPLv2](LICENSE). The vendored GPUI crates are licensed under [Apache 2.0](LICENSE-APACHE).
