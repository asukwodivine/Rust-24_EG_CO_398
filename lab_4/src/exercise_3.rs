pub fn run() {
    fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(f(x))
    }

    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    fn is_prime(num: u32) -> bool {
        if num < 2 {
            return false;
        }
        (2..=(num as f64).sqrt() as u32).all(|i| num.is_multiple_of(i))
    }

    let double = |x| x * 2;
    println!("apply_twice(double, 3) = {}", apply_twice(double, 3));

    let add10 = make_adder(10);
    println!("add10(5) = {}", add10(5));

    let result: Vec<String> = (1..=20)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5)
        .map(|x| format!("{:4}", x))
        .collect();
    println!("First 5 even squares: {:?}", result);

    let primes: Vec<u32> = (1..=20).filter(|&x| is_prime(x)).collect();
    println!("Prime numbers from 1 to 20: {:?}", primes);
}
