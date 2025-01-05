use std::{ fs, error::{ Error } };

pub struct Arguments {
    pub target: String,
    pub path: String,
}

impl Arguments {
    pub fn build(args: &[String]) -> Result<Arguments, &'static str> {
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

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(args.path)?;

    println!("The text is:\n{text}");

    Ok(())
}