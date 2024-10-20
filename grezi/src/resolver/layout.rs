use std::collections::HashMap;
use std::fmt::Display;

use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};
use cassowary::{AddConstraintError, WeightedRelation::*};
use cassowary::{Expression, Solver, Variable};
use eframe::egui::Rect;
use eframe::epaint::{Pos2, Vec2};
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
    /// Space evenly all the elements
    Auto(crate::parser::viewboxes::Direction),
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
            Constraint::Auto(d) => write!(f, "a{}", d),
        }
    }
}

impl Constraint {
    pub fn apply(&self, length: f32) -> f32 {
        match *self {
            Constraint::Percentage(p) => {
                let p = p / 100.0;
                (p * length).min(length)
            }
            Constraint::Ratio(numerator, denominator) => {
                // avoid division by zero by using 1 when denominator is 0
                // this results in 0/0 -> 0 and x/0 -> x for x != 0
                let percentage = numerator / denominator.max(1.0);
                (percentage * length).min(length)
            }
            Constraint::Length(l) => length.min(l),
            Constraint::Max(m) => length.min(m),
            Constraint::Min(m) => length.max(m),
            Constraint::Auto(_) => length,
        }
    }

    /// Convert an iterator of lengths into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_lengths([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_lengths<T>(lengths: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = f32>,
    {
        lengths
            .into_iter()
            .map(Constraint::Length)
            .collect::<Vec<_>>()
    }

    /// Convert an iterator of ratios into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_ratios<T>(ratios: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = (f32, f32)>,
    {
        ratios
            .into_iter()
            .map(|(n, d)| Constraint::Ratio(n, d))
            .collect()
    }

    /// Convert an iterator of percentages into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_percentages([25, 50, 25]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_percentages<T>(percentages: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = f32>,
    {
        percentages
            .into_iter()
            .map(Constraint::Percentage)
            .collect()
    }

    /// Convert an iterator of maxes into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_maxes([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_maxes<T>(maxes: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = f32>,
    {
        maxes.into_iter().map(Constraint::Max).collect()
    }

    /// Convert an iterator of mins into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_mins([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_mins<T>(mins: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = f32>,
    {
        mins.into_iter().map(Constraint::Min).collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnresolvedLayout {
    /// The direction in which the boxes inside of the viewbox should go.
    pub direction: Direction,
    /// The margin between the boxes inside of a viewbox
    pub margin: f32,
    /// The margin between the boxes _within_ a viewbox
    pub margin_per: f32,
    /// Tells the solver how the boxes should be allocated inside of the viewbox
    pub constraints: Vec<Constraint>,
    /// Whether the last chunk of the computed layout should be expanded to fill the available
    /// space.
    pub expand_to_fill: bool,
    pub split_on: viewboxes::ViewboxIn,
}

#[derive(Copy, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum SegmentSize {
    /// prefer equal chunks if other constraints are all satisfied
    EvenDistribution,

    /// the last chunk is expanded to fill the remaining space
    #[default]
    LastTakesRemainder,

    /// extra space is not distributed
    None,
}

/// A raw, unsolved viewbox. You must use the [`Layout.split()`] method to solve the viewbox.
#[derive(Debug, Clone, PartialEq)]
pub struct Layout<'a> {
    /// The direction in which the boxes inside of the viewbox should go.
    direction: Direction,
    /// The margin between the boxes inside of a viewbox
    margin: f32,
    /// The margin between the boxes _within_ a viewbox
    margin_per: f32,
    /// Tells the solver how the boxes should be allocated inside of the viewbox
    constraints: &'a [Constraint],
    /// option for segment size preferences
    segment_size: SegmentSize,
}

