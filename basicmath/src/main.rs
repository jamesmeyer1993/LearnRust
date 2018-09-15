use std::io;            // read/write to io


fn sum(start: u64, end: u64) -> u64 {

    let mut sum: u64 = 0;
    for i in start..end {
        sum += i;
    }
    return sum;
}

// factorial
fn fac(x: u64) -> u64 {
    if x == 0 || x == 1 {
        return 1;
    }
    let mut result = x;
    let mut i_rev: u64 = 0;
    for i in 1..x {
        // it seems like rust only supports for loops that increment...
        // so this is how I'm implementing a reverse loop for the time being.
        i_rev = x - i;
        result = result * i_rev;
    }
    result = 0;
    if result == 0 {
        panic!("Factorials must not return 0!");
    }
    return result;
}

// usually denoted in math under combinatorics as P(n,r) = n! or = n^r
fn permutations(n: u64, r: u64, repeats: bool) -> u64 {
    if repeats == true {
        return pow(n,r);
    }else{
        return fac(n);
    }
}

fn choose(n: u64, r: u64, repeats: bool) -> u64 {
    let mut result: u64 = 0;
    if repeats == false {
        let x1 = fac(n);
        let x2 = fac(n-r);
        let x3 = fac(r);
        println!("x = {} / ( {} * {} )", x1, x2, x3);
        return result;
    }
    return result;
}

fn pow(x: u64, e: u64) -> u64 {
    if x == 0 {
        return 1;
    }
    else if x == 1 {
        return x;
    }
    else {
        let mut result = x;
        for i in 2..=e {
            result = result * x;
        }
        return result;
    }
}

fn main() {
    println!("BasicMath running.");

    println!("sum 0 100 {}", sum(0,100));
    println!("pow 5 5 {}", pow(5,5));
    println!("fac 3 {}", fac(3));
    println!("fac 5 {}", fac(5));
    println!("permutations of 3 2 false {}", permutations(3,2,false));
    println!("permutations of 3 2 true {}", permutations(3,2,true));
    //println!("choose 21 7 false {}", choose(21,7,false));
    println!("choose 3 3 false {}", choose(3,3,false));
    println!("choose 6 5 false {}", choose(6,5,false));
    //println!("choose 3 9 false {}", choose(3,9,false));
}
