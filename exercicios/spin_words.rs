// Write a function that takes in a string of one or more words, and returns the same string, 
// but with all five or more letter words reversed (Just like the name of this Kata). Strings 
// passed in will consist of only letters and spaces. Spaces will be included only when more 
// than one word is present.

// Examples:

// spinWords( "Hey fellow warriors" ) => returns "Hey wollef sroirraw" 
// spinWords( "This is a test") => returns "This is a test" 
// spinWords( "This is another test" )=> returns "This is rehtona test"

fn spin_words(words: &str)->String{
    words.split(" ")
    .map(|s| if s.len() >= 5{
        s.chars().rev().collect()
    } else {
        s.to_string() // deve-se criar uma nova String.
    })
    .collect::<Vec<String>>()
    .join(" ")
}

fn main(){
    println!("{:?}", spin_words("Just kidding there is still one more"))
}

// soluções da comunidade

fn spin_words_comunidade_1(words: &str)->String{
    words.split_whitespace()
    .map(|word| match word.len() >= 5{
        true => word.chars().rev().collect(),
        false=> word.to_string()
    })
    .collect::<Vec<String>>()
    .join(" ")
}