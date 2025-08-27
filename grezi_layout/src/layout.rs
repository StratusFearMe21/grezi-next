use std::{collections::HashMap, iter};

use cassowary::{
    AddConstraintError, Expression, Solver, Variable,
    WeightedRelation::{EQ, GE, LE},
    strength::REQUIRED,
};
use emath::{Pos2, Rect, Vec2};
use itertools::Itertools;
use smallvec::SmallVec;

use self::strengths::{
    ALL_SEGMENT_GROW, FILL_GROW, GROW, LENGTH_SIZE_EQ, MAX_SIZE_EQ, MAX_SIZE_LE, MIN_SIZE_EQ,
    MIN_SIZE_GE, PERCENTAGE_SIZE_EQ, RATIO_SIZE_EQ, SPACE_GROW, SPACER_SIZE_EQ,
};
use crate::{Constraint, Direction, Flex};

// The solution to a Layout solve contains two `Rects`, where `Rects` is effectively a `[Rect]`.
//
// 1. `[Rect]` that contains positions for the segments corresponding to user provided constraints
// 2. `[Rect]` that contains spacers around the user provided constraints
//
// <------------------------------------80 px------------------------------------->
// ┌   ┐┌──────────────────┐┌   ┐┌──────────────────┐┌   ┐┌──────────────────┐┌   ┐
//   1  │        a         │  2  │         b        │  3  │         c        │  4
// └   ┘└──────────────────┘└   ┘└──────────────────┘└   ┘└──────────────────┘└   ┘
//
// Number of spacers will always be one more than number of segments.
type Rects = SmallVec<[Rect; 4]>;
type Segments = Rects;
type Spacers = Rects;

/// Represents the spacing between segments in a layout.
///
/// The `Spacing` enum is used to define the spacing between segments in a layout. It can represent
/// either positive spacing (space between segments) or negative spacing (overlap between segments).
///
/// # Variants
///
/// - `Space(u16)`: Represents positive spacing between segments. The value indicates the number of
///   cells.
/// - `Overlap(u16)`: Represents negative spacing, causing overlap between segments. The value
///   indicates the number of overlapping cells.
///
/// # Default
///
/// The default value for `Spacing` is `Space(0)`, which means no spacing or no overlap between
/// segments.
///
/// # Conversions
///
/// The `Spacing` enum can be created from different integer types:
///
/// - From `u16`: Directly converts the value to `Spacing::Space`.
/// - From `i16`: Converts negative values to `Spacing::Overlap` and non-negative values to
///   `Spacing::Space`.
/// - From `i32`: Clamps the value to the range of `i16` and converts negative values to
///   `Spacing::Overlap` and non-negative values to `Spacing::Space`.
///
/// See the [`Layout::spacing`] method for details on how to use this enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Spacing {
    Space(f64),
    Overlap(f64),
}

impl From<f64> for Spacing {
    fn from(value: f64) -> Self {
        if value < 0.0 {
            Self::Overlap(value.abs())
        } else {
            Self::Space(value.abs())
        }
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::Space(0.0)
    }
}

/// A layout is a set of constraints that can be applied to a given area to split it into smaller
/// ones.
///
/// A layout is composed of:
/// - a direction (horizontal or vertical)
/// - a set of constraints (length, ratio, percentage, fill, min, max)
/// - a margin (horizontal and vertical), the space between the edge of the main area and the split
///   areas
/// - a flex option
/// - a spacing option
///
/// The algorithm used to compute the layout is based on the [`cassowary`] solver. It is a simple
/// linear solver that can be used to solve linear equations and inequalities. In our case, we
/// define a set of constraints that are applied to split the provided area into Rects aligned in a
/// single direction, and the solver computes the values of the position and sizes that satisfy as
/// many of the constraints in order of their priorities.
///
/// When the layout is computed, the result is cached in a thread-local cache, so that subsequent
/// calls with the same parameters are faster. The cache is a `LruCache`, and the size of the cache
/// can be configured using [`Layout::init_cache()`].
///
/// # Constructors
///
/// There are four ways to create a new layout:
///
/// - [`Layout::default`]: create a new layout with default values
/// - [`Layout::new`]: create a new layout with a given direction and constraints
/// - [`Layout::vertical`]: create a new vertical layout with the given constraints
/// - [`Layout::horizontal`]: create a new horizontal layout with the given constraints
///
/// # Setters
///
/// There are several setters to modify the layout:
///
/// - [`Layout::direction`]: set the direction of the layout
/// - [`Layout::constraints`]: set the constraints of the layout
/// - [`Layout::margin`]: set the margin of the layout
/// - [`Layout::horizontal_margin`]: set the horizontal margin of the layout
/// - [`Layout::vertical_margin`]: set the vertical margin of the layout
/// - [`Layout::flex`]: set the way the space is distributed when the constraints are satisfied
/// - [`Layout::spacing`]: sets the gap between the constraints of the layout
///
/// # Example
///
/// ```rust
/// use ratatui_core::{
///     buffer::Buffer,
///     layout::{Constraint, Direction, Layout, Rect},
///     text::Text,
///     widgets::Widget,
/// };
///
/// fn render(area: Rect, buf: &mut ratatui_core::buffer::Buffer) {
///     let layout = Layout::vertical([Constraint::Length(5), Constraint::Min(0)]);
///     let [left, right] = layout.areas(area);
///     Text::from("foo").render(left, buf);
///     Text::from("bar").render(right, buf);
/// }
/// ```
///
/// See the `layout`, `flex`, and `constraints` examples in the [Examples] folder for more details
/// about how to use layouts.
///
/// ![layout
/// example](https://camo.githubusercontent.com/77d22f3313b782a81e5e033ef82814bb48d786d2598699c27f8e757ccee62021/68747470733a2f2f7668732e636861726d2e73682f7668732d315a4e6f4e4c4e6c4c746b4a58706767396e435635652e676966)
///
/// [`cassowary`]: https://crates.io/crates/cassowary
/// [Examples]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Layout<'a> {
    direction: Direction,
    constraints: &'a [Constraint],
    margin: Vec2,
    margin_per: Vec2,
    flex: Flex,
    spacing: Spacing,
}

