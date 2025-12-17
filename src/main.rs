mod rendering;
mod terminal;

use crate::terminal::Terminal;
use std::io::{self, Write};
use crate::rendering::{do_pipeline, gray_to_ascii};

fn main() -> io::Result<()> {
    let mut term = Terminal::new()?;

    term.enter_raw_mode()?;
    term.enter_alternate_screen()?;
    term.hide_cursor()?;

    let ascii_art: String = gray_to_ascii(48, 128);


    loop {
        let input = term.read_input_non_blocking()?;
        const ESC_KEY:u8 = 27;

        if !input.is_empty() && (input == b"q" || input == [ESC_KEY]) {
            break;
        }

        term.clear_screen()?;
        io::stdout().write_all(ascii_art.as_bytes())?;
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }


    term.show_cursor()?;
    term.exit_alternate_screen()?;
    term.restore()?;

    Ok(())
}
