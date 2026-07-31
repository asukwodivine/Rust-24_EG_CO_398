use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        if !clean.is_empty() {
            *freq.entry(clean).or_insert(0) += 1;
        }
    }
    freq
}

fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(&String, &usize)> {
    let mut items: Vec<_> = freq.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));
    items.into_iter().take(n).collect()
}

pub fn run() {
    let text = "the quick brown fox jumps over the lazy dog and the dog was very lazy";
    let freq = word_frequency(text);
    println!("Word frequencies: {:?}", freq);
    for (word, count) in top_n(&freq, 5) {
        println!("{} : {}", word, count);
    }
}
