fn main() {
    let sample = "abc 123 xyz 987";
    for v in split_words(sample) {
        println!("{:?}", v);
    }
    for v in split_words_idiomatic(sample) {
        println!("{:?}", v);
    }
}

// Implement a function split_words(s: &str) -> Vec<&str> that
// splits on whitespace without allocating new strings (use split()).
fn split_words(s: &str) -> Vec<&str> {
    let mut vec_words: Vec<&str> = Vec::new();
    let words = s.split(" ");
    for word in words {
        vec_words.push(word);
    }
    vec_words
}

fn split_words_idiomatic(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}
