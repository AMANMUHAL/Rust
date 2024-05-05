fn shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    if let Some(shortest) = shortest_word(sentence) {
        println!("The shortest word in the sentence is: {}", shortest);
    } else {
        println!("The sentence is empty.");
    }
}
