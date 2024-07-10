use nix::{
    sys::ptrace,
    unistd::{fork, ForkResult},
};
use std::{env, error::Error, os::unix::process::CommandExt, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("The target application must be provided as an argument.");
        std::process::exit(1);
    }

    let debuggee = &args[1];
    let debuggee_args = &args[2..];

    match unsafe { fork()? } {
        ForkResult::Parent { child } => {
            println!("Debugger started. Child PID: {}", child);
            Ok(())
        }
        ForkResult::Child => {
            ptrace::traceme()?;
            Command::new(debuggee).args(debuggee_args).exec();
            Ok(())
        }
    }
}