impl<'a> Layout<'a> {
    /// Creates a new layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on `u16`, so you can pass an array, `Vec`, etc. of `u16` to this function to
    /// create a layout with fixed size chunks.
    ///
    /// Default values for the other fields are:
    ///
    /// - `margin`: 0, 0
    /// - `flex`: [`Flex::Start`]
    /// - `spacing`: 0
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Direction, Layout};
    ///
    /// Layout::new(
    ///     Direction::Horizontal,
    ///     [Constraint::Length(5), Constraint::Min(0)],
    /// );
    ///
    /// Layout::new(
    ///     Direction::Vertical,
    ///     [1, 2, 3].iter().map(|&c| Constraint::Length(c)),
    /// );
    ///
    /// Layout::new(Direction::Horizontal, vec![1, 2]);
    /// ```
    pub fn new(direction: Direction, constraints: &'a [Constraint]) -> Self {
        Self {
            direction,
            constraints,
            ..Self::default()
        }
    }

    /// Set the direction of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Direction, Layout, Rect};
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 5, 10), Rect::new(5, 0, 5, 10)]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 10, 5), Rect::new(0, 5, 10, 5)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the constraints of the layout.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array or vec of u16 to this function to create a
    /// layout with fixed size chunks.
    ///
    /// Note that the constraints are applied to the whole area that is to be split, so using
    /// percentages and ratios with the other constraints may not have the desired effect of
    /// splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in [20,
    /// 40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    ///
    /// let layout = Layout::default()
    ///     .constraints([
    ///         Constraint::Percentage(20),
    ///         Constraint::Ratio(1, 5),
    ///         Constraint::Length(2),
    ///         Constraint::Min(2),
    ///         Constraint::Max(2),
    ///     ])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(
    ///     layout[..],
    ///     [
    ///         Rect::new(0, 0, 10, 2),
    ///         Rect::new(0, 2, 10, 2),
    ///         Rect::new(0, 4, 10, 2),
    ///         Rect::new(0, 6, 10, 2),
    ///         Rect::new(0, 8, 10, 2),
    ///     ]
    /// );
    ///
    /// Layout::default().constraints([Constraint::Min(0)]);
    /// Layout::default().constraints(&[Constraint::Min(0)]);
    /// Layout::default().constraints(vec![Constraint::Min(0)]);
    /// Layout::default().constraints([Constraint::Min(0)].iter().filter(|_| true));
    /// Layout::default().constraints([1, 2, 3].iter().map(|&c| Constraint::Length(c)));
    /// Layout::default().constraints([1, 2, 3]);
    /// Layout::default().constraints(vec![1, 2, 3]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn constraints(mut self, constraints: &'a [Constraint]) -> Self {
        self.constraints = constraints;
        self
    }

    /// Set the margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    ///
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 6, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn margin(mut self, margin: f32) -> Self {
        self.margin = Vec2::splat(margin);
        self
    }

    /// Set the horizontal margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    ///
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .horizontal_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 0, 6, 10)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn horizontal_margin(mut self, horizontal: f32) -> Self {
        self.margin.x = horizontal;
        self
    }

    /// Set the vertical margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    ///
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .vertical_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 2, 10, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn vertical_margin(mut self, vertical: f32) -> Self {
        self.margin.y = vertical;
        self
    }

    /// The `flex` method  allows you to specify the flex behavior of the layout.
    ///
    /// # Arguments
    ///
    /// * `flex`: A [`Flex`] enum value that represents the flex behavior of the layout. It can be
    ///   one of the following:
    ///   - [`Flex::Legacy`]: The last item is stretched to fill the excess space.
    ///   - [`Flex::Start`]: The items are aligned to the start of the layout.
    ///   - [`Flex::Center`]: The items are aligned to the center of the layout.
    ///   - [`Flex::End`]: The items are aligned to the end of the layout.
    ///   - [`Flex::SpaceAround`]: The items are evenly distributed with equal space around them.
    ///   - [`Flex::SpaceBetween`]: The items are evenly distributed with equal space between them.
    ///
    /// # Examples
    ///
    /// In this example, the items in the layout will be aligned to the start.
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint::*, Flex, Layout};
    ///
    /// let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).flex(Flex::Start);
    /// ```
    ///
    /// In this example, the items in the layout will be stretched equally to fill the available
    /// space.
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint::*, Flex, Layout};
    ///
    /// let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).flex(Flex::Legacy);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn margin_per(mut self, margin_per: f32) -> Self {
        self.margin_per = Vec2::splat(margin_per);
        self
    }

    /// Sets the spacing between items in the layout.
    ///
    /// The `spacing` method sets the spacing between items in the layout. The spacing is applied
    /// evenly between all segments. The spacing value represents the number of cells between each
    /// item.
    ///
    /// Spacing can be positive integers, representing gaps between segments; or negative integers
    /// representing overlaps. Additionally, one of the variants of the [`Spacing`] enum can be
    /// passed to this function. See the documentation of the [`Spacing`] enum for more information.
    ///
    /// Note that if the layout has only one segment, the spacing will not be applied.
    /// Also, spacing will not be applied for [`Flex::SpaceAround`] and [`Flex::SpaceBetween`]
    ///
    /// # Examples
    ///
    /// In this example, the spacing between each item in the layout is set to 2 cells.
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint::*, Layout};
    ///
    /// let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).spacing(2);
    /// ```
    ///
    /// In this example, the spacing between each item in the layout is set to -1 cells, i.e. the
    /// three segments will have an overlapping border.
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint::*, Layout};
    /// let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).spacing(-1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn spacing<T>(mut self, spacing: T) -> Self
    where
        T: Into<Spacing>,
    {
        self.spacing = spacing.into();
        self
    }

    pub fn all_ratios(&self) -> bool {
        self.constraints.iter().all(|constraint| {
            matches!(
                constraint,
                Constraint::Ratio(_, _) | Constraint::Percentage(_)
            )
        })
    }

    pub fn try_split(&self, area: Rect) -> Result<(Segments, Spacers), AddConstraintError> {
        let inner_area = area.shrink2(self.margin);
        // This is the most common layout type used in slides
        // (by me, the dev). If all the constrains are just ratios,
        // we can just early-out and skip the overhead of the constraint
        // solver
        if self.all_ratios() && self.flex == Flex::Legacy && self.spacing == Spacing::Space(0.0) {
            let mut segments: SmallVec<[Rect; 4]> = SmallVec::with_capacity(self.constraints.len());
            let mut last_segment = Rect::from_min_size(inner_area.min, Vec2::ZERO);

            for constraint in self.constraints {
                let ratio = match constraint {
                    Constraint::Ratio(num, den) => num / den,
                    Constraint::Percentage(percentage) => percentage / 100.0,
                    _ => unreachable!(),
                };

                match self.direction {
                    Direction::Horizontal => {
                        last_segment.max.y = inner_area.max.y;
                        last_segment.set_width(inner_area.width() * ratio as f32);
                        segments.push(last_segment.shrink2(self.margin_per));
                        last_segment.min.x = last_segment.max.x;
                    }
                    Direction::Vertical => {
                        last_segment.max.x = inner_area.max.x;
                        last_segment.set_height(inner_area.height() * ratio as f32);
                        segments.push(last_segment.shrink2(self.margin_per));
                        last_segment.min.y = last_segment.max.y;
                    }
                }
            }

            if let Some(segment) = segments.last_mut() {
                segment.max = inner_area.max;
            }

            return Ok((segments, SmallVec::new()));
        }
        // To take advantage of all of cassowary features, we would want to store the `Solver` in
        // one of the fields of the Layout struct. And we would want to set it up such that we could
        // add or remove constraints as and when needed.
        // The advantage of doing it as described above is that it would allow users to
        // incrementally add and remove constraints efficiently.
        // Solves will just one constraint different would not need to resolve the entire layout.
        //
        // The disadvantage of this approach is that it requires tracking which constraints were
        // added, and which variables they correspond to.
        // This will also require introducing and maintaining the API for users to do so.
        //
        // Currently we don't support that use case and do not intend to support it in the future,
        // and instead we require that the user re-solve the layout every time they call `split`.
        // To minimize the time it takes to solve the same problem over and over again, we
        // cache the `Layout` struct along with the results.
        //
        // `try_split` is the inner method in `split` that is called only when the LRU cache doesn't
        // match the key. So inside `try_split`, we create a new instance of the solver.
        //
        // This is equivalent to storing the solver in `Layout` and calling `solver.reset()` here.
        let mut solver = Solver::new();

        let (area_start, area_end) = match self.direction {
            Direction::Horizontal => (f64::from(inner_area.min.x), f64::from(inner_area.right())),
            Direction::Vertical => (f64::from(inner_area.min.y), f64::from(inner_area.bottom())),
        };

        // ```plain
        // <───────────────────────────────────area_size──────────────────────────────────>
        // ┌─area_start                                                          area_end─┐
        // V                                                                              V
        // ┌────┬───────────────────┬────┬─────variables─────┬────┬───────────────────┬────┐
        // │    │                   │    │                   │    │                   │    │
        // V    V                   V    V                   V    V                   V    V
        // ┌   ┐┌──────────────────┐┌   ┐┌──────────────────┐┌   ┐┌──────────────────┐┌   ┐
        //      │     Max(20)      │     │      Max(20)     │     │      Max(20)     │
        // └   ┘└──────────────────┘└   ┘└──────────────────┘└   ┘└──────────────────┘└   ┘
        // ^    ^                   ^    ^                   ^    ^                   ^    ^
        // │    │                   │    │                   │    │                   │    │
        // └─┬──┶━━━━━━━━━┳━━━━━━━━━┵─┬──┶━━━━━━━━━┳━━━━━━━━━┵─┬──┶━━━━━━━━━┳━━━━━━━━━┵─┬──┘
        //   │            ┃           │            ┃           │            ┃           │
        //   └────────────╂───────────┴────────────╂───────────┴────────────╂──Spacers──┘
        //                ┃                        ┃                        ┃
        //                ┗━━━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━Segments━━━━━━━━┛
        // ```

        let variable_count = self.constraints.len() * 2 + 2;
        let variables = iter::repeat_with(Variable::new)
            .take(variable_count)
            .collect_vec();
        let spacers = variables
            .iter()
            .tuples()
            .map(|(a, b)| Element::from((*a, *b)))
            .collect_vec();
        let segments = variables
            .iter()
            .skip(1)
            .tuples()
            .map(|(a, b)| Element::from((*a, *b)))
            .collect_vec();

        let flex = self.flex;

        let spacing = match self.spacing {
            Spacing::Space(x) => x,
            Spacing::Overlap(x) => -x,
        };

        let constraints = self.constraints;

        let area_size = Element::from((*variables.first().unwrap(), *variables.last().unwrap()));
        configure_area(&mut solver, area_size, area_start, area_end)?;
        configure_variable_in_area_constraints(&mut solver, &variables, area_size)?;
        configure_variable_constraints(&mut solver, &variables)?;
        configure_flex_constraints(&mut solver, area_size, &spacers, flex, spacing)?;
        configure_constraints(&mut solver, area_size, &segments, constraints, flex)?;
        configure_fill_constraints(&mut solver, &segments, constraints, flex)?;

        if flex != Flex::Legacy {
            for (left, right) in segments.iter().tuple_windows() {
                solver.add_constraint(left.has_size(right, ALL_SEGMENT_GROW))?;
            }
        }

        // `solver.fetch_changes()` can only be called once per solve
        let changes: HashMap<Variable, f64> = solver.fetch_changes().iter().copied().collect();
        // debug_elements(&segments, &changes);
        // debug_elements(&spacers, &changes);

        let segment_rects = changes_to_rects(
            &changes,
            &segments,
            inner_area,
            self.direction,
            self.margin_per,
        );
        let spacer_rects = changes_to_rects(
            &changes,
            &spacers,
            inner_area,
            self.direction,
            self.margin_per,
        );

        Ok((segment_rects, spacer_rects))
    }
}

fn configure_area(
    solver: &mut Solver,
    area: Element,
    area_start: f64,
    area_end: f64,
) -> Result<(), AddConstraintError> {
    solver.add_constraint(area.start | EQ(REQUIRED) | area_start)?;
    solver.add_constraint(area.end | EQ(REQUIRED) | area_end)?;
    Ok(())
}

fn configure_variable_in_area_constraints(
    solver: &mut Solver,
    variables: &[Variable],
    area: Element,
) -> Result<(), AddConstraintError> {
    // all variables are in the range [area.start, area.end]
    for &variable in variables {
        solver.add_constraint(variable | GE(REQUIRED) | area.start)?;
        solver.add_constraint(variable | LE(REQUIRED) | area.end)?;
    }

    Ok(())
}

fn configure_variable_constraints(
    solver: &mut Solver,
    variables: &[Variable],
) -> Result<(), AddConstraintError> {
    // ┌────┬───────────────────┬────┬─────variables─────┬────┬───────────────────┬────┐
    // │    │                   │    │                   │    │                   │    │
    // v    v                   v    v                   v    v                   v    v
    // ┌   ┐┌──────────────────┐┌   ┐┌──────────────────┐┌   ┐┌──────────────────┐┌   ┐
    //      │     Max(20)      │     │      Max(20)     │     │      Max(20)     │
    // └   ┘└──────────────────┘└   ┘└──────────────────┘└   ┘└──────────────────┘└   ┘
    // ^    ^                   ^    ^                   ^    ^                   ^    ^
    // └v0  └v1                 └v2  └v3                 └v4  └v5                 └v6  └v7

    for (&left, &right) in variables.iter().skip(1).tuples() {
        solver.add_constraint(left | LE(REQUIRED) | right)?;
    }
    Ok(())
}

