#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}: {}", ansi_term::Color::Blue.paint("info"), format!($($arg)*));
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}: {}", ansi_term::Color::Yellow.paint("warning"), format!($($arg)*));
    }
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        println!("{}: {}", ansi_term::Color::Red.paint("error"), format!($($arg)*));
    }
}
