use libc::termios;
use std::io::{self, Stdin};

/// Provides the `read_hidden_line` method to implementors.
pub trait ReadHidden {
    fn read_hidden_line(&self, buf: &mut String) -> io::Result<usize>;
}

fn get_termios_attr() -> termios {
    let mut termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    unsafe {
        libc::tcgetattr(libc::STDIN_FILENO, &mut termios);
    }
    termios
}

fn set_termios_attr(termios: &termios) {
    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, termios);
    }
}

impl ReadHidden for Stdin {
    /// Calls `Stdin::read_line`, but prevents any input charactes from bring echoed to `Stdout`.
    fn read_hidden_line(&self, buf: &mut String) -> io::Result<usize> {
        use libc::{ECHO, ECHONL};

        let termios_orig = get_termios_attr();
        let mut termios = termios_orig.clone();

        termios.c_lflag &= !ECHO;
        termios.c_lflag |= ECHONL;

        set_termios_attr(&termios);
        let res = self.read_line(buf);
        set_termios_attr(&termios_orig);

        res
    }
}
