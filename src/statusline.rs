use std::process::Command;

use chrono::Local;
use tuie::prelude::*;

use crate::block::Block;
use crate::blockrow::{BlockRow, Row};
use crate::options::{Action, Options, Side};
use crate::powerline::Powerline;
use crate::render;
use crate::tabs::Tabs;

const GIT_BRANCH_ICON: &str = "\u{e0a0}";
const UNKNOWN_PATH_ICON: &str = "???";
pub use crate::theme::ACCENT;
use crate::theme;
const POWERLINE_FILL: Powerline = Powerline::BLOCK;
const POWERLINE_DIVIDER: Powerline = Powerline::BLOCK;
const ENCLOSE_ACTIVE: bool = false;

fn get_git_branch(pane_path: &str) -> String {
    Command::new("git")
        .current_dir(pane_path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        })
        .unwrap_or_default()
}

#[derive(Clone, Copy)]
pub struct Selection {
    pub y_start: i32,
    pub y_end: i32,
    pub x_start: i32,
    pub x_end: i32,
}

impl Selection {
    fn label(&self) -> String {
        if self.y_start != self.y_end {
            format!("{} rows", (self.y_start - self.y_end).abs() + 1)
        } else {
            let cols = (self.x_start - self.x_end).abs() + 1;
            format!("{} col{}", cols, if cols == 1 { "" } else { "s" })
        }
    }
}

pub fn create_right_row(config: &Options, bar_bg: Color) -> Row {
    let selection = config.has_selection.then_some(config.selection);
    let session = match selection {
        Some(sel) => sel.label(),
        None => config.session_title.clone(),
    };
    let fill = Style::new().fg(theme::FG_ON_ACCENT).bg(ACCENT).bold();
    let block_bg = theme::BLOCK_BG;
    let now = Local::now();
    let clock_time = now.format("%H:%M").to_string();
    let clock_date = now.format(" %d-%b-%y ").to_string();

    let mut row = BlockRow::new(bar_bg)
        .right()
        .families(&POWERLINE_FILL, &POWERLINE_DIVIDER)
        .enclose(ENCLOSE_ACTIVE)
        .separator(Style::new().fg(theme::SEPARATOR))
        .text(config.pane_title.clone(), Style::new())
        .truncate()
        .flex()
        .push(path_block(&config.pane_path));

    let branch = get_git_branch(&config.pane_path);
    if !branch.is_empty() {
        row = row.text(
            format!("{} {}", GIT_BRANCH_ICON, branch),
            Style::new().bg(block_bg),
        );
    }

    row
        .active(session, fill)
        .push(
            Block::new()
                .span(
                    clock_time,
                    Style::new().bg(block_bg).fg(theme::CLOCK_TIME_FG),
                )
                .span(
                    clock_date,
                    Style::new().bg(block_bg).fg(theme::CLOCK_DATE_FG),
                ),
        )
        .row()
}

fn path_block(pane_path: &str) -> Block {
    let filename = if pane_path == "/" {
        Some("/")
    } else {
        std::path::Path::new(pane_path)
            .file_name()
            .and_then(|n| n.to_str())
    };
    match filename {
        Some(name) => Block::new().span(name.to_string(), Style::new()),
        None => Block::new()
            .span(UNKNOWN_PATH_ICON.to_string(), Style::new().fg(theme::ERROR)),
    }
}

fn empty_row(bar_bg: Color) -> Row {
    BlockRow::new(bar_bg).row()
}

pub fn render(config: &Options) -> std::io::Result<std::process::ExitCode> {
    let bar_bg = theme::BAR_BG;
    let width = config.client_size.x;
    let tabs = Tabs::new(config.window_idx, &config.windows)
        .active_style(Style::new().fg(theme::FG_ON_ACCENT).bg(ACCENT).bold())
        .families(&POWERLINE_FILL, &POWERLINE_DIVIDER)
        .enclose(ENCLOSE_ACTIVE)
        .separator(Style::new().fg(theme::SEPARATOR))
        .bar_bg(bar_bg)
        .zoomed(config.is_zoomed);

    // Width of the left half when splitting around a centred badge.
    let left_w = width.saturating_sub(config.badge) / 2;
    let right_w = width.saturating_sub(config.badge).saturating_sub(left_w);

    match config.side {
        // Right half only: just the info row, right-aligned in its width.
        Side::Right => print!(
            "{}",
            render::emit(
                empty_row(bar_bg),
                create_right_row(config, bar_bg),
                right_w
            )
        ),
        // Left half (tabs) only. Clicks/drags hit-test against this width;
        // tabs start at column 0 so the full-client mouse_x maps directly.
        Side::Left => match config.action {
            Action::Drag(mouse_x) => tabs.drag(bar_bg, left_w, mouse_x),
            Action::Click(mouse_x) => {
                render::click(tabs.row(), empty_row(bar_bg), left_w, mouse_x)
            }
            _ => print!(
                "{}",
                render::emit(tabs.row(), empty_row(bar_bg), left_w)
            ),
        },
        // Whole bar in one piece (default / no badge split).
        Side::Both => match config.action {
            Action::Drag(mouse_x) => tabs.drag(bar_bg, width, mouse_x),
            Action::Click(mouse_x) => render::click(
                tabs.row(),
                create_right_row(config, bar_bg),
                width,
                mouse_x,
            ),
            _ => print!(
                "{}",
                render::emit(
                    tabs.row(),
                    create_right_row(config, bar_bg),
                    width
                )
            ),
        },
    }

    Ok(std::process::ExitCode::SUCCESS)
}
