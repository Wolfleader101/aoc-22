use std::fs;
use std::str;

fn main() {
    // read file input as string
    let mut elves: Vec<i32> = fs::read_to_string("input2.txt")
        .expect("Should read file")
        .split_whitespace() // split on space to give each elf
        .map(|s: &str| {
            // for each elf
            s.lines() //   split on new line
                .map(|c: &str| {
                    c.parse::<i32>().expect("a valid number") // convert to i32
                })
                .sum() // sum the calories in the array
        })
        .collect::<Vec<i32>>();

    elves.sort_by(|a, b| b.cmp(a)); // sort elves by highest number

    // for elf in elves {
    //     println!("{elf}");
    // }

    // return first element in array
    println!("{}", elves[0]);
}
