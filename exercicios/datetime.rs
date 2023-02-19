// Write a function, which takes a non-negative integer (seconds) as input and returns the time in a human-readable format (HH:MM:SS)

// HH = hours, padded to 2 digits, range: 00 - 99
// MM = minutes, padded to 2 digits, range: 00 - 59
// SS = seconds, padded to 2 digits, range: 00 - 59
// The maximum time never exceeds 359999 (99:59:59)

// You can find some examples in the test fixtures.

fn make_readable(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = seconds/60 - hours*60;
    let seconds = seconds%60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}


fn main(){
    println!("{:?}", make_readable(7199))
}

// Solução comunidade

fn make_readable_comunidade(s: u32) -> String {
    let (m, s) = (s / 60, s % 60);
    let (h, m) = (m / 60, m % 60);
    format!("{:02}:{:02}:{:02}", h, m, s)
}