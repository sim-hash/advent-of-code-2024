use crate::day1::{duplicates::{contains_duplicate, contains_duplicate_set}, anagram::is_anagram};

mod day1;

fn main() {
    let bool = contains_duplicate(vec![1, 2, 3, 1]);
    println!("{}", bool);

    let bool = contains_duplicate_set(vec![1, 2, 3, 1]);
    println!("{}", bool);



    let s = String::from("anagram");
    let t = String::from("nagrama");
    let bool = is_anagram(s, t);
    println!("{:?}", bool)
}

