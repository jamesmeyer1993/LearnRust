mod lib {

    pub struct PrimeFinder {
        pub data: Vec<u64>,
        pub found: u32,
    }

    impl PrimeFinder {
        pub fn new(capacity: usize) -> PrimeFinder {
            let mut pf = PrimeFinder {
                data: Vec::with_capacity(capacity),
                found: 0,
            };
            pf.data.insert(0, 2);
            pf.data.insert(1, 3);
            pf.data.insert(2, 5);
            pf.data.insert(3, 7);
            return pf;
        }

        pub fn run(&mut self, to: Option<u32>){
            match to {
                Some(x) => self.gen_primes(x),
                None => {
                    let to: u32 = self.data.capacity() as u32;
                    self.gen_primes(to);
                },
            }
        }

        fn gen_primes(&mut self, to: u32){

            let mut i: u64 = 9;
            let mut isprime: bool;
            let mut len = self.data.len();
            while i <= to as u64 {
                isprime = true;

                for j in 0..self.data.len() {
                    match self.data.get(j) {
                        Some(x) => {
                            if (i % x) == 0 {
                                isprime = false;
                                break;
                            }
                        },
                        None => panic!("Index out of bounds!"),
                    }
                }

                assert!(len == self.data.len());
                if isprime {
                    self.data.insert(len, i);
                    len+=1;
                }
                i+=2;
            }
        }
    }

}

fn main() {

    let mut pf = lib::PrimeFinder::new(100);
    pf.run(None);

    pf.run(Some(1000));

    {
        let mut count = 0;
        for i in pf.data.iter() {
            println!("{}: {}", count, i);
            count+=1;
        }
        println!("data.len(): {}\tdata.capacity(): {}", pf.data.len(), pf.data.capacity() as u32);
    }
    // {
    //     let mut v: Vec<u8> = Vec::new();
    //     for i in 0..26 {
    //         v.push(i);
    //     }
    //     assert!(v.len() == 26);
    //
    //     let first: &u8 = &v[0];
    //     println!("first = {}", first);
    //
    //     let first: Option<&u8> = v.get(0);
    //     match first {
    //         Some(x) => println!("first = {}", x),
    //         None => println!("no value at 0th index"),
    //     }
    // }
    //
    // {
    //     let primes = vec![2, 3, 5, 7, 11];
    //     let hundredth = primes.get(100); // returns an Optional<&u32>
    //     match hundredth {
    //         Some(x) => println!("hundredth is {}", x),
    //         None => println!("No value exists at the 100th index"),
    //     }
    // }
    //
    // {
    //     let mut bshifts: Vec<u64> = Vec::new();
    //     for i in 0..32 {
    //         bshifts.push(i);
    //     }
    //
    //     // print the values
    //     for i in &bshifts {
    //         println!("{}", i);
    //     }
    //
    //     // modify the values of bshifts
    //     for i in &mut bshifts {
    //         *i = 1 << *i;
    //     }
    //
    //     // print the values
    //     for i in &bshifts {
    //         println!("{}", i);
    //     }
    // }
}
