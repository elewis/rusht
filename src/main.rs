#![feature(io)]

use std::string::String;

use shell::Shell;

fn main() {
    let mut rusht = Shell::new();

    rusht.run();
}

pub mod shell {
    use std::old_io;

    use parse;

    struct Builtin {
        name: &'static str,
        desc: &'static str,
        func: fn(Vec<&str>) -> cmd::CommandResult
    }

    pub struct Shell {
        prompt: String,
        builtins: Vec<Builtin>,
    }

    impl Shell {
        pub fn new() -> Self {
            Shell {
                prompt: String::from_str("rusht$ "),
                builtins: vec![
                    Builtin { name: "quit", desc: "quit the shell", func: cmd::quit },
                    Builtin { name: "help", desc: "print a help message", func: cmd::help },
                ]
            }
        }

        pub fn run(&mut self) {
            let mut stdin = old_io::stdin();
            loop {
                print!("{}", self.prompt);
                let line = match stdin.read_line() {
                    Ok(line) => line,
                    Err(msg) => panic!("{}: failed to read line", msg)
                };
                let toks = parse::tokenize(&line.trim());
                let comm = self.lookup(toks[0]);

                if comm.is_some() {
                    match ((*comm.unwrap()).func)(toks) {
                        cmd::CommandResult::Exit => break,
                        _ => {}
                    }
                }
            }
            println!("Goodbye");
        }

        fn lookup(&self, name: &str) -> Option<&Builtin> {
            for b in self.builtins.iter() {
                if (*b).name == name {
                    return Some(b);
                }
            }
            None
        }
    }

    mod cmd {
        pub enum CommandResult {
            Success(isize),
            Failure(isize),
            Exit
        }

        pub fn quit(args: Vec<&str>) -> CommandResult {
            CommandResult::Exit
        }

        pub fn help(args: Vec<&str>) -> CommandResult {
            println!("Rust Shell (Rus[h]t) version '{}'", env!("CARGO_PKG_VERSION"));
            println!("");
            println!("Enter 'help' to view this message");
            CommandResult::Success(0)
        }
    }
}

pub mod parse {

    pub fn tokenize(line : &str) -> Vec<&str> {
        let tokens: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();

        for token in tokens.iter() {
            println!("   - {}\\", token);
        }
        tokens
    }

}