fn configure_constraints(
    solver: &mut Solver,
    area: Element,
    segments: &[Element],
    constraints: &[Constraint],
    flex: Flex,
) -> Result<(), AddConstraintError> {
    for (&constraint, &segment) in constraints.iter().zip(segments.iter()) {
        match constraint {
            Constraint::Max(max) => {
                solver.add_constraint(segment.has_max_size(max, MAX_SIZE_LE))?;
                solver.add_constraint(segment.has_int_size(max, MAX_SIZE_EQ))?;
            }
            Constraint::Min(min) => {
                solver.add_constraint(segment.has_min_size(min, MIN_SIZE_GE))?;
                if flex == Flex::Legacy {
                    solver.add_constraint(segment.has_int_size(min, MIN_SIZE_EQ))?;
                } else {
                    solver.add_constraint(segment.has_size(area, FILL_GROW))?;
                }
            }
            Constraint::Length(length) => {
                solver.add_constraint(segment.has_int_size(length, LENGTH_SIZE_EQ))?;
            }
            Constraint::Percentage(p) => {
                let size = area.size() * p / 100.00;
                solver.add_constraint(segment.has_size(size, PERCENTAGE_SIZE_EQ))?;
            }
            Constraint::Ratio(num, den) => {
                // avoid division by zero by using 1 when denominator is 0
                let size = area.size() * num / den.max(1.0);
                solver.add_constraint(segment.has_size(size, RATIO_SIZE_EQ))?;
            }
            Constraint::Fill(_) => {
                // given no other constraints, this segment will grow as much as possible.
                solver.add_constraint(segment.has_size(area, FILL_GROW))?;
            }
        }
    }
    Ok(())
}

fn configure_flex_constraints(
    solver: &mut Solver,
    area: Element,
    spacers: &[Element],
    flex: Flex,
    spacing: f64,
) -> Result<(), AddConstraintError> {
    let spacers_except_first_and_last = spacers.get(1..spacers.len() - 1).unwrap_or(&[]);
    match flex {
        Flex::Legacy => {
            for spacer in spacers_except_first_and_last {
                solver.add_constraint(spacer.has_size(spacing, SPACER_SIZE_EQ))?;
            }
            if let (Some(first), Some(last)) = (spacers.first(), spacers.last()) {
                solver.add_constraint(first.is_empty())?;
                solver.add_constraint(last.is_empty())?;
            }
        }
        // all spacers are the same size and will grow to fill any remaining space after the
        // constraints are satisfied
        Flex::SpaceAround => {
            for (left, right) in spacers.iter().tuple_combinations() {
                solver.add_constraint(left.has_size(right, SPACER_SIZE_EQ))?;
            }
            for spacer in spacers {
                solver.add_constraint(spacer.has_min_size(spacing, SPACER_SIZE_EQ))?;
                solver.add_constraint(spacer.has_size(area, SPACE_GROW))?;
            }
        }

        // all spacers are the same size and will grow to fill any remaining space after the
        // constraints are satisfied, but the first and last spacers are zero size
        Flex::SpaceBetween => {
            for (left, right) in spacers_except_first_and_last.iter().tuple_combinations() {
                solver.add_constraint(left.has_size(right.size(), SPACER_SIZE_EQ))?;
            }
            for spacer in spacers_except_first_and_last {
                solver.add_constraint(spacer.has_min_size(spacing, SPACER_SIZE_EQ))?;
                solver.add_constraint(spacer.has_size(area, SPACE_GROW))?;
            }
            if let (Some(first), Some(last)) = (spacers.first(), spacers.last()) {
                solver.add_constraint(first.is_empty())?;
                solver.add_constraint(last.is_empty())?;
            }
        }
        Flex::Start => {
            for spacer in spacers_except_first_and_last {
                solver.add_constraint(spacer.has_size(spacing, SPACER_SIZE_EQ))?;
            }
            if let (Some(first), Some(last)) = (spacers.first(), spacers.last()) {
                solver.add_constraint(first.is_empty())?;
                solver.add_constraint(last.has_size(area, GROW))?;
            }
        }
        Flex::Center => {
            for spacer in spacers_except_first_and_last {
                solver.add_constraint(spacer.has_size(spacing, SPACER_SIZE_EQ))?;
            }
            if let (Some(first), Some(last)) = (spacers.first(), spacers.last()) {
                solver.add_constraint(first.has_size(area, GROW))?;
                solver.add_constraint(last.has_size(area, GROW))?;
                solver.add_constraint(first.has_size(last, SPACER_SIZE_EQ))?;
            }
        }
        Flex::End => {
            for spacer in spacers_except_first_and_last {
                solver.add_constraint(spacer.has_size(spacing, SPACER_SIZE_EQ))?;
            }
            if let (Some(first), Some(last)) = (spacers.first(), spacers.last()) {
                solver.add_constraint(last.is_empty())?;
                solver.add_constraint(first.has_size(area, GROW))?;
            }
        }
    }
    Ok(())
}

/// Make every `Fill` constraint proportionally equal to each other
/// This will make it fill up empty spaces equally
///
/// [Fill(1), Fill(1)]
/// ┌──────┐┌──────┐
/// │abcdef││abcdef│
/// └──────┘└──────┘
///
/// [Min(0), Fill(2)]
/// ┌──────┐┌────────────┐
/// │abcdef││abcdefabcdef│
/// └──────┘└────────────┘
///
/// `size == base_element * scaling_factor`
fn configure_fill_constraints(
    solver: &mut Solver,
    segments: &[Element],
    constraints: &[Constraint],
    flex: Flex,
) -> Result<(), AddConstraintError> {
    for ((&left_constraint, &left_segment), (&right_constraint, &right_segment)) in constraints
        .iter()
        .zip(segments.iter())
        .filter(|(c, _)| {
            matches!(c, Constraint::Fill(_))
                || (flex != Flex::Legacy && matches!(c, Constraint::Min(_)))
        })
        .tuple_combinations()
    {
        let left_scaling_factor = match left_constraint {
            Constraint::Fill(scale) => scale.max(1e-6),
            Constraint::Min(_) => 1.0,
            _ => unreachable!(),
        };
        let right_scaling_factor = match right_constraint {
            Constraint::Fill(scale) => scale.max(1e-6),
            Constraint::Min(_) => 1.0,
            _ => unreachable!(),
        };
        solver.add_constraint(
            (right_scaling_factor * left_segment.size())
                | EQ(GROW)
                | (left_scaling_factor * right_segment.size()),
        )?;
    }
    Ok(())
}

fn changes_to_rects(
    changes: &HashMap<Variable, f64>,
    elements: &[Element],
    area: Rect,
    direction: Direction,
    margin_per: Vec2,
) -> Rects {
    // convert to Rects
    elements
        .iter()
        .map(|element| {
            let start = changes.get(&element.start).unwrap_or(&0.0);
            let end = changes.get(&element.end).unwrap_or(&0.0);
            let size = (end - start).max(0.0);
            match direction {
                Direction::Horizontal => Rect::from_min_size(
                    Pos2::new(*start as f32, area.min.y),
                    Vec2::new(size as f32, area.height()),
                )
                .shrink2(margin_per),
                Direction::Vertical => Rect::from_min_size(
                    Pos2::new(area.min.x, *start as f32),
                    Vec2::new(area.width(), size as f32),
                )
                .shrink2(margin_per),
            }
        })
        .collect::<Rects>()
}

/// please leave this here as it's useful for debugging unit tests when we make any changes to
/// layout code - we should replace this with tracing in the future.
#[allow(dead_code)]
fn debug_elements(elements: &[Element], changes: &HashMap<Variable, f64>) {
    let variables = format!(
        "{:?}",
        elements
            .iter()
            .map(|e| (
                *changes.get(&e.start).unwrap_or(&0.0),
                *changes.get(&e.end).unwrap_or(&0.0),
            ))
            .collect::<Vec<(f64, f64)>>()
    );
    dbg!(variables);
}

/// A container used by the solver inside split
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Element {
    start: Variable,
    end: Variable,
}

impl From<(Variable, Variable)> for Element {
    fn from((start, end): (Variable, Variable)) -> Self {
        Self { start, end }
    }
}

impl Element {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            start: Variable::new(),
            end: Variable::new(),
        }
    }

    fn size(&self) -> Expression {
        self.end - self.start
    }

    fn has_max_size(&self, size: f64, strength: f64) -> cassowary::Constraint {
        self.size() | LE(strength) | size
    }

    fn has_min_size(&self, size: f64, strength: f64) -> cassowary::Constraint {
        self.size() | GE(strength) | size
    }

    fn has_int_size(&self, size: f64, strength: f64) -> cassowary::Constraint {
        self.size() | EQ(strength) | size
    }

    fn has_size<E: Into<Expression>>(&self, size: E, strength: f64) -> cassowary::Constraint {
        self.size() | EQ(strength) | size.into()
    }

    fn is_empty(&self) -> cassowary::Constraint {
        self.size() | EQ(REQUIRED - 1.0) | 0.0
    }
}

/// allow the element to represent its own size in expressions
impl From<Element> for Expression {
    fn from(element: Element) -> Self {
        element.size()
    }
}

/// allow the element to represent its own size in expressions
impl From<&Element> for Expression {
    fn from(element: &Element) -> Self {
        element.size()
    }
}

mod strengths {
    use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};

    /// The strength to apply to Spacers to ensure that their sizes are equal.
    ///
    /// ┌     ┐┌───┐┌     ┐┌───┐┌     ┐
    ///   ==x  │   │  ==x  │   │  ==x
    /// └     ┘└───┘└     ┘└───┘└     ┘
    pub const SPACER_SIZE_EQ: f64 = REQUIRED / 10.0;

    /// The strength to apply to Min inequality constraints.
    ///
    /// ┌────────┐
    /// │Min(>=x)│
    /// └────────┘
    pub const MIN_SIZE_GE: f64 = STRONG * 100.0;

    /// The strength to apply to Max inequality constraints.
    ///
    /// ┌────────┐
    /// │Max(<=x)│
    /// └────────┘
    pub const MAX_SIZE_LE: f64 = STRONG * 100.0;

    /// The strength to apply to Length constraints.
    ///
    /// ┌───────────┐
    /// │Length(==x)│
    /// └───────────┘
    pub const LENGTH_SIZE_EQ: f64 = STRONG * 10.0;

    /// The strength to apply to Percentage constraints.
    ///
    /// ┌───────────────┐
    /// │Percentage(==x)│
    /// └───────────────┘
    pub const PERCENTAGE_SIZE_EQ: f64 = STRONG;

    /// The strength to apply to Ratio constraints.
    ///
    /// ┌────────────┐
    /// │Ratio(==x,y)│
    /// └────────────┘
    pub const RATIO_SIZE_EQ: f64 = STRONG / 10.0;

    /// The strength to apply to Min equality constraints.
    ///
    /// ┌────────┐
    /// │Min(==x)│
    /// └────────┘
    pub const MIN_SIZE_EQ: f64 = MEDIUM * 10.0;

    /// The strength to apply to Max equality constraints.
    ///
    /// ┌────────┐
    /// │Max(==x)│
    /// └────────┘
    pub const MAX_SIZE_EQ: f64 = MEDIUM * 10.0;

    /// The strength to apply to Fill growing constraints.
    ///
    /// ┌─────────────────────┐
    /// │<=     Fill(x)     =>│
    /// └─────────────────────┘
    pub const FILL_GROW: f64 = MEDIUM;

    /// The strength to apply to growing constraints.
    ///
    /// ┌────────────┐
    /// │<= Min(x) =>│
    /// └────────────┘
    pub const GROW: f64 = MEDIUM / 10.0;

    /// The strength to apply to Spacer growing constraints.
    ///
    /// ┌       ┐
    ///  <= x =>
    /// └       ┘
    pub const SPACE_GROW: f64 = WEAK * 10.0;

    /// The strength to apply to growing the size of all segments equally.
    ///
    /// ┌───────┐
    /// │<= x =>│
    /// └───────┘
    pub const ALL_SEGMENT_GROW: f64 = WEAK;
}

