use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("test_2.txt")?;
    let reader = io::BufReader::new(file);

    // Mapping alphabetic words to their numeric values
    let mut word_to_num = HashMap::new();
    word_to_num.insert("one", 1);
    word_to_num.insert("two", 2);
    word_to_num.insert("three", 3);
    word_to_num.insert("four", 4);
    word_to_num.insert("five", 5);
    word_to_num.insert("six", 6);
    word_to_num.insert("seven", 7);
    word_to_num.insert("eight", 8);
    word_to_num.insert("nine", 9);

    let mut total_sum = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut current_number = String::new();
            for c in line.chars() {
                if c.is_digit(10) {
                    current_number.push(c);
                } else {
                    if !current_number.is_empty() {
                        let number: u32 = current_number.parse().unwrap_or(0);
                        total_sum += number;
                        current_number.clear();
                    }

                    let word: String = line.chars().filter(|&c| c.is_alphabetic()).collect();
                    if !word.is_empty() {
                        if let Some(&num) = word_to_num.get(&word.as_str()) {
                            total_sum += num;
                        }
                    }
                }
            }

            if !current_number.is_empty() {
                let number: u32 = current_number.parse().unwrap_or(0);
                total_sum += number;
            }
        }
    }

    println!("Total Sum of value is: {total_sum}");

    Ok(())
}
