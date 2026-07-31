pub fn run() {
    let s1 = String::from("long string is long");
    let result;

    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("Exercise 1 longest: {}", result);
    }

    let article = String::from("Rust 2024 edition brings many improvements...");
    let imp = Important { content: &article };
    println!("Exercise 1 summary: {}", imp.summarise());

    let text = "First sentence. Second sentence.";
    println!("Exercise 1 first sentence: {}", first_sentence(text));
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn summarise(&self) -> &str {
        &self.content[..self.content.len().min(80)]
    }
}

fn first_sentence(text: &str) -> &str {
    match text.find('.') {
        Some(i) => &text[..i],
        None => text,
    }
}
