use std::{ env, process };
use minigrep::Arguments;

fn get_args() -> Result<Arguments, &'static str> {
    // Get arguments from env.args...
    let arguments: Vec<String> = env::args().collect();

    // Returns the structure with the recollected data.
    Arguments::build(&arguments)
}

fn main() {
    let arguments = get_args().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("\nLooking for '{}'", arguments.target);
    println!("At: {}\n", arguments.path);

    if let Err(e) = minigrep::run(arguments) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
