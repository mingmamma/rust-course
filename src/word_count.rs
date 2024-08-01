use std::collections::HashMap;

fn main() {
    let sentence = "lazy dog".to_string();

    // iterate over the characters in the sentence
    let mut vowels_map: HashMap<char, usize> = HashMap::new();
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 
            { vowels_map.entry(c).and_modify(|counter| *counter +=1).or_insert(1); }, 
            // { *vowels_map.entry(c).or_insert(0) += 1; },
            _ => continue,
        }
    }

    for (vowel, vowel_count) in &vowels_map {
        println!("{}: {}", vowel, vowel_count);
    }

    // Split and collect into a vector
    let words = sentence.split(' ').collect::<Vec<_>>();
    let longest_word = find_longest_word(&words);

    println!("longest word in sentence: {:?}", longest_word);
}

fn find_longest_word<'a>(words: &'a Vec<&str>) -> &'a str {
    let mut longest = 0;
    let mut longest_word = "";
    for word in words {
        if word.len() > longest {
            longest_word = *word;
            longest = word.len();
        }
    }
    longest_word
}
