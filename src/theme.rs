//! Tokyo Night theme, ported from the user's tmux-dotbar palette.
//!
//! All statusline colors are centralized here so the look can be tweaked
//! in one place. Edit these constants and `cargo build --release` again.

use tuie::prelude::Color;

/// Primary accent (selection / active blocks / clock). dotbar `fg-session`.
pub const ACCENT: Color = Color::Rgb(0x7a, 0xa2, 0xf7); // #7AA2F7 blue

/// Bar background (normal). Darker than blocks for contrast. dotbar `bg`.
pub const BAR_BG: Color = Color::Rgb(0x11, 0x13, 0x1d); // #11131D

/// Bar background when the pane is zoomed (subtle highlight).
pub const BAR_BG_ZOOMED: Color = Color::Rgb(0x29, 0x2e, 0x42); // #292E42

/// Block background (git/time segments), normal.
pub const BLOCK_BG: Color = Color::Rgb(0x1f, 0x23, 0x35); // #1F2335 (bg_dark)

/// Block background when zoomed.
pub const BLOCK_BG_ZOOMED: Color = Color::Rgb(0x3b, 0x42, 0x61); // #3B4261

/// Foreground placed on top of the accent fill (dark text on blue).
pub const FG_ON_ACCENT: Color = Color::Rgb(0x16, 0x16, 0x1e); // #16161E

/// Separator / divider glyph color. dotbar `fg`.
pub const SEPARATOR: Color = Color::Rgb(0x56, 0x5f, 0x89); // #565F89 comment

/// Error / unknown path color.
pub const ERROR: Color = Color::Rgb(0xf7, 0x76, 0x8e); // #F7768E red
