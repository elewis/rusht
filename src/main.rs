#![feature(io)]
#![feature(process)]

use shell::Shell;

fn main() {
    Shell::new().run();
}

pub mod shell {
    use std::old_io;
    use std::process;
    use std::vec::Vec;

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
                    Builtin { name: "quit", desc: "quit the shell",       func: cmd_quit },
                    Builtin { name: "help", desc: "print a help message", func: cmd_help },
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

                let mut args = parse::tokenize(&line.trim());
                let cmnd = args.remove(0);
                let builtin = self.lookup(cmnd);

                if builtin.is_some() {
                    match ((*builtin.unwrap()).func)(args) {
                        CommandResult::Exit => break,
                        _ => {}
                    }
                } else {
                    let child = process::Command::new(cmnd).args(&args).spawn();

                    if child.is_ok() {
                        let _ = child.unwrap().wait();
                    } else {
                        println!("{}: command not found", cmnd);
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

    fn cmd_help(args: Vec<&str>) -> CommandResult {
        println!("Rust Shell (Rus[h]t) version '{}'", env!("CARGO_PKG_VERSION"));
        println!("");
        println!("Enter 'help' to view this message");
        CommandResult::Success(0)
    }
}

pub mod parse {

    pub fn tokenize(line : &str) -> Vec<&str> {
        if line.len() > 0 {
            line.split(' ').filter(|s| !s.is_empty()).collect()
        } else {
            vec![]
        }
    }

}
