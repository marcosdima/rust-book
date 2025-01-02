use std::env;
use std::fs;

#[derive(Debug)]
struct Arguments {
    target: String,
    path: String,
}

impl Arguments {
    fn new(args: &[String]) -> Arguments {
        Arguments {
            target: args[1].clone(),
            path: args[2].clone()
        }
    }
}

fn get_args() -> Arguments {
    // Get arguments from env.args...
    let arguments: Vec<String> = env::args().collect();

    // Returns the structure with the recollected data.
    Arguments::new(&arguments)
}

fn main() {
    let arguments = get_args();

    println!("{arguments:#?}\n");

    let text = fs::read_to_string(arguments.path)
        .expect("File can't be readed...");

    println!("The text is:\n{text}");
}
