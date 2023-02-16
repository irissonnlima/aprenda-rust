fn high(input: &str) -> &str {
    let mut response = String::new();
    let mut max_word = String::new();
    let mut value:u32 = 0;
    let mut max_value:u32 = 0;

    for c in input.chars(){
        let c_int = c as u32;
        if c_int > 96 && c_int < 123{
            value += c_int - 96;
            response.push(c);
        }
        else if value > max_value{
            max_value = value;
            value = 0;
            max_word = response.clone();
            response = "".to_string();
        }
    }
    if max_word == ""{
        max_word = response.clone();
    }
    Box::leak(max_word.into_boxed_str())
}

fn main(){
    println!("{:?}", high("abad zzzz"))
}