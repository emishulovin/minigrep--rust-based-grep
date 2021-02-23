use std::env;
use std::fs;
use std:: process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)
                .unwrap_or_else(|err| {
                    println!("Problem passing the arguments: {}", err);
                    process::exit(1);
                });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read file.");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Needs more arguments: a string to search for and a filename to search in.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
