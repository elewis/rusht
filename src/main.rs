
use shell::Shell;

fn main() {
    Shell::new().run();
}

pub mod shell {
    use std::env;
    use std::io::{self, Write};
    use std::process;
    use std::vec::Vec;
    use std::path;

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
        stdin:  io::Stdin,
        stdout: io::Stdout,
        stderr: io::Stderr,
        builtins: Vec<Builtin>,
    }

    impl Shell {
        pub fn new() -> Self {
            Shell {
                stdin:  io::stdin(),
                stdout: io::stdout(),
                stderr: io::stderr(),
                builtins: vec![
                    Builtin { name: "quit", desc: "quit the shell",           func: cmd_quit },
                    Builtin { name: "help", desc: "print a help message",     func: cmd_help },
                    Builtin { name: "pwd",  desc: "print working directory",  func: cmd_pwd },
                    Builtin { name: "cd",   desc: "change working directory", func: cmd_cd }
                ]
            }
        }

        pub fn run(&mut self) {
            cmd_help(vec![]);
            loop {
                self.prompt();

                let mut line = String::new();
                match self.stdin.read_line(&mut line) {
                    Err(msg) => panic!("{}: failed to read line", msg),
                    _ => {}
                };

                let expanded = self.expand_shortcuts(&line.trim());
                let mut args = parse::tokenize(&expanded);
                if args.len() == 0 {
                    continue;
                }
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

        fn prompt(&mut self) {
            print!("{}$ ", "rusht");
            let _ = self.stdout.flush();
        }

        fn lookup(&self, name: &str) -> Option<&Builtin> {
            for b in self.builtins.iter() {
                if (*b).name == name {
                    return Some(b);
                }
            }
            None
        }

        fn expand_shortcuts(&self, line: &str) -> String {
            let test_home = env::var_os("HOME");

            if test_home.is_none() {
                return line.to_string();
            }

            let home_os = test_home.unwrap();
            let home = home_os.to_str().unwrap();
            let mut out = String::new();
            for c in line.chars() {
                if c == '~' {
                    out.push_str(&home);
                } else {
                    out.push(c);
                }
            }
            out
        }
    }

    fn cmd_cd(args: Vec<&str>) -> CommandResult {
        let dir = match args.len() {
            0 => env::home_dir().unwrap().into_os_string().into_string().unwrap(),
            _ => args[0].to_string()
        };
        match env::set_current_dir(path::Path::new(&dir)) {
            Ok(_) => {
                CommandResult::Success(0)
            },
            Err(msg) => {
                println!("{}", msg);
                CommandResult::Failure(1)
            }
        }
    }

    fn cmd_pwd(_: Vec<&str>) -> CommandResult {
        match env::current_dir() {
            Ok(path) => {
                println!("{}", path.display());
                CommandResult::Success(0)
            },
            Err(msg) => {
                println!("{}", msg);
                CommandResult::Failure(1)
            }
        }
    }

    fn cmd_quit(_: Vec<&str>) -> CommandResult {
        CommandResult::Exit
    }

    fn cmd_help(_: Vec<&str>) -> CommandResult {
        println!("Rust Shell (Rus[h]t) version '{}'", env!("CARGO_PKG_VERSION"));
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
