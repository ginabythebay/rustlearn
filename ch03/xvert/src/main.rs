use std::io;

const PROMPT: &str = "Enter something like 31f or 20c to have it converted.  'quit' to quit";

fn main() {
    println!("Hello, world!");

    println!("{}", PROMPT);
    loop {
        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        let cmd = cmd.trim();

        if cmd.is_empty() {
            println!("Blank entry found.\n{}", PROMPT);
            continue;
        }
        if cmd == "quit" {
            println!("Thanks for playing!");
            break;
        }
        if cmd.ends_with("c") {
            let temp: f64 = match cmd.trim_end_matches("c").parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Unable to treat {cmd} as temp.\n{}", PROMPT);
                    continue;
                }
            };

            let result = ((temp * 2.0) * 0.9) + 32.0;

            println!("{temp}c is {result}f");
        } else if cmd.ends_with("f") {
            let temp: f64 = match cmd.trim_end_matches("f").parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Unable to treat {cmd} as temp.\n{}", PROMPT);
                    continue;
                }
            };

            let result = ((temp - 32.0) / 0.9) / 2.0;

            println!("{temp}f is {result}c");
        } else {
            println!("Enter a number followed by 'c' or 'f'.\n{}", PROMPT);
        }
    }
}
