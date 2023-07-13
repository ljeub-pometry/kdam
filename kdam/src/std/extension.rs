use std::io::{Result, Write};

/// Comman progress bar functionalities shared between different types of progress bars.
pub trait BarExt {
    /// Clear current bar display.
    fn clear(&mut self) -> Result<()>;

    /// Take input via bar (without overlap with bars).
    fn input<T: Into<String>>(&mut self, text: T) -> Result<String>;

    /// Force refresh the display of this bar.
    fn refresh(&mut self) -> Result<()>;

    /// Render progress bar.
    fn render(&mut self) -> String;

    /// Resets to initial iterations for repeated use.
    /// Consider combining with `leave=true`.
    fn reset(&mut self, total: Option<usize>);

    /// Manually update the progress bar, useful for streams such as reading files.
    /// Returns wheter a update was triggered or not depending on constraints.
    fn update(&mut self, n: usize) -> Result<bool>;

    /// Set counter position instead of incrementing progress bar through [update](Self::update).
    /// Alternative way to update bar.
    /// Returns wheter a update was triggered or not depending on constraints.
    fn update_to(&mut self, update_to_n: usize) -> Result<bool>;

    /// Print a message via bar (without overlap with bars).
    fn write<T: Into<String>>(&mut self, text: T) -> Result<()>;

    /// Write rendered text to a writer, useful for writing files.
    /// If `n` is supplied then this method behaves like [update](Self::update).
    /// Returns wheter a update was triggered or not depending on constraints.
    ///
    /// # Example
    ///
    /// Using [write_to](Self::write_to) as [update_to](Self::update_to).
    ///
    /// ```
    /// use kdam::{tqdm, BarExt};
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut pb = tqdm!(total = 100);
    /// let mut f = File::create("logs.txt").unwrap();
    /// 
    /// for i in 1..101 {
    ///     pb.set_counter(i);
    ///     pb.write_to(&mut f, Some(0));
    /// }
    /// ```
    fn write_to<T: Write>(&mut self, writer: &mut T, n: Option<usize>) -> Result<bool>;
}
