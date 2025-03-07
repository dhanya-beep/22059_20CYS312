fn extract_word<'a>(sentence: &'a str, word: &'a str) -> &'a str {
    if let Some(start) = sentence.find(word) {
        &sentence[start..start + word.len()]
    } else {
        "Word not found"
    }
}

fn main() {
    let sentence = "Rust is fast and safe.";
    let extracted = extract_word(sentence, "fast");
    println!("Extracted word: {}", extracted);
}
