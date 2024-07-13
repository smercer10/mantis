use nix::{
    sys::wait::{waitpid, WaitPidFlag},
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
        }

        Ok(())
    }
}
