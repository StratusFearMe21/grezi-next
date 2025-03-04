use std::fmt;

/// A constraint that defines the size of a layout element.
///
/// Constraints can be used to specify a fixed size, a percentage of the available space, a ratio of
/// the available space, a minimum or maximum size or a fill proportional value for a layout
/// element.
///
/// Relative constraints (percentage, ratio) are calculated relative to the entire space being
/// divided, rather than the space available after applying more fixed constraints (min, max,
/// length).
///
/// Constraints are prioritized in the following order:
///
/// 1. [`Constraint::Min`]
/// 2. [`Constraint::Max`]
/// 3. [`Constraint::Length`]
/// 4. [`Constraint::Percentage`]
/// 5. [`Constraint::Ratio`]
/// 6. [`Constraint::Fill`]
///
/// # Examples
///
/// `Constraint` provides helper methods to create lists of constraints from various input formats.
///
/// ```rust
/// use ratatui_core::layout::Constraint;
///
/// // Create a layout with specified lengths for each element
/// let constraints = Constraint::from_lengths([10, 20, 10]);
///
/// // Create a centered layout using ratio or percentage constraints
/// let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
/// let constraints = Constraint::from_percentages([25, 50, 25]);
///
/// // Create a centered layout with a minimum size constraint for specific elements
/// let constraints = Constraint::from_mins([0, 100, 0]);
///
/// // Create a sidebar layout specifying maximum sizes for the columns
/// let constraints = Constraint::from_maxes([30, 170]);
///
/// // Create a layout with fill proportional sizes for each element
/// let constraints = Constraint::from_fills([1, 2, 1]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constraint {
    /// Applies a minimum size constraint to the element
    ///
    /// The element size is set to at least the specified amount.
    ///
    /// # Examples
    ///
    /// `[Percentage(100), Min(20)]`
    ///
    /// ```plain
    /// ┌────────────────────────────┐┌──────────────────┐
    /// │            30 px           ││       20 px      │
    /// └────────────────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Percentage(100), Min(10)]`
    ///
    /// ```plain
    /// ┌──────────────────────────────────────┐┌────────┐
    /// │                 40 px                ││  10 px │
    /// └──────────────────────────────────────┘└────────┘
    /// ```
    Min(f64),

    /// Applies a maximum size constraint to the element
    ///
    /// The element size is set to at most the specified amount.
    ///
    /// # Examples
    ///
    /// `[Percentage(0), Max(20)]`
    ///
    /// ```plain
    /// ┌────────────────────────────┐┌──────────────────┐
    /// │            30 px           ││       20 px      │
    /// └────────────────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Percentage(0), Max(10)]`
    ///
    /// ```plain
    /// ┌──────────────────────────────────────┐┌────────┐
    /// │                 40 px                ││  10 px │
    /// └──────────────────────────────────────┘└────────┘
    /// ```
    Max(f64),

    /// Applies a length constraint to the element
    ///
    /// The element size is set to the specified amount.
    ///
    /// # Examples
    ///
    /// `[Length(20), Length(20)]`
    ///
    /// ```plain
    /// ┌──────────────────┐┌──────────────────┐
    /// │       20 px      ││       20 px      │
    /// └──────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Length(20), Length(30)]`
    ///
    /// ```plain
    /// ┌──────────────────┐┌────────────────────────────┐
    /// │       20 px      ││            30 px           │
    /// └──────────────────┘└────────────────────────────┘
    /// ```
    Length(f64),

    /// Applies a percentage of the available space to the element
    ///
    /// Converts the given percentage to a floating-point value and multiplies that with area. This
    /// value is rounded back to a integer as part of the layout split calculation.
    ///
    /// **Note**: As this value only accepts a `u16`, certain percentages that cannot be
    /// represented exactly (e.g. 1/3) are not possible. You might want to use
    /// [`Constraint::Ratio`] or [`Constraint::Fill`] in such cases.
    ///
    /// # Examples
    ///
    /// `[Percentage(75), Fill(1)]`
    ///
    /// ```plain
    /// ┌────────────────────────────────────┐┌──────────┐
    /// │                38 px               ││   12 px  │
    /// └────────────────────────────────────┘└──────────┘
    /// ```
    ///
    /// `[Percentage(50), Fill(1)]`
    ///
    /// ```plain
    /// ┌───────────────────────┐┌───────────────────────┐
    /// │         25 px         ││         25 px         │
    /// └───────────────────────┘└───────────────────────┘
    /// ```
    Percentage(f64),

    /// Applies a ratio of the available space to the element
    ///
    /// Converts the given ratio to a floating-point value and multiplies that with area.
    /// This value is rounded back to a integer as part of the layout split calculation.
    ///
    /// # Examples
    ///
    /// `[Ratio(1, 2) ; 2]`
    ///
    /// ```plain
    /// ┌───────────────────────┐┌───────────────────────┐
    /// │         25 px         ││         25 px         │
    /// └───────────────────────┘└───────────────────────┘
    /// ```
    ///
    /// `[Ratio(1, 4) ; 4]`
    ///
    /// ```plain
    /// ┌───────────┐┌──────────┐┌───────────┐┌──────────┐
    /// │   13 px   ││   12 px  ││   13 px   ││   12 px  │
    /// └───────────┘└──────────┘└───────────┘└──────────┘
    /// ```
    Ratio(f64, f64),

    /// Applies the scaling factor proportional to all other [`Constraint::Fill`] elements
    /// to fill excess space
    ///
    /// The element will only expand or fill into excess available space, proportionally matching
    /// other [`Constraint::Fill`] elements while satisfying all other constraints.
    ///
    /// # Examples
    ///
    ///
    /// `[Fill(1), Fill(2), Fill(3)]`
    ///
    /// ```plain
    /// ┌──────┐┌───────────────┐┌───────────────────────┐
    /// │ 8 px ││     17 px     ││         25 px         │
    /// └──────┘└───────────────┘└───────────────────────┘
    /// ```
    ///
    /// `[Fill(1), Percentage(50), Fill(1)]`
    ///
    /// ```plain
    /// ┌───────────┐┌───────────────────────┐┌──────────┐
    /// │   13 px   ││         25 px         ││   12 px  │
    /// └───────────┘└───────────────────────┘└──────────┘
    /// ```
    Fill(f64),
}