/*
                     ___
                    /\_ \
 _ __    __     __  \//\ \         ___ ___      __    ___
/\`'__\/'__`\ /'__`\  \ \ \      /' __` __`\  /'__`\/' _ `\
\ \ \//\  __//\ \L\.\_ \_\ \_    /\ \/\ \/\ \/\  __//\ \/\ \
 \ \_\\ \____\ \__/.\_\/\____\   \ \_\ \_\ \_\ \____\ \_\ \_\
  \/_/ \/____/\/__/\/_/\/____/    \/_/\/_/\/_/\/____/\/_/\/_/


 __                   __
/\ \__               /\ \__      __
\ \ ,_\    __    ____\ \ ,_\    /\_\    ___
 \ \ \/  /'__`\ /',__\\ \ \/    \/\ \ /' _ `\
  \ \ \_/\  __//\__, `\\ \ \_    \ \ \/\ \/\ \
   \ \__\ \____\/\____/ \ \__\    \ \_\ \_\ \_\
    \/__/\/____/\/___/   \/__/     \/_/\/_/\/_/


                        __                  __
                       /\ \                /\ \__  __
 _____   _ __   ___    \_\ \  __  __    ___\ \ ,_\/\_\    ___     ___
/\ '__`\/\`'__\/ __`\  /'_` \/\ \/\ \  /'___\ \ \/\/\ \  / __`\ /' _ `\
\ \ \L\ \ \ \//\ \L\ \/\ \L\ \ \ \_\ \/\ \__/\ \ \_\ \ \/\ \L\ \/\ \/\ \
 \ \ ,__/\ \_\\ \____/\ \___,_\ \____/\ \____\\ \__\\ \_\ \____/\ \_\ \_\
  \ \ \/  \/_/ \/___/  \/__,_ /\/___/  \/____/ \/__/ \/_/\/___/  \/_/\/_/
   \ \_\
    \/_/
*/

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     // The compiler will optimize out the comparisons, but this ensures that the constants are
//     // defined in the correct order of priority.
//     #[allow(clippy::assertions_on_constants)]
//     pub fn strength_is_valid() {
//         use strengths::*;
//         assert!(SPACER_SIZE_EQ > MAX_SIZE_LE);
//         assert!(MAX_SIZE_LE > MAX_SIZE_EQ);
//         assert!(MIN_SIZE_GE == MAX_SIZE_LE);
//         assert!(MAX_SIZE_LE > LENGTH_SIZE_EQ);
//         assert!(LENGTH_SIZE_EQ > PERCENTAGE_SIZE_EQ);
//         assert!(PERCENTAGE_SIZE_EQ > RATIO_SIZE_EQ);
//         assert!(RATIO_SIZE_EQ > MAX_SIZE_EQ);
//         assert!(MIN_SIZE_GE > FILL_GROW);
//         assert!(FILL_GROW > GROW);
//         assert!(GROW > SPACE_GROW);
//         assert!(SPACE_GROW > ALL_SEGMENT_GROW);
//     }

//     #[test]
//     fn default() {
//         assert_eq!(
//             Layout::default(),
//             Layout {
//                 direction: Direction::Vertical,
//                 margin: Vec2::new(0.0, 0.0),
//                 constraints: &[],
//                 flex: Flex::default(),
//                 spacing: Spacing::default(),
//             }
//         );
//     }

//     #[test]
//     fn new() {
//         // array
//         let fixed_size_array = [Constraint::Min(0.0)];
//         let layout = Layout::new(Direction::Horizontal, &fixed_size_array);
//         assert_eq!(layout.direction, Direction::Horizontal);
//         assert_eq!(layout.constraints, [Constraint::Min(0.0)]);

//         // array_ref
//         #[allow(clippy::needless_borrows_for_generic_args)] // backwards compatibility test
//         let layout = Layout::new(Direction::Horizontal, &[Constraint::Min(0.0)]);
//         assert_eq!(layout.direction, Direction::Horizontal);
//         assert_eq!(layout.constraints, [Constraint::Min(0.0)]);
//     }

//     /// The purpose of this test is to ensure that layout can be constructed with any type that
//     /// implements `IntoIterator<Item = AsRef<Constraint>>`.
//     #[test]
//     #[allow(
//         clippy::needless_borrow,
//         clippy::unnecessary_to_owned,
//         clippy::useless_asref
//     )]
//     fn constraints() {
//         const CONSTRAINTS: [Constraint; 2] = [Constraint::Min(0.0), Constraint::Max(10.0)];
//         let fixed_size_array = CONSTRAINTS;
//         assert_eq!(
//             Layout::default().constraints(&fixed_size_array).constraints,
//             CONSTRAINTS,
//             "constraints should be settable with an array"
//         );

//         let slice_of_fixed_size_array = &CONSTRAINTS;
//         assert_eq!(
//             Layout::default()
//                 .constraints(slice_of_fixed_size_array)
//                 .constraints,
//             CONSTRAINTS,
//             "constraints should be settable with a slice"
//         );
//     }

//     #[test]
//     fn direction() {
//         assert_eq!(
//             Layout::default().direction(Direction::Horizontal).direction,
//             Direction::Horizontal
//         );
//         assert_eq!(
//             Layout::default().direction(Direction::Vertical).direction,
//             Direction::Vertical
//         );
//     }

//     #[test]
//     fn margins() {
//         assert_eq!(Layout::default().margin(10.0).margin, Vec2::new(10.0, 10.0));
//         assert_eq!(
//             Layout::default().horizontal_margin(10.0).margin,
//             Vec2::new(10.0, 0.0)
//         );
//         assert_eq!(
//             Layout::default().vertical_margin(10.0).margin,
//             Vec2::new(0.0, 10.0)
//         );
//         assert_eq!(
//             Layout::default()
//                 .horizontal_margin(10.0)
//                 .vertical_margin(20.0)
//                 .margin,
//             Vec2::new(10.0, 20.0)
//         );
//     }

//     #[test]
//     fn flex() {
//         assert_eq!(Layout::default().flex, Flex::Start);
//         assert_eq!(Layout::default().flex(Flex::Center).flex, Flex::Center);
//     }

//     #[test]
//     fn spacing() {
//         assert_eq!(
//             Layout::default().spacing(10.0).spacing,
//             Spacing::Space(10.0)
//         );
//         assert_eq!(Layout::default().spacing(0.0).spacing, Spacing::Space(0.0));
//         assert_eq!(
//             Layout::default().spacing(-10.0).spacing,
//             Spacing::Overlap(10.0)
//         );
//     }

//     /// Tests for the `Layout::split()` function.
//     ///
//     /// There are many tests in this as the number of edge cases that are caused by the interaction
//     /// between the constraints is quite large. The tests are split into sections based on the type
//     /// of constraints that are used.
//     ///
//     /// These tests are characterization tests. This means that they are testing the way the code
//     /// currently works, and not the way it should work. This is because the current behavior is not
//     /// well defined, and it is not clear what the correct behavior should be. This means that if
//     /// the behavior changes, these tests should be updated to match the new behavior.
//     ///
//     ///  EOL comments in each test are intended to communicate the purpose of each test and to make
//     ///  it easy to see that the tests are as exhaustive as feasible:
//     /// - zero: constraint is zero
//     /// - exact: constraint is equal to the space
//     /// - underflow: constraint is for less than the full space
//     /// - overflow: constraint is for more than the full space
//     mod split {
//         use std::ops::Range;

//         use emath::{Pos2, Rect, Vec2};
//         use itertools::Itertools;
//         use pretty_assertions::assert_eq;
//         use rstest::rstest;

//         use crate::{
//             Constraint::{self, *},
//             Direction, Flex, Layout,
//         };

//         #[test]
//         fn vertical_split_by_height() {
//             let target = Rect {
//                 x: 2,
//                 y: 2,
//                 width: 10,
//                 height: 10,
//             };

//             let chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .constraints([
//                     Constraint::Percentage(10),
//                     Constraint::Max(5),
//                     Constraint::Min(1),
//                 ])
//                 .split(target);

//             assert_eq!(chunks.iter().map(|r| r.height).sum::<u16>(), target.height);
//             chunks.windows(2).for_each(|w| assert!(w[0].y <= w[1].y));
//         }

//         #[test]
//         fn edge_cases() {
//             // stretches into last
//             let layout = Layout::default()
//                 .constraints([
//                     Constraint::Percentage(50),
//                     Constraint::Percentage(50),
//                     Constraint::Min(0),
//                 ])
//                 .split(Rect::new(0, 0, 1, 1));
//             assert_eq!(
//                 layout[..],
//                 [
//                     Rect::new(0, 0, 1, 1),
//                     Rect::new(0, 1, 1, 0),
//                     Rect::new(0, 1, 1, 0)
//                 ]
//             );

//             // stretches into last
//             let layout = Layout::default()
//                 .constraints([
//                     Constraint::Max(1),
//                     Constraint::Percentage(99),
//                     Constraint::Min(0),
//                 ])
//                 .split(Rect::new(0, 0, 1, 1));
//             assert_eq!(
//                 layout[..],
//                 [
//                     Rect::new(0, 0, 1, 0),
//                     Rect::new(0, 0, 1, 1),
//                     Rect::new(0, 1, 1, 0)
//                 ]
//             );

