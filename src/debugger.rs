use crate::debugger::breakpoint::Breakpoint;
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

mod breakpoint;

pub struct Debugger {
    pid: Pid,
    breakpoints: Vec<Breakpoint>,
}

impl Debugger {
    pub fn new(pid: Pid) -> Self {
        Self {
            pid,
            breakpoints: Vec::new(),
        }
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

                let address = usize::from_str_radix(args[1], 16)?;
                self.set_breakpoint(address)?;
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

    fn set_breakpoint(&mut self, address: usize) -> Result<(), Box<dyn Error>> {
        let mut breakpoint = Breakpoint::new(self.pid, address);
        breakpoint.enable()?;
        println!("Breakpoint set at address 0x{:x}.", address);
        self.breakpoints.push(breakpoint);
        Ok(())
    }
}
