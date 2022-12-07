use std::fs;

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(contents) => {
            let lines = contents.split("\n");
            let lines = lines.collect::<Vec<&str>>();

            let mut groups = 0;

            for line in &lines {
                let length = line.len();
                if length == 0 {
                    groups += 1;
                }
            }

            let mut totals: Vec<i32> = vec![0; groups];

            let mut current_integer = 0;

            for line in &lines {
                let length = line.len();
                if length == 0 {
                    // highest = if current_integer > highest { current_integer } else { highest };
                    totals.push(current_integer);
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

            let mut highests: [i32; 3] = [0, 0, 0];

            for total in totals {
                if total > highests[0] {
                    highests[2] = highests[1];
                    highests[1] = highests[0];
                    highests[0] = total;
                } else if total > highests[1] {
                    highests[2] = highests[1];
                    highests[1] = total;
                } else if total > highests[2] {
                    highests[2] = total;
                }
            }

            println!("Top three: {:?}", highests);
            println!("Highest: {}", highests[2]);
            println!("Total sum: {}", highests.iter().sum::<i32>());
        }
        Err(error) => {
            println!("Error whilst reading: {error}");
        }
    }
}