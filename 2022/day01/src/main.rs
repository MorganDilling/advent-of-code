use std::fs;

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(contents) => {
            let lines = contents.split("\n");
            let lines = lines.collect::<Vec<&str>>();
            let mut highest = 0;

            let mut current_integer = 0;

            for line in &lines {
                let length = line.len();
                if length == 0 {
                    highest = if current_integer > highest { current_integer } else { highest };
                    current_integer = 0;
                    continue;
                }

                let number: i32 = match line.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        continue;
                    }
                };

                current_integer += number;
            }

            println!("Highest: {}", highest)
        }
        Err(error) => {
            println!("Error whilst reading: {error}");
        }
    }
}