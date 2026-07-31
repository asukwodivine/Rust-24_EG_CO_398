pub fn run() {
    //immutable variable
    let x = 5;
    println!("x ={}", x);

    // mutable variable
    let mut y = 10;
    println!("y before = {}", y);
    y += 5;
    print!("y after = {}", y);

    // TOD 1: Declare a float f64 called 'pi' with value 3.14159
    // TOD 2: Declare a boolean called 'is_learning' set to true
    // TOD 3: DECLARE a char called 'grade' set to 'A'
    // TOD 4: Print all three variabless using println!

    // shodowing
    let z = "42"; // & str
    let z: u32 = z.parse().expect("not a number!"); // u32
    println!("perse z = {}", z);
}
