pub fn run() {
    struct Fibonacci {
        a: u64,
        b: u64,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { a: 0, b: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let next = self.a + self.b;
            self.a = self.b;
            self.b = next;
            Some(self.a)
        }
    }

    struct Primes {
        current: u64,
    }

    impl Primes {
        fn new() -> Self {
            Primes { current: 2 }
        }
    }

    impl Iterator for Primes {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            while !is_prime_u64(self.current) {
                self.current += 1;
            }
            let prime = self.current;
            self.current += 1;
            Some(prime)
        }
    }

    fn is_prime_u64(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        (2..=(n as f64).sqrt() as u64).all(|i| !n.is_multiple_of(i))
    }

    let fibs: Vec<u64> = Fibonacci::new().take(15).collect();
    println!("First 15 Fibonacci: {:?}", fibs);

    let first_big = Fibonacci::new().find(|&x| x > 1_000_000).unwrap();
    println!("First fib > 1,000,000: {}", first_big);

    let primes: Vec<u64> = Primes::new().take(10).collect();
    println!("First 10 primes: {:?}", primes);
}
