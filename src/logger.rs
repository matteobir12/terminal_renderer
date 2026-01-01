use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::logger::log_fn(format_args!($($arg)*));
    }};
}

// Can maybe do like a log ring and add it to frames
static LOG_FILE: Lazy<Mutex<std::fs::File>> = Lazy::new(|| {
  Mutex::new(
      OpenOptions::new()
          .create(true)
          .append(false)
          .write(true)
          .open("/tmp/debug.log")
          .unwrap())
});

pub fn log_fn(args: std::fmt::Arguments) {
  let mut f = LOG_FILE.lock().unwrap();
  writeln!(f, "{}", args).unwrap();
}