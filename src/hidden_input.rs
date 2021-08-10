use libc::*;

fn main() {
    unsafe {
        let term: *mut termios;
        let oldterm: *mut termios;
        let pass: *mut c_char;

        term = &mut termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        oldterm = term.clone();
        pass = "".as_ptr() as *mut c_char;
        tcgetattr(STDIN_FILENO, term);
        tcgetattr(STDIN_FILENO, oldterm);

        (*term).c_lflag &= !ECHO;
        (*term).c_lflag |= ECHONL;

        tcsetattr(STDIN_FILENO, TCSANOW, term);

        fgets(pass, 10, fdopen(STDIN_FILENO, "w".as_ptr() as *const c_char));
        printf("password: %s".as_ptr() as *const c_char, pass);
        tcsetattr(STDIN_FILENO, TCSANOW, oldterm);
    }
}
