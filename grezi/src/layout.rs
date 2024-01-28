use std::fmt::Display;

use ahash::RandomState;
use eframe::egui::Rect;
pub type AHashMap<K, V> = std::collections::HashMap<K, V, RandomState>;
use cassowary::strength::{REQUIRED, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Constraint as CassowaryConstraint, Expression, Solver, Variable};
use eframe::epaint::Pos2;
use serde::{Deserialize, Serialize};

use crate::parser::viewboxes;

/// The direction in which the viewbox's boxes go
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Direction {
    /// Left to right
    Horizontal,
    /// Top to bottom
    Vertical,
}

impl From<viewboxes::Direction> for Direction {
    fn from(value: viewboxes::Direction) -> Self {
        match value {
            viewboxes::Direction::Up | viewboxes::Direction::Down => Direction::Vertical,
            viewboxes::Direction::Left | viewboxes::Direction::Right => Direction::Horizontal,
            viewboxes::Direction::Center => Direction::Vertical,
        }
    }
}

/// A Constraint decides how viewboxes are split. In the `.grz` format, the available constraints
/// are
/// - `1:2`: [`Constraint::Ratio`]
/// - `50%`: [`Constraint::Percentage`]
/// - `50+`: [`Constraint::Max`]
/// - `50-`: [`Constraint::Min`]
/// - `50`: [`Constraint::Length`]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Constraint {
    /// Allocate this percentage of the box
    Percentage(f32),
    /// Allocate this portion of the box
    Ratio(f32, f32),
    /// Allocate exactly this amount of the box
    Length(f32),
    /// Allocate at most this much of the box
    Max(f32),
    /// Allocate at least this much of the box
    Min(f32),
}

impl Display for Constraint {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Constraint::Max(n) => write!(f, "{}+", n),
            Constraint::Min(n) => write!(f, "{}-", n),
            Constraint::Ratio(n, m) => write!(f, "{}:{}", n, m),
            Constraint::Length(n) => write!(f, "{}~", n),
            Constraint::Percentage(n) => write!(f, "{}%", n),
        }
    }
}

impl Constraint {
    // /// Apply a constraint directly on a given length
    // #[inline]
    // pub fn apply(&self, length: f32) -> f32 {
    //     match *self {
    //         Constraint::Percentage(p) => length * p / 100.0,
    //         Constraint::Ratio(num, den) => num * length / den,
    //         Constraint::Length(l) => length.min(l),
    //         Constraint::Max(m) => length.min(m),
    //         Constraint::Min(m) => length.max(m),
    //     }
    // }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnresolvedLayout {
    /// The direction in which the boxes inside of the viewbox should go.
    pub direction: Direction,
    /// The margin between the boxes inside of a viewbox
    pub margin: f32,
    /// Tells the solver how the boxes should be allocated inside of the viewbox
    pub constraints: Vec<Constraint>,
    /// Whether the last chunk of the computed layout should be expanded to fill the available
    /// space.
    pub expand_to_fill: bool,
    pub split_on: viewboxes::ViewboxIn,
}

/// A raw, unsolved viewbox. You must use the [`Layout.split()`] method to solve the viewbox.
#[derive(Debug, Clone, PartialEq)]
pub struct Layout<'a> {
    /// The direction in which the boxes inside of the viewbox should go.
    direction: Direction,
    /// The margin between the boxes inside of a viewbox
    margin: f32,
    /// Tells the solver how the boxes should be allocated inside of the viewbox
    constraints: &'a [Constraint],
    /// Whether the last chunk of the computed layout should be expanded to fill the available
    /// space.
    expand_to_fill: bool,
}

impl<'a> Default for Layout<'a> {
    #[inline]
    fn default() -> Layout<'a> {
        Layout {
            direction: Direction::Vertical,
            margin: 15.0,
            constraints: &[],
            expand_to_fill: true,
        }
    }
}

