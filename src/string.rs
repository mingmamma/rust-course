use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);

    println!("Can still access sentence: {}", sentence);
    println!("{}", description);

    // use + for string concat
    let description_2 = String::from("Title: Quick story\n") + &sentence;
    println!("{}", description_2);

    // iterate over the characters in the sentence
    let mut vowels_map: HashMap<char, usize> = HashMap::new();
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => *vowels_map.entry(c).or_insert(0) += 1,
            _ => continue,
        }

        // alternative HashMap update approach (if type is no issue)
        // *vowels_map.entry(c).and_modify(|counter| *counter +=1).or_insert(1);
    }

    for (vowel, vowel_count) in &vowels_map {
        println!("{}: {}", vowel, vowel_count);
    }

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    let longest_word = find_longest_word(&words);

    println!("{:?}", words);
    println!("{:?}", longest_word);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}

fn find_longest_word(words: &Vec<&str>) -> String {
    let mut longest = 0;
    let mut longest_word = "";
    for word in words {
        if word.len() > longest {
            longest_word = word;
            longest = word.len();
        }
    }
    longest_word.to_string()
}
