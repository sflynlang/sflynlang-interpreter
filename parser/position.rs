use std::fmt;

#[derive(Clone, Debug)]
pub struct Position {
    start_position: usize,
    end_position: usize,
    line: usize,
    column: usize,
}

impl Position {
    /// Create a new position object.
    ///
    /// ```rust
    /// use slang_parser::Position;
    ///
    /// fn main() {
    ///     let position = Position::new(0, 0, 1, 1);
    ///
    ///     println!("{}", position);
    ///     // Output: Ln 1, Col 1
    /// }
    /// ```
    pub fn new(start_position: usize, end_position: usize, line: usize, column: usize) -> Self {
        Self {
            start_position,
            end_position,
            line,
            column,
        }
    }

    /// Get the start position.
    pub fn get_start_position(&self) -> usize {
        self.start_position
    }

    /// Get the end position.
    pub fn get_end_position(&self) -> usize {
        self.end_position
    }

    /// Set a new end position.
    pub fn set_end_position(&mut self, end_position: usize) {
        self.end_position = end_position;
    }

    /// Get the number of the line.
    pub fn get_line(&self) -> usize {
        self.line
    }

    /// Get the number of the column inside the line.
    pub fn get_column(&self) -> usize {
        self.column
    }

    /// Get the start and end positions as a range.
    pub fn get_range(&self) -> std::ops::Range<usize> {
        self.start_position..self.end_position
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ln {}, Col {}", self.get_line(), self.get_column())
    }
}