impl<'a> Layout<'a> {
    /// Sets the constraints for the unsolved viewbox
    #[inline]
    pub fn constraints(mut self, constraints: &'a [Constraint]) -> Layout<'a> {
        self.constraints = constraints;
        self
    }

    // /// Sets the vertical and horizontal margins for the unsolved viewbox
    // #[inline]
    // pub fn margin(mut self, margin: f32) -> Layout<'a> {
    //     self.margin = margin;
    //     self
    // }

    /// Sets the direction of the unsolved viewbox
    #[inline]
    pub fn direction(mut self, direction: Direction) -> Layout<'a> {
        self.direction = direction;
        self
    }

    #[inline]
    pub fn margin(mut self, margin: f32) -> Layout<'a> {
        self.margin = margin;
        self
    }

    /// Wrapper function around the cassowary-rs solver to be able to split a given
    /// viewbox into smaller ones based on the constraints and the direction.
    ///
    /// # Examples
    /// ```
    /// # use grezi::layout::{Rect, Constraint, Direction, Layout};
    /// let chunks = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5.0), Constraint::Min(0.0)].as_ref())
    ///     .split(Rect {
    ///         left: 2.0,
    ///         top: 2.0,
    ///         right: 12.0,
    ///         bottom: 12.0,
    ///     });
    /// assert_eq!(
    ///     chunks,
    ///     vec![
    ///         Rect {
    ///             left: 2.0,
    ///             top: 2.0,
    ///             right: 12.0,
    ///             bottom: 7.0
    ///         },
    ///         Rect {
    ///             left: 2.0,
    ///             top: 7.0,
    ///             right: 12.0,
    ///             bottom: 12.0
    ///         }
    ///     ]
    /// );
    ///
    /// let chunks = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Ratio(1.0, 3.0), Constraint::Ratio(2.0, 3.0)].as_ref())
    ///     .split(Rect {
    ///         left: 0.0,
    ///         top: 0.0,
    ///         right: 9.0,
    ///         bottom: 2.0,
    ///     });
    /// assert_eq!(
    ///     chunks,
    ///     vec![
    ///         Rect {
    ///             left: 0.0,
    ///             top: 0.0,
    ///             right: 3.0,
    ///             bottom: 2.0
    ///         },
    ///         Rect {
    ///             left: 3.0,
    ///             top: 0.0,
    ///             right: 9.0,
    ///             bottom: 2.0
    ///         }
    ///     ]
    /// );
    /// ```
    pub fn split(self, dest_area: Rect) -> Vec<Rect> {
        let dest_area = dest_area.shrink(self.margin);
        let mut solver = Solver::new();
        let mut vars: AHashMap<Variable, (usize, usize)> = AHashMap::default();
        let elements = self
            .constraints
            .iter()
            .map(|_| Element::new())
            .collect::<Vec<Element>>();
        let mut results = self
            .constraints
            .iter()
            .map(|_| Rect {
                min: Pos2::ZERO,
                max: Pos2::ZERO,
            })
            .collect::<Vec<Rect>>();

        for (i, e) in elements.iter().enumerate() {
            vars.insert(e.left, (i, 0));
            vars.insert(e.top, (i, 1));
            vars.insert(e.right, (i, 2));
            vars.insert(e.bottom, (i, 3));
        }
        let mut ccs: Vec<CassowaryConstraint> =
            Vec::with_capacity(elements.len() * 4 + self.constraints.len() * 6);
        for elt in &elements {
            ccs.push(elt.left | GE(REQUIRED) | dest_area.min.x);
            ccs.push(elt.top | GE(REQUIRED) | dest_area.min.y);
            ccs.push(elt.right | LE(REQUIRED) | dest_area.max.x);
            ccs.push(elt.bottom | LE(REQUIRED) | dest_area.max.y);
        }
        if let Some(first) = elements.first() {
            ccs.push(match self.direction {
                Direction::Horizontal => first.left | EQ(REQUIRED) | dest_area.min.x,
                Direction::Vertical => first.top | EQ(REQUIRED) | dest_area.min.y,
            });
        }
        if self.expand_to_fill {
            if let Some(last) = elements.last() {
                ccs.push(match self.direction {
                    Direction::Horizontal => last.right | EQ(REQUIRED) | dest_area.max.x,
                    Direction::Vertical => last.bottom | EQ(REQUIRED) | dest_area.max.y,
                });
            }
        }
        match self.direction {
            Direction::Horizontal => {
                for pair in elements.windows(2) {
                    ccs.push((pair[0].left + pair[0].width()) | EQ(REQUIRED) | pair[1].left);
                }
                for (i, size) in self.constraints.iter().enumerate() {
                    ccs.push(elements[i].top | EQ(REQUIRED) | dest_area.min.y);
                    ccs.push(elements[i].height() | EQ(REQUIRED) | dest_area.height());
                    ccs.push(match *size {
                        Constraint::Length(v) => elements[i].width() | EQ(WEAK) | v,
                        Constraint::Percentage(v) => {
                            elements[i].width() | EQ(WEAK) | (v * dest_area.width() / 100.0)
                        }
                        Constraint::Ratio(n, d) => {
                            elements[i].width() | EQ(WEAK) | (dest_area.width() * n / d)
                        }
                        Constraint::Min(v) => elements[i].width() | GE(WEAK) | v,
                        Constraint::Max(v) => elements[i].width() | LE(WEAK) | v,
                    });
                }
            }
            Direction::Vertical => {
                for pair in elements.windows(2) {
                    ccs.push((pair[0].top + pair[0].height()) | EQ(REQUIRED) | pair[1].top);
                }
                for (i, size) in self.constraints.iter().enumerate() {
                    ccs.push(elements[i].left | EQ(REQUIRED) | dest_area.min.x);
                    ccs.push(elements[i].width() | EQ(REQUIRED) | dest_area.width());
                    ccs.push(match *size {
                        Constraint::Length(v) => elements[i].height() | EQ(WEAK) | v,
                        Constraint::Percentage(v) => {
                            elements[i].height() | EQ(WEAK) | (v * dest_area.height() / 100.0)
                        }
                        Constraint::Ratio(n, d) => {
                            elements[i].height() | EQ(WEAK) | (dest_area.height() * n / d)
                        }
                        Constraint::Min(v) => elements[i].height() | GE(WEAK) | v,
                        Constraint::Max(v) => elements[i].height() | LE(WEAK) | v,
                    });
                }
            }
        }
        solver.add_constraints(&ccs).unwrap();
        for &(var, value) in solver.fetch_changes() {
            let (index, attr) = vars[&var];
            let value = if value.is_sign_negative() {
                0.0
            } else {
                value as f32
            };
            match attr {
                0 => {
                    results[index].min.x = value;
                }
                1 => {
                    results[index].min.y = value;
                }
                2 => {
                    results[index].max.x = value;
                }
                3 => {
                    results[index].max.y = value;
                }
                _ => {}
            }
        }

        if self.expand_to_fill {
            // Fix imprecision by extending the last item a bit if necessary
            if let Some(last) = results.last_mut() {
                match self.direction {
                    Direction::Vertical => {
                        last.max.y = dest_area.max.y;
                    }
                    Direction::Horizontal => {
                        last.max.x = dest_area.max.x;
                    }
                }
            }
        }
        results
    }
}

/// A container used by the solver inside split
struct Element {
    left: Variable,
    top: Variable,
    right: Variable,
    bottom: Variable,
}

impl Element {
    #[inline]
    fn new() -> Element {
        Element {
            left: Variable::new(),
            top: Variable::new(),
            right: Variable::new(),
            bottom: Variable::new(),
        }
    }

    #[inline]
    pub fn width(&self) -> Expression {
        self.right - self.left
    }

    #[inline]
    pub fn height(&self) -> Expression {
        self.bottom - self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_split_by_height() {
        let target = Rect {
            min: Pos2::new(2.0, 2.0),
            max: Pos2::new(12.0, 12.0),
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Percentage(10.0),
                Constraint::Max(5.0),
                Constraint::Min(1.0),
            ])
            .split(target);

        println!("{:?}", chunks);

        assert_eq!(
            target.height(),
            chunks.iter().map(|r| r.height()).sum::<f32>()
        );
        chunks
            .windows(2)
            .for_each(|w| assert!(w[0].min.y <= w[1].min.y));
    }
}
