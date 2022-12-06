use std::fs;
use std::str;

    const PRIORITIES: [char; 52] = [ 'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z', 'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
fn main() {
    //read file
    let rucksacks: i32 = fs::read_to_string("input.txt")
        .expect("should read file")
        .lines() //split on new line
        .map(|sack: &str| {
            // split string in half
            let (comp1, comp2): (&str, &str) = sack.split_at(sack.chars().count() / 2);
            let x = comp1.chars().find(|c| comp2.contains(*c)).unwrap();
            let z = PRIORITIES.iter().position(|c| *c == x).unwrap() + 1; // get the priority from the array for it
            
          z as i32
        })
        .sum();

    println!("{rucksacks}");
}