//             // minimal bug from
//             // https://github.com/ratatui/ratatui/pull/404#issuecomment-1681850644
//             // TODO: check if this bug is now resolved?
//             let layout = Layout::default()
//                 .constraints([Min(1), Length(0), Min(1)])
//                 .direction(Direction::Horizontal)
//                 .split(Rect::new(0, 0, 1, 1));
//             assert_eq!(
//                 layout[..],
//                 [
//                     Rect::new(0, 0, 1, 1),
//                     Rect::new(1, 0, 0, 1),
//                     Rect::new(1, 0, 0, 1),
//                 ]
//             );

//             // This stretches the 2nd last length instead of the last min based on ranking
//             let layout = Layout::default()
//                 .constraints([Length(3), Min(4), Length(1), Min(4)])
//                 .direction(Direction::Horizontal)
//                 .split(Rect::new(0, 0, 7, 1));
//             assert_eq!(
//                 layout[..],
//                 [
//                     Rect::new(0, 0, 0, 1),
//                     Rect::new(0, 0, 4, 1),
//                     Rect::new(4, 0, 0, 1),
//                     Rect::new(4, 0, 3, 1),
//                 ]
//             );
//         }

//         #[rstest]
//         #[case::len_min1(vec![Length(25), Min(100)], vec![0..0,  0..100])]
//         #[case::len_min2(vec![Length(25), Min(0)], vec![0..25, 25..100])]
//         #[case::len_max1(vec![Length(25), Max(0)], vec![0..100, 100..100])]
//         #[case::len_max2(vec![Length(25), Max(100)], vec![0..25, 25..100])]
//         #[case::len_perc(vec![Length(25), Percentage(25)], vec![0..25, 25..100])]
//         #[case::perc_len(vec![Percentage(25), Length(25)], vec![0..75, 75..100])]
//         #[case::len_ratio(vec![Length(25), Ratio(1, 4)], vec![0..25, 25..100])]
//         #[case::ratio_len(vec![Ratio(1, 4), Length(25)], vec![0..75, 75..100])]
//         #[case::len_len(vec![Length(25), Length(25)], vec![0..25, 25..100])]
//         #[case::len1(vec![Length(25), Length(25), Length(25)], vec![0..25, 25..50, 50..100])]
//         #[case::len2(vec![Length(15), Length(35), Length(25)], vec![0..15, 15..50, 50..100])]
//         #[case::len3(vec![Length(25), Length(25), Length(25)], vec![0..25, 25..50, 50..100])]
//         fn constraint_length(
//             #[case] constraints: Vec<Constraint>,
//             #[case] expected: Vec<Range<u16>>,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .flex(Flex::Legacy)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect_vec();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case(7, vec![Length(4), Length(4)], vec![0..3, 4..7])]
//         #[case(4, vec![Length(4), Length(4)], vec![0..2, 3..4])]
//         fn table_length(
//             #[case] width: u16,
//             #[case] constraints: Vec<Constraint>,
//             #[case] expected: Vec<Range<u16>>,
//         ) {
//             let rect = Rect::new(0, 0, width, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .spacing(1)
//                 .flex(Flex::Start)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect::<Vec<Range<u16>>>();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case::min_len_max(vec![Min(25), Length(25), Max(25)], vec![0..50, 50..75, 75..100])]
//         #[case::max_len_min(vec![Max(25), Length(25), Min(25)], vec![0..25, 25..50, 50..100])]
//         #[case::len_len_len(vec![Length(33), Length(33), Length(33)], vec![0..33, 33..66, 66..100])]
//         #[case::len_len_len_25(vec![Length(25), Length(25), Length(25)], vec![0..25, 25..50, 50..100])]
//         #[case::perc_len_ratio(vec![Percentage(25), Length(25), Ratio(1, 4)], vec![0..25, 25..50, 50..100])]
//         #[case::len_ratio_perc(vec![Length(25), Ratio(1, 4), Percentage(25)], vec![0..25, 25..75, 75..100])]
//         #[case::ratio_len_perc(vec![Ratio(1, 4), Length(25), Percentage(25)], vec![0..50, 50..75, 75..100])]
//         #[case::ratio_perc_len(vec![Ratio(1, 4), Percentage(25), Length(25)], vec![0..50, 50..75, 75..100])]
//         #[case::len_len_min(vec![Length(100), Length(1), Min(20)], vec![0..80, 80..80, 80..100])]
//         #[case::min_len_len(vec![Min(20), Length(1), Length(100)], vec![0..20, 20..21, 21..100])]
//         #[case::fill_len_fill(vec![Fill(1), Length(10), Fill(1)], vec![0..45, 45..55, 55..100])]
//         #[case::fill_len_fill_2(vec![Fill(1), Length(10), Fill(2)], vec![0..30, 30..40, 40..100])]
//         #[case::fill_len_fill_4(vec![Fill(1), Length(10), Fill(4)], vec![0..18, 18..28, 28..100])]
//         #[case::fill_len_fill_5(vec![Fill(1), Length(10), Fill(5)], vec![0..15, 15..25, 25..100])]
//         #[case::len_len_len_25(vec![Length(25), Length(25), Length(25)], vec![0..25, 25..50, 50..100])]
//         #[case::unstable_test(vec![Length(25), Length(25), Length(25)], vec![0..25, 25..50, 50..100])]
//         fn length_is_higher_priority(
//             #[case] constraints: Vec<Constraint>,
//             #[case] expected: Vec<Range<u16>>,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .flex(Flex::Legacy)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect_vec();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case::min_len_max(vec![Min(25), Length(25), Max(25)], vec![50, 25, 25])]
//         #[case::max_len_min(vec![Max(25), Length(25), Min(25)], vec![25, 25, 50])]
//         #[case::len_len_len1(vec![Length(33), Length(33), Length(33)], vec![33, 33, 33])]
//         #[case::len_len_len2(vec![Length(25), Length(25), Length(25)], vec![25, 25, 25])]
//         #[case::perc_len_ratio(vec![Percentage(25), Length(25), Ratio(1, 4)], vec![25, 25, 25])]
//         #[case::len_ratio_perc(vec![Length(25), Ratio(1, 4), Percentage(25)], vec![25, 25, 25])]
//         #[case::ratio_len_perc(vec![Ratio(1, 4), Length(25), Percentage(25)], vec![25, 25, 25])]
//         #[case::ratio_perc_len(vec![Ratio(1, 4), Percentage(25), Length(25)], vec![25, 25, 25])]
//         #[case::len_len_min(vec![Length(100), Length(1), Min(20)], vec![79, 1, 20])]
//         #[case::min_len_len(vec![Min(20), Length(1), Length(100)], vec![20, 1, 79])]
//         #[case::fill_len_fill1(vec![Fill(1), Length(10), Fill(1)], vec![45, 10, 45])]
//         #[case::fill_len_fill2(vec![Fill(1), Length(10), Fill(2)], vec![30, 10, 60])]
//         #[case::fill_len_fill4(vec![Fill(1), Length(10), Fill(4)], vec![18, 10, 72])]
//         #[case::fill_len_fill5(vec![Fill(1), Length(10), Fill(5)], vec![15, 10, 75])]
//         #[case::len_len_len3(vec![Length(25), Length(25), Length(25)], vec![25, 25, 25])]
//         fn length_is_higher_priority_in_flex(
//             #[case] constraints: Vec<Constraint>,
//             #[case] expected: Vec<u16>,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             for flex in [
//                 Flex::Start,
//                 Flex::End,
//                 Flex::Center,
//                 Flex::SpaceAround,
//                 Flex::SpaceBetween,
//             ] {
//                 let widths = Layout::horizontal(&constraints)
//                     .flex(flex)
//                     .split(rect)
//                     .iter()
//                     .map(|r| r.width)
//                     .collect_vec();
//                 assert_eq!(widths, expected);
//             }
//         }

