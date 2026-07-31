pub fn run() {
// -move sementics---
 let s1 = String::from("hello");
 let s2 = s1;     // s1 is MOVED into s2
 // println!("{}", s1); // <- this would NOT compile. try it!
 println!("s2 = {}", s2);

 // - Clone (deep copy) - 
 let s3 = String :: from ("world");
 let s4 = s3. clone();
 println!("s3 = {}, s4 = {}", s3, s4);   // Both valid

 // - Borrowing (immutable reference) -
 let s5 = String ::from ("Rust is great");
 let length = claculate_length(&s5);          // pass reference
 println!("'{}' has {} cahracters", s5, length);

 //TOD 5:write a function ' first_word (& str) -> &str
 //      that return a slice of the first word
 //      call it here and print the result.
}
 fn claculate_length(s: &Str) -> usize{
     s.len()   // s is borrowed; not dropped at end of scope 
 }

 // TOD 5: implement first_word below