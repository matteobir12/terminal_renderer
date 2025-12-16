use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

const ENTER_ALTERNATE_SCREEN: &str = "\x1b[?1049h";
const EXIT_ALTERNATE_SCREEN: &str = "\x1b[?1049l";
const CLEAR_SCREEN: &str = "\x1b[2J";
const CURSOR_HOME: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";

pub struct Terminal {
    original_termios: libc::termios,
    stdin_fd: i32,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let stdin_fd = io::stdin().as_raw_fd();
        let original_termios = get_termios(stdin_fd)?;

        Ok(Self {
            original_termios,
            stdin_fd,
        })
    }

    pub fn enter_raw_mode(&mut self) -> io::Result<()> {
        let mut termios = self.original_termios;

        termios.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ISIG);
        termios.c_iflag &= !(libc::IXON | libc::ICRNL);
        termios.c_oflag &= !libc::OPOST;
        termios.c_cc[libc::VMIN] = 0;
        termios.c_cc[libc::VTIME] = 0;
        set_termios(self.stdin_fd, &termios)?;
        Ok(())
    }

    pub fn enter_alternate_screen(&self) -> io::Result<()> {
        io::stdout().write_all(ENTER_ALTERNATE_SCREEN.as_bytes())?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn exit_alternate_screen(&self) -> io::Result<()> {
        io::stdout().write_all(EXIT_ALTERNATE_SCREEN.as_bytes())?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        io::stdout().write_all(CLEAR_SCREEN.as_bytes())?;
        io::stdout().write_all(CURSOR_HOME.as_bytes())?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn hide_cursor(&self) -> io::Result<()> {
        io::stdout().write_all(HIDE_CURSOR.as_bytes())?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn show_cursor(&self) -> io::Result<()> {
        io::stdout().write_all(SHOW_CURSOR.as_bytes())?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn read_input_non_blocking(&self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; 1024];
        let mut stdin = io::stdin();

        match stdin.read(&mut buffer) {
            Ok(0) => Ok(vec![]),
            Ok(n) => {
                buffer.truncate(n);
                Ok(buffer)
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    pub fn restore(&self) -> io::Result<()> {
        set_termios(self.stdin_fd, &self.original_termios)?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.restore();
        let _ = self.show_cursor();
        let _ = self.exit_alternate_screen();
    }
}

fn get_termios(fd: i32) -> io::Result<libc::termios> {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        if libc::tcgetattr(fd, &mut termios) == 0 {
            Ok(termios)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

fn set_termios(fd: i32, termios: &libc::termios) -> io::Result<()> {
    unsafe {
        if libc::tcsetattr(fd, libc::TCSANOW, termios) == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
