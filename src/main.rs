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
        func: fn(Vec<&str>) -> CommandResult
    }

    enum CommandResult {
        Success(isize),
        Failure(isize),
        Exit
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
                    Builtin { name: "quit", desc: "quit the shell", func: cmd_quit },
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
                        CommandResult::Exit => break,
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

    fn cmd_quit(args: Vec<&str>) -> CommandResult {
        CommandResult::Exit
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
