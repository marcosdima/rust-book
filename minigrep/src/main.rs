use std::{ self, env, fs, process, error::{ Error } };

struct Arguments {
    target: String,
    path: String,
}

impl Arguments {
    fn build(args: &[String]) -> Result<Arguments, &'static str> {
        let expected = 3;
        let received = args.len();

        // Validate the number of arguments.
        if received < expected {
            return Err("Not enough arguments!")
        } else if received > expected {
            return Err("Too many arguments!")
        }

        Ok(
            Arguments {
                target: args[1].clone(),
                path: args[2].clone()
            }
        )
    }
}

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

    run(arguments);
}

fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(args.path)?;

    println!("The text is:\n{text}");

    Ok(())
}
