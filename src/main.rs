use nix::{
    sys::ptrace,
    unistd::{fork, ForkResult},
};
use std::{env, os::unix::process::CommandExt, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("The target application must be provided as an argument.");
        std::process::exit(1);
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("Debugger started. Child PID: {}", child);
        }
        Ok(ForkResult::Child) => {
            if let Err(err) = ptrace::traceme() {
                eprintln!("Failed to trace child process: {}", err);
                std::process::exit(1);
            }

            Command::new(&args[1]).args(&args[2..]).exec();

            // This line should never be reached if the exec call is successful.
            eprintln!("Failed to execute the target application.");
        }
        Err(_) => {
            eprintln!("Failed to fork the process.");
            std::process::exit(1);
        }
    }
}
