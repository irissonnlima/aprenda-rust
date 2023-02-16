fn odd_or_even(numbers: Vec<i32>) -> String {
    let sum_values:i32 = numbers.iter().sum();
    if (sum_values%2) == 0{
        return "even".to_string();
    }else{
        return "odd".to_string();
    }
}

//Solução da comunidade
fn odd_or_even_comunidade_1(numbers: Vec<i32>) -> String {
    (if numbers.iter().sum::<i32>() % 2 == 0 {"even"} else {"odd"}).to_string()
}

fn odd_or_even_comunidade_2(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() % 2 == 0 {
        true => "even",
        false => "odd"
    }.to_string()
}

fn odd_or_even_comunidade_3(numbers : Vec<i32>) -> String {
    let sum : i32 = numbers.iter().sum();
    match (sum % 2) {
	  0 => "even".to_string(),
	  _ => "odd".to_string()
    }
}