//         #[rstest]
//         #[case::fill_len_fill(vec![Fill(1), Length(10), Fill(2)], vec![0..13, 13..23, 23..50])]
//         #[case::len_fill_fill(vec![Length(10), Fill(2), Fill(1)], vec![0..10, 10..37, 37..50])] // might be unstable?
//         fn fixed_with_50_width(
//             #[case] constraints: Vec<Constraint>,
//             #[case] expected: Vec<Range<u16>>,
//         ) {
//             let rect = Rect::new(0, 0, 50, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .flex(Flex::Legacy)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect_vec();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case::same_fill(vec![Fill(1), Fill(2), Fill(1), Fill(1)], vec![0..20, 20..60, 60..80, 80..100])]
//         #[case::inc_fill(vec![Fill(1), Fill(2), Fill(3), Fill(4)], vec![0..10, 10..30, 30..60, 60..100])]
//         #[case::dec_fill(vec![Fill(4), Fill(3), Fill(2), Fill(1)], vec![0..40, 40..70, 70..90, 90..100])]
//         #[case::rand_fill1(vec![Fill(1), Fill(3), Fill(2), Fill(4)], vec![0..10, 10..40, 40..60, 60..100])]
//         #[case::rand_fill2(vec![Fill(1), Fill(3), Length(50), Fill(2), Fill(4)], vec![0..5, 5..20, 20..70, 70..80, 80..100])]
//         #[case::rand_fill3(vec![Fill(1), Fill(3), Percentage(50), Fill(2), Fill(4)], vec![0..5, 5..20, 20..70, 70..80, 80..100])]
//         #[case::rand_fill4(vec![Fill(1), Fill(3), Min(50), Fill(2), Fill(4)], vec![0..5, 5..20, 20..70, 70..80, 80..100])]
//         #[case::rand_fill5(vec![Fill(1), Fill(3), Max(50), Fill(2), Fill(4)], vec![0..5, 5..20, 20..70, 70..80, 80..100])]
//         #[case::zero_fill1(vec![Fill(0), Fill(1), Fill(0)], vec![0..0, 0..100, 100..100])]
//         #[case::zero_fill2(vec![Fill(0), Length(1), Fill(0)], vec![0..50, 50..51, 51..100])]
//         #[case::zero_fill3(vec![Fill(0), Percentage(1), Fill(0)], vec![0..50, 50..51, 51..100])]
//         #[case::zero_fill4(vec![Fill(0), Min(1), Fill(0)], vec![0..50, 50..51, 51..100])]
//         #[case::zero_fill5(vec![Fill(0), Max(1), Fill(0)], vec![0..50, 50..51, 51..100])]
//         #[case::zero_fill6(vec![Fill(0), Fill(2), Fill(0), Fill(1)], vec![0..0, 0..67, 67..67, 67..100])]
//         #[case::space_fill1(vec![Fill(0), Fill(2), Percentage(20)], vec![0..0, 0..80, 80..100])]
//         #[case::space_fill2(vec![Fill(0), Fill(0), Percentage(20)], vec![0..40, 40..80, 80..100])]
//         #[case::space_fill3(vec![Fill(0), Ratio(1, 5)], vec![0..80, 80..100])]
//         #[case::space_fill4(vec![Fill(0), Fill(u16::MAX)], vec![0..0, 0..100])]
//         #[case::space_fill5(vec![Fill(u16::MAX), Fill(0)], vec![0..100, 100..100])]
//         #[case::space_fill6(vec![Fill(0), Percentage(20)], vec![0..80, 80..100])]
//         #[case::space_fill7(vec![Fill(1), Percentage(20)], vec![0..80, 80..100])]
//         #[case::space_fill8(vec![Fill(u16::MAX), Percentage(20)], vec![0..80, 80..100])]
//         #[case::space_fill9(vec![Fill(u16::MAX), Fill(0), Percentage(20)], vec![0..80, 80..80, 80..100])]
//         #[case::space_fill10(vec![Fill(0), Length(20)], vec![0..80, 80..100])]
//         #[case::space_fill11(vec![Fill(0), Min(20)], vec![0..80, 80..100])]
//         #[case::space_fill12(vec![Fill(0), Max(20)], vec![0..80, 80..100])]
//         #[case::fill_collapse1(vec![Fill(1), Fill(1), Fill(1), Min(30), Length(50)], vec![0..7, 7..13, 13..20, 20..50, 50..100])]
//         #[case::fill_collapse2(vec![Fill(1), Fill(1), Fill(1), Length(50), Length(50)], vec![0..0, 0..0, 0..0, 0..50, 50..100])]
//         #[case::fill_collapse3(vec![Fill(1), Fill(1), Fill(1), Length(75), Length(50)], vec![0..0, 0..0, 0..0, 0..75, 75..100])]
//         #[case::fill_collapse4(vec![Fill(1), Fill(1), Fill(1), Min(50), Max(50)], vec![0..0, 0..0, 0..0, 0..50, 50..100])]
//         #[case::fill_collapse5(vec![Fill(1), Fill(1), Fill(1), Ratio(1, 1)], vec![0..0, 0..0, 0..0, 0..100])]
//         #[case::fill_collapse6(vec![Fill(1), Fill(1), Fill(1), Percentage(100)], vec![0..0, 0..0, 0..0, 0..100])]
//         fn fill(#[case] constraints: Vec<Constraint>, #[case] expected: Vec<Range<u16>>) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .flex(Flex::Legacy)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect_vec();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case::min_percentage(vec![Min(0), Percentage(20)], vec![0..80, 80..100])]
//         #[case::max_percentage(vec![Max(0), Percentage(20)], vec![0..0, 0..100])]
//         fn percentage_parameterized(
//             #[case] constraints: Vec<Constraint>,
//             #[case] expected: Vec<Range<u16>>,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .flex(Flex::Legacy)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect_vec();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case::max_min(vec![Max(100), Min(0)], vec![0..100, 100..100])]
//         #[case::min_max(vec![Min(0), Max(100)], vec![0..0, 0..100])]
//         #[case::length_min(vec![Length(u16::MAX), Min(10)], vec![0..90, 90..100])]
//         #[case::min_length(vec![Min(10), Length(u16::MAX)], vec![0..10, 10..100])]
//         #[case::length_max(vec![Length(0), Max(10)], vec![0..90, 90..100])]
//         #[case::max_length(vec![Max(10), Length(0)], vec![0..10, 10..100])]
//         fn min_max(#[case] constraints: Vec<Constraint>, #[case] expected: Vec<Range<u16>>) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .flex(Flex::Legacy)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect_vec();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case::length_legacy(vec![Length(50)], vec![0..100], Flex::Legacy)]
//         #[case::length_start(vec![Length(50)], vec![0..50], Flex::Start)]
//         #[case::length_end(vec![Length(50)], vec![50..100], Flex::End)]
//         #[case::length_center(vec![Length(50)], vec![25..75], Flex::Center)]
//         #[case::ratio_legacy(vec![Ratio(1, 2)], vec![0..100], Flex::Legacy)]
//         #[case::ratio_start(vec![Ratio(1, 2)], vec![0..50], Flex::Start)]
//         #[case::ratio_end(vec![Ratio(1, 2)], vec![50..100], Flex::End)]
//         #[case::ratio_center(vec![Ratio(1, 2)], vec![25..75], Flex::Center)]
//         #[case::percent_legacy(vec![Percentage(50)], vec![0..100], Flex::Legacy)]
//         #[case::percent_start(vec![Percentage(50)], vec![0..50], Flex::Start)]
//         #[case::percent_end(vec![Percentage(50)], vec![50..100], Flex::End)]
//         #[case::percent_center(vec![Percentage(50)], vec![25..75], Flex::Center)]
//         #[case::min_legacy(vec![Min(50)], vec![0..100], Flex::Legacy)]
//         #[case::min_start(vec![Min(50)], vec![0..100], Flex::Start)]
//         #[case::min_end(vec![Min(50)], vec![0..100], Flex::End)]
//         #[case::min_center(vec![Min(50)], vec![0..100], Flex::Center)]
//         #[case::max_legacy(vec![Max(50)], vec![0..100], Flex::Legacy)]
//         #[case::max_start(vec![Max(50)], vec![0..50], Flex::Start)]
//         #[case::max_end(vec![Max(50)], vec![50..100], Flex::End)]
//         #[case::max_center(vec![Max(50)], vec![25..75], Flex::Center)]
//         #[case::spacebetween_becomes_stretch1(vec![Min(1)], vec![0..100], Flex::SpaceBetween)]
//         #[case::spacebetween_becomes_stretch2(vec![Max(20)], vec![0..100], Flex::SpaceBetween)]
//         #[case::spacebetween_becomes_stretch3(vec![Length(20)], vec![0..100], Flex::SpaceBetween)]
//         #[case::length_legacy2(vec![Length(25), Length(25)], vec![0..25, 25..100], Flex::Legacy)]
//         #[case::length_start2(vec![Length(25), Length(25)], vec![0..25, 25..50], Flex::Start)]
//         #[case::length_center2(vec![Length(25), Length(25)], vec![25..50, 50..75], Flex::Center)]
//         #[case::length_end2(vec![Length(25), Length(25)], vec![50..75, 75..100], Flex::End)]
//         #[case::length_spacebetween(vec![Length(25), Length(25)], vec![0..25, 75..100], Flex::SpaceBetween)]
//         #[case::length_spacearound(vec![Length(25), Length(25)], vec![17..42, 58..83], Flex::SpaceAround)]
//         #[case::percentage_legacy(vec![Percentage(25), Percentage(25)], vec![0..25, 25..100], Flex::Legacy)]
//         #[case::percentage_start(vec![Percentage(25), Percentage(25)], vec![0..25, 25..50], Flex::Start)]
//         #[case::percentage_center(vec![Percentage(25), Percentage(25)], vec![25..50, 50..75], Flex::Center)]
//         #[case::percentage_end(vec![Percentage(25), Percentage(25)], vec![50..75, 75..100], Flex::End)]
//         #[case::percentage_spacebetween(vec![Percentage(25), Percentage(25)], vec![0..25, 75..100], Flex::SpaceBetween)]
//         #[case::percentage_spacearound(vec![Percentage(25), Percentage(25)], vec![17..42, 58..83], Flex::SpaceAround)]
//         #[case::min_legacy2(vec![Min(25), Min(25)], vec![0..25, 25..100], Flex::Legacy)]
//         #[case::min_start2(vec![Min(25), Min(25)], vec![0..50, 50..100], Flex::Start)]
//         #[case::min_center2(vec![Min(25), Min(25)], vec![0..50, 50..100], Flex::Center)]
//         #[case::min_end2(vec![Min(25), Min(25)], vec![0..50, 50..100], Flex::End)]
//         #[case::min_spacebetween(vec![Min(25), Min(25)], vec![0..50, 50..100], Flex::SpaceBetween)]
//         #[case::min_spacearound(vec![Min(25), Min(25)], vec![0..50, 50..100], Flex::SpaceAround)]
//         #[case::max_legacy2(vec![Max(25), Max(25)], vec![0..25, 25..100], Flex::Legacy)]
//         #[case::max_start2(vec![Max(25), Max(25)], vec![0..25, 25..50], Flex::Start)]
//         #[case::max_center2(vec![Max(25), Max(25)], vec![25..50, 50..75], Flex::Center)]
//         #[case::max_end2(vec![Max(25), Max(25)], vec![50..75, 75..100], Flex::End)]
//         #[case::max_spacebetween(vec![Max(25), Max(25)], vec![0..25, 75..100], Flex::SpaceBetween)]
//         #[case::max_spacearound(vec![Max(25), Max(25)], vec![17..42, 58..83], Flex::SpaceAround)]
//         #[case::length_spaced_around(vec![Length(25), Length(25), Length(25)], vec![0..25, 38..63, 75..100], Flex::SpaceBetween)]
//         fn flex_constraint(
//             #[case] constraints: Vec<Constraint>,
//             #[case] expected: Vec<Range<u16>>,
//             #[case] flex: Flex,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let ranges = Layout::horizontal(constraints)
//                 .flex(flex)
//                 .split(rect)
//                 .iter()
//                 .map(|r| r.left()..r.right())
//                 .collect_vec();
//             assert_eq!(ranges, expected);
//         }

