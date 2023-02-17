// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in. Additionally, if the number is negative, return 0 (for languages that do have them).

// Note: If the number is a multiple of both 3 and 5, only count it once.



fn solution(num: i32) -> i32 {
    let mut sum: i32=0;
    for number in 1..num{
        if (number%3==0) || (number%5==0){
            sum += number;
        } 
    }
    sum
}

fn main(){
    println!("{:?}", solution(10))
}

// soluções da comunidade

fn solution_comunidade_1(num:i32)->i32{
    (1..num).filter(|x| x%3==0 || x%5==0).sum()
}
