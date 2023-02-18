// Write a function that takes an integer as input, and returns the number of bits that are equal to one in the binary representation of that number. You can guarantee that input is non-negative.

// Example: The binary representation of 1234 is 10011010010, so the function should return 5 in this case

fn count_bits(n: i64) -> u32{
    let mut number = n;
    let mut count:u32 = 0;
    let bit = 0b0001;
    for _ in 0..64{
        if number & bit == bit{
            count+=1;
        }
        number = number>>1;
    }
    count
}

fn main(){
    println!("{:?}", count_bits(1235))
}

// Soluções da comunidade

fn count_bits_comunidade_1(n: i64)->u32{
    n.count_ones()
}
