

fn main() {

    {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..26 {
            v.push(i);
        }
        assert!(v.len() == 26);

        let first: &u8 = &v[0];
        println!("first = {}", first);

        let first: Option<&u8> = v.get(0);
        match first {
            Some(x) => println!("first = {}", x),
            None => println!("no value at 0th index"),
        }
    }

    {
        let primes = vec![2, 3, 5, 7, 11];
        let hundredth = primes.get(100); // returns an Optional<&u32>
        match hundredth {
            Some(x) => println!("hundredth is {}", x),
            None => println!("No value exists at the 100th index"),
        }
    }

    {
        let mut bshifts: Vec<u64> = Vec::new();
        for i in 0..32 {
            bshifts.push(i);
        }

        // print the values
        for i in &bshifts {
            println!("{}", i);
        }

        // modify the values of bshifts
        for i in &mut bshifts {
            *i = 1 << *i;
        }

        // print the values
        for i in &bshifts {
            println!("{}", i);
        }
    }
}
