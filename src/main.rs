use std::io;

fn main() {
    let mut num = String::new();
    println!("Sample 1: Prime number checker");

    println!("Enter the number to validate:");

    io::stdin()
        .read_line(&mut num)
        .expect("Unable to read the number.");

    let n: i32 = num
        .trim()
        .parse()
        .expect("Unable to convert the input to an integer.");

    println!("Input number is prime? [{}]", check_prime(n));
}

fn check_prime(_num: i32) -> bool {
    return _num % 2 == 0;
}
