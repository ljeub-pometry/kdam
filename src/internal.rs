use std::sync::mpsc;

#[derive(Debug)]
pub struct BarInternal {
    pub started: bool,
    pub elapsed_time: f64,
    pub its_per: f64,
    pub bar_length: i16,
    pub user_ncols: i16,
    pub charset: String,
    pub charset_len: u64,
    pub timer: std::time::Instant,
    pub stdout: std::io::Stdout,
    pub nrows: i16,
    pub tx: Option<mpsc::Sender<(i16, String, bool)>>,
}

impl Default for BarInternal {
    fn default() -> BarInternal {
        BarInternal {
            started: false,
            elapsed_time: 0.0,
            its_per: 0.0,
            bar_length: 0,
            user_ncols: -1,
            charset: [
                "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}",
                "\u{2588}",
            ]
            .join(""),
            charset_len: 8,
            timer: std::time::Instant::now(),
            stdout: std::io::stdout(),
            nrows: -1,
            tx: None,
        }
    }
}