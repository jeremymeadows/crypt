use std::env;
use std::collections::HashMap;

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut arg_parser = ArgParser::new();
    arg_parser.parse();

    // for e in args.iter() {
        // println!("{}", e);
        println!("{:?}", arg_parser.args.subcommands);
        println!("{:?}", arg_parser.args.options);
        println!("{:?}", arg_parser.args.flags);
    // }
}
pub struct ArgParser {
    args: Args,
    flags: Vec<char>,
    options: HashMap::<String, Vec<String>>,
    subcommands: Vec<String>,
}

struct Args {
    args: Vec<String>,
    flags: Vec<char>,
    options: HashMap::<String, Vec<String>>,
    subcommands: Vec<String>,
}

impl ArgParser {
    pub fn new() -> ArgParser {
        ArgParser {
            args: Args::new(),
            flags: Vec::<char>::new(),
            options: HashMap::<String, Vec<String>>::new(),
            subcommands: Vec::<String>::new(),
        }
    }
    pub fn parse(&mut self) {
        self.args.parse(&self.flags, &self.options, &self.subcommands);
    }

    pub fn add_flag(&mut self, flag: char) {

    }
    pub fn add_option(&mut self, option: String, num: u8) {

    }
    pub fn add_subcommand(&mut self, command: String) {
        self.subcommands.push(command);
    }

    pub fn has_flag(&self, flag: char) -> bool {
        self.args.flags.contains(&flag)
    }
}

impl Args {
    fn new() -> Args {
        Args {
            args: env::args().collect::<Vec<String>>(),
            flags: Vec::<char>::new(),
            options: HashMap::<String, Vec<String>>::new(),
            subcommands: Vec::<String>::new(),
        }
    }

    fn parse(&mut self, flags: &Vec<char>, options: &HashMap<String, Vec<String>>, subcommands: &Vec<String>) {
        self.args.remove(0);

        self.parse_subcommand();
        self.parse_options();
        self.parse_flags();
    }

    fn parse_subcommand(&mut self) {
        if self.args.len() > 0 {
            for arg in self.args.clone() {
                if !arg.starts_with("-") {
                    self.subcommands.push(arg);
                    self.args.remove(0);
                } else {
                    return;
                }
            }
        }
    }
    fn parse_options(&mut self) {
        let mut found_opt = false;
        let mut opt = String::new();

        for arg in self.args.clone() {
            if found_opt == false {
                if arg.starts_with("--") {
                    opt = arg.replace("--", "");
                    match self.options.get(&opt) {
                        Some (_) => (),
                        None => {
                            found_opt = true;
                            self.options.insert(opt.clone(), Vec::<String>::new());
                        },
                    }
                }
            } else {
                if arg.starts_with("-") {
                    if arg.starts_with("--") {
                        self.parse_options();
                    }
                    break;
                } else {
                    let mut v = self.options[&opt].clone();
                    v.push(arg);
                    self.options.insert(opt.clone(), v);
                }
            }
        }
    }
    fn parse_flags(&mut self) {
        for arg in self.args.clone() {
            if arg.starts_with("-") && !arg.starts_with("--") {
                let a = arg.replace("-", "");
                if a.len() == 1 {
                    self.flags.push(a.chars().next().unwrap());
                }
            }
        }
    }
}
