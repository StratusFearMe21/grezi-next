#![warn(clippy::missing_const_for_fn)]
//! Provides types and traits for working with layout and positioning in the terminal.
//! Code borrowed from <https://github.com/ratatui/ratatui>

mod constraint;
mod flex;
mod layout;
mod rect;

pub use constraint::Constraint;
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}
impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' | '_' => Ok(Direction::Vertical),
            '>' | '<' => Ok(Direction::Horizontal),
            _ => Err(()),
        }
    }
}
pub use flex::Flex;
pub use layout::{Layout, Spacing};
pub use rect::{Columns, Positions, Rows};
