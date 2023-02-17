// Your task is to sort a given string. Each word in the string will contain a single number. This number is the position the word should have in the result.
// Note: Numbers can be from 1 to 9. So 1 will be the first word (not 0).
// If the input string is empty, return an empty string. The words in the input String will only contain valid consecutive numbers.
// ex: "is2 Thi1s T4est 3a"  -->  "Thi1s is2 3a T4est"

fn remove_chars(word: &str)-> String{
    word.chars().filter(|c| c.is_numeric()).collect::<String>()
}

fn order(sentence: &str) -> String {
    let mut words = sentence.split(" ").collect::<Vec<&str>>();
    words.sort_by( |a,b| remove_chars(a).cmp(&remove_chars(b)) );
    words.join(" ")
}

fn main(){
    println!("{:?}", order("ir1sson li3ma 2nascimento"))
}

// Solução da comunidade
use itertools::Itertools;

fn order(s: &str) -> String {
    s.split_whitespace()
        .sorted_by_key(|word| word.chars().find_map(|c| c.to_digit(10)))
        .join(" ")
}

