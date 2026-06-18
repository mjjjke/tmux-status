//! Tokyo Night theme, ported from the user's tmux-dotbar palette.
//!
//! All statusline colors are centralized here so the look can be tweaked
//! in one place. Edit these constants and `cargo build --release` again.

use tuie::prelude::Color;

/// Primary accent (selection / active blocks / clock). dotbar `fg-session`.
pub const ACCENT: Color = Color::Rgb(0x7a, 0xa2, 0xf7); // #7AA2F7 blue

/// Bar background. Darker than blocks for contrast. dotbar `bg`.
pub const BAR_BG: Color = Color::Rgb(0x11, 0x13, 0x1d); // #11131D

/// Block background (git/time segments).
pub const BLOCK_BG: Color = Color::Rgb(0x1f, 0x23, 0x35); // #1F2335 (bg_dark)

/// Foreground placed on top of the accent fill (dark text on blue).
pub const FG_ON_ACCENT: Color = Color::Rgb(0x16, 0x16, 0x1e); // #16161E

/// Separator / divider glyph color. dotbar `fg`.
pub const SEPARATOR: Color = Color::Rgb(0x56, 0x5f, 0x89); // #565F89 comment

/// Clock time (HH:MM) foreground: bright, so it stands out. dotbar `fg-current`.
pub const CLOCK_TIME_FG: Color = Color::Rgb(0xc0, 0xca, 0xf5); // #C0CAF5

/// Clock date (dd-Mon-yy) foreground: muted, recedes vs the time.
pub const CLOCK_DATE_FG: Color = Color::Rgb(0x56, 0x5f, 0x89); // #565F89

/// Error / unknown path color.
pub const ERROR: Color = Color::Rgb(0xf7, 0x76, 0x8e); // #F7768E red
