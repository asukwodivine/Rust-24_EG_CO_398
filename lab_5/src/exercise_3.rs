pub fn run() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() >= y.len() { x } else { y }
    }

    let s1 = "Hello";
    let s2 = "Goodbye";
    println!("Exercise 3 longest: {}", longest(s1, s2));

    let text = "First sentence. Second sentence.";
    println!("Exercise 3 first sentence: {}", first_sentence(text));
}

fn first_sentence(s: &str) -> &str {
    match s.find('.') {
        Some(i) => &s[..i],
        None => s,
    }
}