//         #[rstest]
//         #[case::length_overlap1(vec![(0  , 20) , (20 , 20) , (40 , 20)] , vec![Length(20) , Length(20) , Length(20)] , Flex::Start        , 0)]
//         #[case::length_overlap2(vec![(0  , 20) , (19 , 20) , (38 , 20)] , vec![Length(20) , Length(20) , Length(20)] , Flex::Start        , -1)]
//         #[case::length_overlap3(vec![(21 , 20) , (40 , 20) , (59 , 20)] , vec![Length(20) , Length(20) , Length(20)] , Flex::Center       , -1)]
//         #[case::length_overlap4(vec![(42 , 20) , (61 , 20) , (80 , 20)] , vec![Length(20) , Length(20) , Length(20)] , Flex::End          , -1)]
//         #[case::length_overlap5(vec![(0  , 20) , (19 , 20) , (38 , 62)] , vec![Length(20) , Length(20) , Length(20)] , Flex::Legacy       , -1)]
//         #[case::length_overlap6(vec![(0  , 20) , (40 , 20) , (80 , 20)] , vec![Length(20) , Length(20) , Length(20)] , Flex::SpaceBetween , -1)]
//         #[case::length_overlap7(vec![(10 , 20) , (40 , 20) , (70 , 20)] , vec![Length(20) , Length(20) , Length(20)] , Flex::SpaceAround  , -1)]
//         fn flex_overlap(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split(rect);
//             let result = r
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();

//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::length_spacing(vec![(0 , 20), (20, 20) , (40, 20)], vec![Length(20), Length(20), Length(20)], Flex::Start      , 0)]
//         #[case::length_spacing(vec![(0 , 20), (22, 20) , (44, 20)], vec![Length(20), Length(20), Length(20)], Flex::Start      , 2)]
//         #[case::length_spacing(vec![(18, 20), (40, 20) , (62, 20)], vec![Length(20), Length(20), Length(20)], Flex::Center     , 2)]
//         #[case::length_spacing(vec![(36, 20), (58, 20) , (80, 20)], vec![Length(20), Length(20), Length(20)], Flex::End        , 2)]
//         #[case::length_spacing(vec![(0 , 20), (22, 20) , (44, 56)], vec![Length(20), Length(20), Length(20)], Flex::Legacy     , 2)]
//         #[case::length_spacing(vec![(0 , 20), (40, 20) , (80, 20)], vec![Length(20), Length(20), Length(20)], Flex::SpaceBetween, 2)]
//         #[case::length_spacing(vec![(10, 20), (40, 20) , (70, 20)], vec![Length(20), Length(20), Length(20)], Flex::SpaceAround, 2)]
//         fn flex_spacing(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split(rect);
//             let result = r
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::a(vec![(0, 25), (25, 75)], vec![Length(25), Length(25)])]
//         #[case::b(vec![(0, 25), (25, 75)], vec![Length(25), Percentage(25)])]
//         #[case::c(vec![(0, 75), (75, 25)], vec![Percentage(25), Length(25)])]
//         #[case::d(vec![(0, 75), (75, 25)], vec![Min(25), Percentage(25)])]
//         #[case::e(vec![(0, 25), (25, 75)], vec![Percentage(25), Min(25)])]
//         #[case::f(vec![(0, 25), (25, 75)], vec![Min(25), Percentage(100)])]
//         #[case::g(vec![(0, 75), (75, 25)], vec![Percentage(100), Min(25)])]
//         #[case::h(vec![(0, 25), (25, 75)], vec![Max(75), Percentage(75)])]
//         #[case::i(vec![(0, 75), (75, 25)], vec![Percentage(75), Max(75)])]
//         #[case::j(vec![(0, 25), (25, 75)], vec![Max(25), Percentage(25)])]
//         #[case::k(vec![(0, 75), (75, 25)], vec![Percentage(25), Max(25)])]
//         #[case::l(vec![(0, 25), (25, 75)], vec![Length(25), Ratio(1, 4)])]
//         #[case::m(vec![(0, 75), (75, 25)], vec![Ratio(1, 4), Length(25)])]
//         #[case::n(vec![(0, 25), (25, 75)], vec![Percentage(25), Ratio(1, 4)])]
//         #[case::o(vec![(0, 75), (75, 25)], vec![Ratio(1, 4), Percentage(25)])]
//         #[case::p(vec![(0, 25), (25, 75)], vec![Ratio(1, 4), Fill(25)])]
//         #[case::q(vec![(0, 75), (75, 25)], vec![Fill(25), Ratio(1, 4)])]
//         fn constraint_specification_tests_for_priority(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints)
//                 .flex(Flex::Legacy)
//                 .split(rect)
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(r, expected);
//         }

//         #[rstest]
//         #[case::a(vec![(0, 20), (20, 20), (40, 20)], vec![Length(20), Length(20), Length(20)], Flex::Start, 0)]
//         #[case::b(vec![(18, 20), (40, 20), (62, 20)], vec![Length(20), Length(20), Length(20)], Flex::Center, 2)]
//         #[case::c(vec![(36, 20), (58, 20), (80, 20)], vec![Length(20), Length(20), Length(20)], Flex::End, 2)]
//         #[case::d(vec![(0, 20), (22, 20), (44, 56)], vec![Length(20), Length(20), Length(20)], Flex::Legacy, 2)]
//         #[case::e(vec![(0, 20), (22, 20), (44, 56)], vec![Length(20), Length(20), Length(20)], Flex::Legacy, 2)]
//         #[case::f(vec![(10, 20), (40, 20), (70, 20)], vec![Length(20), Length(20), Length(20)], Flex::SpaceAround, 2)]
//         fn constraint_specification_tests_for_priority_with_spacing(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints)
//                 .spacing(spacing)
//                 .flex(flex)
//                 .split(rect)
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(r, expected);
//         }

//         #[rstest]
//         #[case::prop(vec![(0 , 10), (10, 80), (90 , 10)] , vec![Length(10), Fill(1), Length(10)], Flex::Legacy)]
//         #[case::flex(vec![(0 , 10), (90 , 10)] , vec![Length(10), Length(10)], Flex::SpaceBetween)]
//         #[case::prop(vec![(0 , 27), (27, 10), (37, 26), (63, 10), (73, 27)] , vec![Fill(1), Length(10), Fill(1), Length(10), Fill(1)], Flex::Legacy)]
//         #[case::flex(vec![(27 , 10), (63, 10)] , vec![Length(10), Length(10)], Flex::SpaceAround)]
//         #[case::prop(vec![(0 , 10), (10, 10), (20 , 80)] , vec![Length(10), Length(10), Fill(1)], Flex::Legacy)]
//         #[case::flex(vec![(0 , 10), (10, 10)] , vec![Length(10), Length(10)], Flex::Start)]
//         #[case::prop(vec![(0 , 80), (80 , 10), (90, 10)] , vec![Fill(1), Length(10), Length(10)], Flex::Legacy)]
//         #[case::flex(vec![(80 , 10), (90, 10)] , vec![Length(10), Length(10)], Flex::End)]
//         #[case::prop(vec![(0 , 40), (40, 10), (50, 10), (60, 40)] , vec![Fill(1), Length(10), Length(10), Fill(1)], Flex::Legacy)]
//         #[case::flex(vec![(40 , 10), (50, 10)] , vec![Length(10), Length(10)], Flex::Center)]
//         fn fill_vs_flex(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints).flex(flex).split(rect);
//             let result = r
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::flex0(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::Legacy , 0)]
//         #[case::flex0(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::SpaceAround , 0)]
//         #[case::flex0(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::SpaceBetween , 0)]
//         #[case::flex0(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::Start , 0)]
//         #[case::flex0(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::Center , 0)]
//         #[case::flex0(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::End , 0)]
//         #[case::flex10(vec![(0 , 45), (55 , 45)] , vec![Fill(1), Fill(1)], Flex::Legacy , 10)]
//         #[case::flex10(vec![(0 , 45), (55 , 45)] , vec![Fill(1), Fill(1)], Flex::Start , 10)]
//         #[case::flex10(vec![(0 , 45), (55 , 45)] , vec![Fill(1), Fill(1)], Flex::Center , 10)]
//         #[case::flex10(vec![(0 , 45), (55 , 45)] , vec![Fill(1), Fill(1)], Flex::End , 10)]
//         #[case::flex10(vec![(10 , 35), (55 , 35)] , vec![Fill(1), Fill(1)], Flex::SpaceAround , 10)]
//         #[case::flex10(vec![(0 , 45), (55 , 45)] , vec![Fill(1), Fill(1)], Flex::SpaceBetween , 10)]
//         #[case::flex_length0(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::Legacy , 0)]
//         #[case::flex_length0(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceAround , 0)]
//         #[case::flex_length0(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceBetween , 0)]
//         #[case::flex_length0(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::Start , 0)]
//         #[case::flex_length0(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::Center , 0)]
//         #[case::flex_length0(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::End , 0)]
//         #[case::flex_length10(vec![(0 , 35), (45, 10), (65 , 35)] , vec![Fill(1), Length(10), Fill(1)], Flex::Legacy , 10)]
//         #[case::flex_length10(vec![(0 , 35), (45, 10), (65 , 35)] , vec![Fill(1), Length(10), Fill(1)], Flex::Start , 10)]
//         #[case::flex_length10(vec![(0 , 35), (45, 10), (65 , 35)] , vec![Fill(1), Length(10), Fill(1)], Flex::Center , 10)]
//         #[case::flex_length10(vec![(0 , 35), (45, 10), (65 , 35)] , vec![Fill(1), Length(10), Fill(1)], Flex::End , 10)]
//         #[case::flex_length10(vec![(10 , 25), (45, 10), (65 , 25)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceAround , 10)]
//         #[case::flex_length10(vec![(0 , 35), (45, 10), (65 , 35)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceBetween , 10)]
//         fn fill_spacing(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split(rect);
//             let result = r
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(expected, result);
//         }

