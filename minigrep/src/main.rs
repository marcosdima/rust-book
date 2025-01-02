use std::env;
use std::fs;

#[derive(Debug)]
struct Arguments {
    target: String,
    path: String,
}

fn get_args() -> Arguments {
    // Get arguments from env.args...
    let arguments: Vec<String> = env::args().collect();

    // Returns the structure with the recollected data.
    Arguments {
        target: arguments[1].clone(),
        path: arguments[2].clone()
    }
}

fn main() {
    let arguments = get_args();

    println!("{arguments:#?}\n");

    let text = fs::read_to_string(arguments.path)
        .expect("File can't be readed...");

    println!("The text is:\n{text}");
}
