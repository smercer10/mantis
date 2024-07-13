use nix::{
    sys::{
        ptrace,
        wait::{waitpid, WaitPidFlag},
    },
    unistd::Pid,
};
use std::{
    error::Error,
    io::{self, Write},
};

pub struct Debugger {
    pid: Pid,
}

impl Debugger {
    pub fn new(pid: Pid) -> Self {
        Self { pid }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Debugger started. Child PID: {}", self.pid);

        waitpid(self.pid, Some(WaitPidFlag::empty()))?;

        loop {
            print!("(mantis) ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            if input == "quit" {
                break;
            }

            self.handle_command(input)?;
        }

        Ok(())
    }

    fn handle_command(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let args: Vec<&str> = line.split_whitespace().collect();
        if args.is_empty() {
            return Ok(());
        }

        let command = args[0];

        match command {
            "continue" | "c" => self.continue_execution()?,
            "break" | "b" => {
                if args.len() != 2 {
                    println!("Usage: break <address>");
                    return Ok(());
                }
                // TODO: Implement breakpoint setting
                println!("Breakpoint setting not implemented yet.");
            }
            _ => println!("Unknown command."),
        }

        Ok(())
    }

    fn continue_execution(&mut self) -> Result<(), Box<dyn Error>> {
        ptrace::cont(self.pid, None)?;

        waitpid(self.pid, None)?;
        Ok(())
    }
}