//         #[rstest]
//         #[case::flex0_1(vec![(0 , 55), (45 , 55)] , vec![Fill(1), Fill(1)], Flex::Legacy , -10)]
//         #[case::flex0_2(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::SpaceAround , -10)]
//         #[case::flex0_3(vec![(0 , 55), (45 , 55)] , vec![Fill(1), Fill(1)], Flex::SpaceBetween , -10)]
//         #[case::flex0_4(vec![(0 , 55), (45 , 55)] , vec![Fill(1), Fill(1)], Flex::Start , -10)]
//         #[case::flex0_5(vec![(0 , 55), (45 , 55)] , vec![Fill(1), Fill(1)], Flex::Center , -10)]
//         #[case::flex0_6(vec![(0 , 55), (45 , 55)] , vec![Fill(1), Fill(1)], Flex::End , -10)]
//         #[case::flex10_1(vec![(0 , 51), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::Legacy , -1)]
//         #[case::flex10_2(vec![(0 , 51), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::Start , -1)]
//         #[case::flex10_3(vec![(0 , 51), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::Center , -1)]
//         #[case::flex10_4(vec![(0 , 51), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::End , -1)]
//         #[case::flex10_5(vec![(0 , 50), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::SpaceAround , -1)]
//         #[case::flex10_6(vec![(0 , 51), (50 , 50)] , vec![Fill(1), Fill(1)], Flex::SpaceBetween , -1)]
//         #[case::flex_length0_1(vec![(0 , 55), (45, 10), (45 , 55)] , vec![Fill(1), Length(10), Fill(1)], Flex::Legacy , -10)]
//         #[case::flex_length0_2(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceAround , -10)]
//         #[case::flex_length0_3(vec![(0 , 55), (45, 10), (45 , 55)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceBetween , -10)]
//         #[case::flex_length0_4(vec![(0 , 55), (45, 10), (45 , 55)] , vec![Fill(1), Length(10), Fill(1)], Flex::Start , -10)]
//         #[case::flex_length0_5(vec![(0 , 55), (45, 10), (45 , 55)] , vec![Fill(1), Length(10), Fill(1)], Flex::Center , -10)]
//         #[case::flex_length0_6(vec![(0 , 55), (45, 10), (45 , 55)] , vec![Fill(1), Length(10), Fill(1)], Flex::End , -10)]
//         #[case::flex_length10_1(vec![(0 , 46), (45, 10), (54 , 46)] , vec![Fill(1), Length(10), Fill(1)], Flex::Legacy , -1)]
//         #[case::flex_length10_2(vec![(0 , 46), (45, 10), (54 , 46)] , vec![Fill(1), Length(10), Fill(1)], Flex::Start , -1)]
//         #[case::flex_length10_3(vec![(0 , 46), (45, 10), (54 , 46)] , vec![Fill(1), Length(10), Fill(1)], Flex::Center , -1)]
//         #[case::flex_length10_4(vec![(0 , 46), (45, 10), (54 , 46)] , vec![Fill(1), Length(10), Fill(1)], Flex::End , -1)]
//         #[case::flex_length10_5(vec![(0 , 45), (45, 10), (55 , 45)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceAround , -1)]
//         #[case::flex_length10_6(vec![(0 , 46), (45, 10), (54 , 46)] , vec![Fill(1), Length(10), Fill(1)], Flex::SpaceBetween , -1)]
//         fn fill_overlap(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split(rect);
//             let result = r
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::flex_length10(vec![(0, 10), (90, 10)], vec![Length(10), Length(10)], Flex::Center, 80)]
//         fn flex_spacing_lower_priority_than_user_spacing(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split(rect);
//             let result = r
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::spacers(vec![(0, 0), (10, 0), (100, 0)], vec![Length(10), Length(10)], Flex::Legacy)]
//         #[case::spacers(vec![(0, 0), (10, 80), (100, 0)], vec![Length(10), Length(10)], Flex::SpaceBetween)]
//         #[case::spacers(vec![(0, 27), (37, 26), (73, 27)], vec![Length(10), Length(10)], Flex::SpaceAround)]
//         #[case::spacers(vec![(0, 0), (10, 0), (20, 80)], vec![Length(10), Length(10)], Flex::Start)]
//         #[case::spacers(vec![(0, 40), (50, 0), (60, 40)], vec![Length(10), Length(10)], Flex::Center)]
//         #[case::spacers(vec![(0, 80), (90, 0), (100, 0)], vec![Length(10), Length(10)], Flex::End)]
//         fn split_with_spacers_no_spacing(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let (_, s) = Layout::horizontal(&constraints)
//                 .flex(flex)
//                 .split_with_spacers(rect);
//             assert_eq!(s.len(), constraints.len() + 1);
//             let result = s
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::spacers(vec![(0, 0), (10, 5), (100, 0)], vec![Length(10), Length(10)], Flex::Legacy, 5)]
//         #[case::spacers(vec![(0, 0), (10, 80), (100, 0)], vec![Length(10), Length(10)], Flex::SpaceBetween, 5)]
//         #[case::spacers(vec![(0, 27), (37, 26), (73, 27)], vec![Length(10), Length(10)], Flex::SpaceAround, 5)]
//         #[case::spacers(vec![(0, 0), (10, 5), (25, 75)], vec![Length(10), Length(10)], Flex::Start, 5)]
//         #[case::spacers(vec![(0, 38), (48, 5), (63, 37)], vec![Length(10), Length(10)], Flex::Center, 5)]
//         #[case::spacers(vec![(0, 75), (85, 5), (100, 0)], vec![Length(10), Length(10)], Flex::End, 5)]
//         fn split_with_spacers_and_spacing(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let (_, s) = Layout::horizontal(&constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split_with_spacers(rect);
//             assert_eq!(s.len(), constraints.len() + 1);
//             let result = s
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(expected, result);
//         }

//         #[rstest]
//         #[case::spacers_1(vec![(0, 0), (10, 0), (100, 0)], vec![Length(10), Length(10)], Flex::Legacy, -1)]
//         #[case::spacers_2(vec![(0, 0), (10, 80), (100, 0)], vec![Length(10), Length(10)], Flex::SpaceBetween, -1)]
//         #[case::spacers_3(vec![(0, 27), (37, 26), (73, 27)], vec![Length(10), Length(10)], Flex::SpaceAround, -1)]
//         #[case::spacers_4(vec![(0, 0), (10, 0), (19, 81)], vec![Length(10), Length(10)], Flex::Start, -1)]
//         #[case::spacers_5(vec![(0, 41), (51, 0), (60, 40)], vec![Length(10), Length(10)], Flex::Center, -1)]
//         #[case::spacers_6(vec![(0, 81), (91, 0), (100, 0)], vec![Length(10), Length(10)], Flex::End, -1)]
//         fn split_with_spacers_and_overlap(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let (_, s) = Layout::horizontal(&constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split_with_spacers(rect);
//             assert_eq!(s.len(), constraints.len() + 1);
//             let result = s
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::spacers(vec![(0, 0), (0, 100), (100, 0)], vec![Length(10), Length(10)], Flex::Legacy, 200)]
//         #[case::spacers(vec![(0, 0), (0, 100), (100, 0)], vec![Length(10), Length(10)], Flex::SpaceBetween, 200)]
//         #[case::spacers(vec![(0, 33), (33, 34), (67, 33)], vec![Length(10), Length(10)], Flex::SpaceAround, 200)]
//         #[case::spacers(vec![(0, 0), (0, 100), (100, 0)], vec![Length(10), Length(10)], Flex::Start, 200)]
//         #[case::spacers(vec![(0, 0), (0, 100), (100, 0)], vec![Length(10), Length(10)], Flex::Center, 200)]
//         #[case::spacers(vec![(0, 0), (0, 100), (100, 0)], vec![Length(10), Length(10)], Flex::End, 200)]
//         fn split_with_spacers_and_too_much_spacing(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//             #[case] spacing: i16,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let (_, s) = Layout::horizontal(&constraints)
//                 .flex(flex)
//                 .spacing(spacing)
//                 .split_with_spacers(rect);
//             assert_eq!(s.len(), constraints.len() + 1);
//             let result = s
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }

//         #[rstest]
//         #[case::compare(vec![(0, 90), (90, 10)], vec![Min(10), Length(10)], Flex::Legacy)]
//         #[case::compare(vec![(0, 90), (90, 10)], vec![Min(10), Length(10)], Flex::Start)]
//         #[case::compare(vec![(0, 10), (10, 90)], vec![Min(10), Percentage(100)], Flex::Legacy)]
//         #[case::compare(vec![(0, 10), (10, 90)], vec![Min(10), Percentage(100)], Flex::Start)]
//         #[case::compare(vec![(0, 50), (50, 50)], vec![Percentage(50), Percentage(50)], Flex::Legacy)]
//         #[case::compare(vec![(0, 50), (50, 50)], vec![Percentage(50), Percentage(50)], Flex::Start)]
//         fn legacy_vs_default(
//             #[case] expected: Vec<(u16, u16)>,
//             #[case] constraints: Vec<Constraint>,
//             #[case] flex: Flex,
//         ) {
//             let rect = Rect::new(0, 0, 100, 1);
//             let r = Layout::horizontal(constraints).flex(flex).split(rect);
//             let result = r
//                 .iter()
//                 .map(|r| (r.x, r.width))
//                 .collect::<Vec<(u16, u16)>>();
//             assert_eq!(result, expected);
//         }
//     }

//     #[test]
//     fn test_solver() {
//         use super::*;

//         let mut solver = Solver::new();
//         let x = Variable::new();
//         let y = Variable::new();

//         solver.add_constraint((x + y) | EQ(4.0) | 5.0).unwrap();
//         solver.add_constraint(x | EQ(1.0) | 2.0).unwrap();
//         for _ in 0..5 {
//             solver.add_constraint(y | EQ(1.0) | 2.0).unwrap();
//         }

//         let changes: HashMap<Variable, f64> = solver.fetch_changes().iter().copied().collect();
//         let x = changes.get(&x).unwrap_or(&0.0).round() as u16;
//         let y = changes.get(&y).unwrap_or(&0.0).round() as u16;
//         assert_eq!(x, 3);
//         assert_eq!(y, 2);

//         let mut solver = Solver::new();
//         let x = Variable::new();
//         let y = Variable::new();

//         solver.add_constraint((x + y) | EQ(4.0) | 5.0).unwrap();
//         solver.add_constraint(y | EQ(1.0) | 2.0).unwrap();
//         for _ in 0..5 {
//             solver.add_constraint(x | EQ(1.0) | 2.0).unwrap();
//         }

//         let changes: HashMap<Variable, f64> = solver.fetch_changes().iter().copied().collect();
//         let x = changes.get(&x).unwrap_or(&0.0).round() as u16;
//         let y = changes.get(&y).unwrap_or(&0.0).round() as u16;
//         assert_eq!(x, 2);
//         assert_eq!(y, 3);
//     }
// }
