use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("The target application must be provided as an argument.");
        std::process::exit(1);
    }

    let _debuggee = std::process::Command::new(&args[1])
        .spawn()
        .expect("Failed to start the target application.");
}
