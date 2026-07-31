pub fn run() {
    //- if as an expresssion-
    let number = 7;
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{}  is  {}", number, description);

    //- loop with break value-
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // loop returns a value
        }
    };
    println!("loop result:{}", result);

    //-while-
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("first power of 2 >= 100: {}", n);

    // - for over a range-
    let sum: i32 = (1..=100).sum();
    println!("sum 1 ..= 100={}", sum);
    // TOD 1: using a for loop and a range, print the multiplication table for 7 (7x1 through 7x12).
}
