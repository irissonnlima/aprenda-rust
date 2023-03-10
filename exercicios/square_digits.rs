fn square_digits(num: u64) -> u64 {
    let mut divisor = num;
    let mut resto:u64;
    let mut order:u32 = 0;
    let mut resp:u64 = 0;

    while divisor > 0{
        resto = divisor % 10;

        resp += resto*resto * 10_i32.pow(order) as u64;
        if resto>3{
            order+=2;
        }else{
            order+=1;
        }
        divisor /= 10;
    }

    return resp;
}
// comunidade
fn square_digits_comunidade_1(num: u64) -> u64 {
    num
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("result isnt u64 parsable")
}

fn square_digits_comunidade_2(mut num: u64) -> u64 {
    let mut res = 0;
    let mut mul = 1;
    while num != 0 {
        let m = if num % 10 < 4 { 10 } else { 100 };
        res += (num % 10).pow(2) * mul;
        mul *= m;
        num /= 10;
    }
    res
}

fn square_digits_comunidade_3(mut num: u64) -> u64 {
    let mut res = 0;
    let mut shift = 1;
    while num != 0 {
        let x = num % 10;
        res += x * x * shift;
        num /= 10;
        shift *= if x > 3 {100} else {10};
    }
    res
}