// impl Constraint {
//     fn apply(&self, length: f64) -> f64 {
//         match *self {
//             Self::Percentage(p) => {
//                 let p = p / 100.0;
//                 (p * length).min(length)
//             }
//             Self::Ratio(numerator, denominator) => {
//                 // avoid division by zero by using 1 when denominator is 0
//                 // this results in 0/0 -> 0 and x/0 -> x for x != 0
//                 let percentage = numerator / denominator.max(1.0);
//                 let length = length;
//                 (percentage * length).min(length)
//             }
//             Self::Length(l) | Self::Fill(l) => length.min(l),
//             Self::Max(m) => length.min(m),
//             Self::Min(m) => length.max(m),
//         }
//     }
// }

impl From<&Self> for Constraint {
    fn from(constraint: &Self) -> Self {
        *constraint
    }
}

impl AsRef<Self> for Constraint {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for Constraint {
    fn default() -> Self {
        Self::Percentage(100.0)
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Percentage(p) => write!(f, "{p}%"),
            Self::Ratio(n, d) => write!(f, "{n}:{d})"),
            Self::Length(l) => write!(f, "{l}~"),
            Self::Fill(l) => write!(f, "{l}#"),
            Self::Max(m) => write!(f, "{m}+"),
            Self::Min(m) => write!(f, "{m}-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Constraint::default(), Constraint::Percentage(100.0));
    }

    #[test]
    fn to_string() {
        assert_eq!(Constraint::Percentage(50.0).to_string(), "50%");
        assert_eq!(Constraint::Ratio(1.0, 2.0).to_string(), "1:2");
        assert_eq!(Constraint::Length(10.0).to_string(), "10~");
        assert_eq!(Constraint::Max(10.0).to_string(), "10+");
        assert_eq!(Constraint::Min(10.0).to_string(), "10-");
    }

    // #[test]
    // #[allow(deprecated)]
    // fn apply() {
    //     assert_eq!(Constraint::Percentage(0.0).apply(100.0), 0.0);
    //     assert_eq!(Constraint::Percentage(50.0).apply(100.0), 50.0);
    //     assert_eq!(Constraint::Percentage(100.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Percentage(200.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Percentage(f64::MAX).apply(100.0), 100.0);

    //     // 0.0/0 intentionally avoids a panic by returning 0.0.
    //     assert_eq!(Constraint::Ratio(0.0, 0.0).apply(100.0), 0.0);
    //     // 1.0/0 intentionally avoids a panic by returning 100.0% of the length.
    //     assert_eq!(Constraint::Ratio(1.0, 0.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Ratio(0.0, 1.0).apply(100.0), 0.0);
    //     assert_eq!(Constraint::Ratio(1.0, 2.0).apply(100.0), 50.0);
    //     assert_eq!(Constraint::Ratio(2.0, 2.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Ratio(3.0, 2.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Ratio(f64::MAX, 2.0).apply(100.0), 100.0);

    //     assert_eq!(Constraint::Length(0.0).apply(100.0), 0.0);
    //     assert_eq!(Constraint::Length(50.0).apply(100.0), 50.0);
    //     assert_eq!(Constraint::Length(100.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Length(200.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Length(f64::MAX).apply(100.0), 100.0);

    //     assert_eq!(Constraint::Max(0.0).apply(100.0), 0.0);
    //     assert_eq!(Constraint::Max(50.0).apply(100.0), 50.0);
    //     assert_eq!(Constraint::Max(100.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Max(200.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Max(f64::MAX).apply(100.0), 100.0);

    //     assert_eq!(Constraint::Min(0.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Min(50.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Min(100.0).apply(100.0), 100.0);
    //     assert_eq!(Constraint::Min(200.0).apply(100.0), 200.0);
    //     assert_eq!(Constraint::Min(f64::MAX).apply(100.0), f64::MAX);
    // }
}
