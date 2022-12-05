use std::fs;
use std::str;

fn main() {
    // read file input as string
    let mut elves: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Should read file")
        .split("\r\n\r\n") // split on space to give each elf
        .map(|s: &str| {
            // for each elf
            s.lines() //   split on new line
                .filter_map(|l: &str| l.parse::<i32>().ok())
                .sum() // sum the calories in the array
        })
        .collect::<Vec<i32>>();

    elves.sort_by(|a, b| b.cmp(a)); // sort elves by highest number

    // return first element in array
    println!("{}", elves[0] + elves[1] + elves[2]);
}