impl<'a> Default for Layout<'a> {
    #[inline]
    fn default() -> Layout<'a> {
        Layout {
            direction: Direction::Vertical,
            margin: 15.0,
            margin_per: 0.0,
            constraints: &[],
            segment_size: SegmentSize::LastTakesRemainder,
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

    #[inline]
    pub fn margin_per(mut self, margin_per: f32) -> Layout<'a> {
        self.margin_per = margin_per;
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
    pub fn split(self, area: Rect) -> Result<Vec<Rect>, AddConstraintError> {
        let mut solver = Solver::new();
        let inner = area.shrink(self.margin);

        let (area_start, area_end) = match self.direction {
            Direction::Horizontal => (inner.min.x, inner.right()),
            Direction::Vertical => (inner.min.y, inner.bottom()),
        };
        let area_size = area_end - area_start;

        // create an element for each constraint that needs to be applied. Each element defines the
        // variables that will be used to compute the layout.
        let elements = self
            .constraints
            .iter()
            .map(|_| Element::new())
            .collect::<Vec<Element>>();

        // ensure that all the elements are inside the area
        for element in &elements {
            solver.add_constraints(&[
                element.start | GE(REQUIRED) | area_start,
                element.end | LE(REQUIRED) | area_end,
                element.start | LE(REQUIRED) | element.end,
            ])?;
        }
        // ensure there are no gaps between the elements
        for pair in elements.windows(2) {
            solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
        }
        // ensure the first element touches the left/top edge of the area
        if let Some(first) = elements.first() {
            solver.add_constraint(first.start | EQ(REQUIRED) | area_start)?;
        }
        if self.segment_size != SegmentSize::None {
            // ensure the last element touches the right/bottom edge of the area
            if let Some(last) = elements.last() {
                solver.add_constraint(last.end | EQ(REQUIRED) | area_end)?;
            }
        }
        // apply the constraints
        for (&constraint, &element) in self.constraints.iter().zip(elements.iter()) {
            match constraint {
                Constraint::Percentage(p) => {
                    let percent = p / 100.00;
                    solver.add_constraint(element.size() | EQ(STRONG) | (area_size * percent))?;
                }
                Constraint::Ratio(n, d) => {
                    // avoid division by zero by using 1 when denominator is 0
                    let ratio = n / d.max(1.0);
                    solver.add_constraint(element.size() | EQ(STRONG) | (area_size * ratio))?;
                }
                Constraint::Length(l) => solver.add_constraint(element.size() | EQ(STRONG) | l)?,
                Constraint::Max(m) => {
                    solver.add_constraints(&[
                        element.size() | LE(STRONG) | m,
                        element.size() | EQ(MEDIUM) | m,
                    ])?;
                }
                Constraint::Min(m) => {
                    solver.add_constraints(&[
                        element.size() | GE(STRONG) | m,
                        element.size() | EQ(MEDIUM) | m,
                    ])?;
                }
                Constraint::Auto(_) => {
                    solver.add_constraints(&[
                        element.size() | GE(STRONG) | 0.0,
                        element.size() | EQ(MEDIUM) | 0.0,
                    ])?;
                }
            }
        }
        // prefer equal chunks if other constraints are all satisfied
        if self.segment_size == SegmentSize::EvenDistribution {
            for el in elements.chunks(2) {
                solver.add_constraint(el[0].size() | EQ(WEAK) | el[1].size())?;
            }
        }

        let changes: HashMap<Variable, f64, ahash::RandomState> =
            solver.fetch_changes().iter().copied().collect();

        // please leave this comment here as it's useful for debugging unit tests when we make any
        // changes to layout code - we should replace this with tracing in the future.
        // let ends = format!(
        //     "{:?}",
        //     elements
        //         .iter()
        //         .map(|e| changes.get(&e.end).unwrap_or(&0.0))
        //         .collect::<Vec<&f64>>()
        // );
        // dbg!(ends);

        // convert to Rects
        let results = elements
            .iter()
            .map(|element| {
                let start = changes.get(&element.start).unwrap_or(&0.0).round() as f32;
                let end = changes.get(&element.end).unwrap_or(&0.0).round() as f32;
                let size = end - start;
                match self.direction {
                    Direction::Horizontal => Rect::from_min_size(
                        Pos2::new(start, inner.min.y),
                        Vec2::new(size, inner.height()),
                    )
                    .shrink(self.margin_per),
                    Direction::Vertical => Rect::from_min_size(
                        Pos2::new(inner.min.x, start),
                        Vec2::new(inner.width(), size),
                    )
                    .shrink(self.margin_per),
                }
            })
            .collect::<Vec<_>>();
        Ok(results)
    }
}

/// A container used by the solver inside split
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Element {
    start: Variable,
    end: Variable,
}

impl Element {
    fn new() -> Element {
        Element {
            start: Variable::new(),
            end: Variable::new(),
        }
    }

    fn size(&self) -> Expression {
        self.end - self.start
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
            .split(target)
            .unwrap();

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
