// Jamie is a programmer, and James' girlfriend. She likes diamonds,
// and wants a diamond string from James. Since James doesn't know 
// how to make this happen, he needs your help.
// 
// Task
// You need to return a string that looks like a diamond shape when 
// printed on the screen, using asterisk (*) characters. Trailing spaces
// should be removed, and every line must be terminated with a newline
// character (\n).
// 
// Return null/nil/None/... if the input is an even number or negative,
// as it is not possible to print a diamond of even or negative size.
// 
// Examples
// A size 3 diamond:
// 
//  *
// ***
//  *
// ...which would appear as a string of " *\n***\n *\n"
// 
// A size 5 diamond:
// 
//   *
//  ***
// *****
//  ***
//   *
// ...that is:
// 
// "  *\n ***\n*****\n ***\n  *\n"

fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }
    let mut diamond = String::new();

    for i in (1..=n).step_by(2) {
        diamond.push_str(" ".repeat(((n - i) / 2) as usize).as_str());
        diamond.push_str("*".repeat(i as usize).as_str());
        diamond.push_str("\n");
    }

    for i in (1..=n-2).rev().step_by(2) {
        diamond.push_str(" ".repeat(((n - i) / 2) as usize).as_str());
        diamond.push_str("*".repeat(i as usize).as_str());
        diamond.push_str("\n");
    }

    Some(diamond)
}

fn main(){
    if let Some(v) = print(5){
        println!("{}", v)
    }
}

// SOluções da comunidade

fn print_comunidade_1(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let n = n as usize;
    let diamond = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();

    Some(diamond)
}

// Chain -> encadeia iteradores
// step_by -> muda o passo de um iterador