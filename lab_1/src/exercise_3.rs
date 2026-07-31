fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon = expression = return value
}
fn greet(name: &str) -> String {
    format!("hello, {}!", name)
}

pub fn run() {
    println!("{}", add(3, 7));
    println!("{}", greet("Rustacean"));

    // TODO 6: write a recursive function 'factorial(n:u64) -> u64'
    //         and print factorial(10).
}
