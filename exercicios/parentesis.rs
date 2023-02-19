// Write a function that takes a string of parentheses, and determines if the
// order of the parentheses is valid. The function should return true if the string
// is valid, and false if it's invalid.

// Examples
// "()"              =>  true
// ")(()))"          =>  false
// "("               =>  false
// "(())((()())())"  =>  true
// Constraints
// 0 <= input.length <= 100

// Along with opening (() and closing ()) parenthesis, input may contain any valid ASCII characters. 
// Furthermore, the input string may be empty and/or not contain any parentheses at all. Do not treat
// other forms of brackets as parentheses (e.g. [], {}, <>).

fn valid_parentheses(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                if let Some('(') = stack.pop() {
                    continue;
                } else {
                    return false;
                }
            },
            _ => continue,
        }
    }

    stack.is_empty()
}

fn main(){
    println!("{:?}", valid_parentheses(")(())"))
}

// solução da comunidade

fn valid_parentheses_comunidade(s: &str) -> bool {
    s.chars().try_fold(0u8, |a, c| match c {
        '(' => Some(a + 1),
        ')' => a.checked_sub(1),
        _   => Some(a)
    }).map_or(false, |r| r == 0)
}