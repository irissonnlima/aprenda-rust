fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut numbers = Vec::new();

	for i in (2..(integer-1)).rev(){
        if integer%i == 0{
            numbers.push(i);
        }
    }
    if numbers.len() > 0{
        numbers.reverse();
        Ok(numbers)
    }else{
        let integer = integer.to_string();
        Err(integer.to_owned() + " is prime")
    }
}

fn main(){

    println!("{}", divisors(21))

}

// Forma da comunidade 
fn divisors_comunidade_1(integer: u32) -> Result<Vec<u32>, String> {
	let divs = (2..integer)
                .filter(|k| integer % k == 0)
                .collect::<Vec<u32>>();

    if divs.len() > 0 {
        Ok(divs)
    } else {
        Err(format!("{} is prime", integer))
    }
}

fn divisors_comunidade_2(integer: u32) -> Result<Vec<u32>, String> {
    let divisors: Vec<u32> = (2..integer / 2 + 1).filter(|x| integer % x == 0).collect();

    match !divisors.is_empty() {
        true => Ok(divisors),
        _ => Err(format!("{} is prime", integer))
    }
}

