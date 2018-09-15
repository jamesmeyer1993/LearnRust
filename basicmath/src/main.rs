use std::io;            // read/write to io


fn main() {
    println!("BasicMath running.");

    let mut op_add = "add".to_string();
    let mut op_sub = "sub".to_string();
    let mut op_mul = "mul".to_string();
    let mut op_div = "div".to_string();
    let mut op_fac = "fac".to_string();     // factorial
    let mut op_sum = "sum".to_string();
    let mut op_pow = "pow".to_string();     // powers:              pow 2 2 = 4
    let mut op_root = "root".to_string();   // roots of numbers:    root 2 81 = 9
    let mut op_log = "log".to_string();
    let mut op_primes = "primes".to_string();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");   // read in a string

        match input[0] {
            Ok('a') => println!("input={}", op_sub),
            Ok('s') => println!("input={}", op_sub),
            Ok('m') => println!("input={}", op_mul),
            Err(_) => println!("Invalid input!"),
        }
    }
}
