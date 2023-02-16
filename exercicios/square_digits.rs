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