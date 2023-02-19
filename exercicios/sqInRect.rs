// The drawing below gives an idea of how to cut a given "true" rectangle into squares ("true" rectangle meaning that the two dimensions are 
// different).

// alternative text

// Can you translate this drawing into an algorithm?

// You will be given two dimensions

// a positive integer length
// a positive integer width
// You will return a collection or a string (depending on the language; Shell bash, PowerShell, Pascal and Fortran return a string) with the size 
// of each of the squares.

// Examples in general form:
// (depending on the language)

//   sqInRect(5, 3) should return [3, 2, 1, 1]
//   sqInRect(3, 5) should return [3, 2, 1, 1]

//   You can see examples for your language in **"SAMPLE TESTS".**
// Notes:
// lng == wdth as a starting case would be an entirely different problem and the drawing is planned to be interpreted with lng != wdth. 
// (See kata, Square into Squares. Protect trees! http://www.codewars.com/kata/54eb33e5bc1a25440d000891 for this problem).

// When the initial parameters are so that lng == wdth, the solution [lng] would be the most obvious but not in the spirit of this kata so,
// in that case, return None/nil/null/Nothing or return {} with C++, Array() with Scala, [] with Perl, Raku.

// In that case the returned structure of C will have its sz component equal to 0.

// Return the string "nil" with Bash, PowerShell, Pascal and Fortran.

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None
    }

    let mut response:Vec<i32> = Vec::new();

    let mut par;
    let mut min = std::cmp::min(lng, wdth);
    let mut max = std::cmp::max(lng, wdth);

    while min > 0{
        response.push(min);
        max -= min;

        par = min;
        min = std::cmp::min(max, min);
        max = std::cmp::max(max, par);
    }

    Some(response)
}

fn main(){
    if let Some(val) = sq_in_rect(14,20){
        println!("{:?}", val)
    }else{
        println!("Valores iguais!")
    }
}

// Comunidade

use std::cmp;

fn sq_in_rect_comunidade(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth { return None; }

    let large = cmp::max(lng, wdth);
    let small = cmp::min(lng, wdth);

    match sq_in_rect(small, large - small) { //recursão usando match
        None => Some(vec![small, small]),
        Some(x) => Some([vec![small], x].concat())
    }
}