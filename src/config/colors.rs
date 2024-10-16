use clap::ValueEnum;

/// This enum tracks the user's preference on whether we should
/// use ANSI color codes in terminal output. If no preference is
/// provided, we check whether the output file is a TTY (and thus
/// support color codes).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum EnableColors {
    /// Always color the output
    Always,
    /// Never color the output.
    Never,
    /// Detect whether the terminal is a tty before deciding to color
    #[default]
    Auto,
}

impl EnableColors {
    /// A convenience helper function that returns the user's
    /// preference for color. None indicates that this program
    /// should decide (i.e. "auto").
    pub fn color_preference(self) -> Option<bool> {
        match self {
            EnableColors::Always => Some(true),
            EnableColors::Never => Some(false),
            EnableColors::Auto => None,
        }
    }
}
