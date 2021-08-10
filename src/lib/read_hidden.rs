use std::io::{self, Stdin, Write};
use libc::termios;

pub trait ReadHidden {
    fn read_hidden_line(&self, buf: &mut String) -> io::Result<usize>;
}

pub trait Input {
    fn input(&self, prompt: &str) -> io::Result<String>;
    fn input_hidden(&self, prompt: &str) -> io::Result<String>;
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

impl Input for Stdin {
    fn input(&self, prompt: &str) -> io::Result<String> {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();

        let mut val = String::new();
        match io::stdin().read_line(&mut val) {
            Ok(_) => Ok(String::from(val.trim())),
            Err(e) => Err(e)
        }
    }
    fn input_hidden(&self, prompt: &str) -> io::Result<String> {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();

        let mut val = String::new();
        match io::stdin().read_hidden_line(&mut val) {
            Ok(_) => Ok(String::from(val.trim())),
            Err(e) => Err(e)
        }
    }
}

//#[cfg(test)]
//mod tests {
    //use std::io::{self, Write};
    //use crate::ReadHidden;
//
    //#[test]
    //fn test() {
        //let mut var = String::new();
        //print!("var: ");
        //io::stdout().flush().unwrap();
        //io::stdin().read_line(&mut var).unwrap();
//
        //let mut hid = String::new();
        //print!("hid: ");
        //io::stdout().flush().unwrap();
        //io::stdin().read_hidden_line(&mut hid).unwrap();
//
        //println!("my variable is {} and password is {}.", var.trim(), hid.trim());
    //}
//}
