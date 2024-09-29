use std::io;
use regex::Regex;

fn main() {
    // Collect user input
    println!("Enter your text below (right click to paste):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: no text provided");

    let exclude_symbols = [
        // Titles
        "mr.", "mrs.", "ms.", "messrs.", "mmes.", "msgr.", 
        "prof.", "dr.", "esq.", "rev.", "sr.", "jr.", "st.", "mlle.", "mme.",
        // Streets
        "ave.", "blvd.", "bldg.", "crt.", "cres.", "dr.", "pl.", "rd.", "sq.",
        "stn.", "st.", "terr."
    ];

    // Let's replace all of these symbols in the text with a dummy word for counting
    // to avoid confusing these as additional sentences
    let mut prepared_text = input.trim().to_lowercase();
    for symbol in exclude_symbols {
        prepared_text = prepared_text.replace(symbol, "dummy");
    }

    // Remove bootleg ellipses, and replace question marks and exclamation marks
    prepared_text = prepared_text.replace("...", "");
    prepared_text = prepared_text.replace("?", ".");
    prepared_text = prepared_text.replace("!", ".");

    // Define our sentence lengths and corresponding label
    let sentence_types = [
        SentenceType {
            label: "T".to_string(),
            min_length: 1,
        },
        SentenceType {
            label: "S".to_string(),
            min_length: 5,
        },
        SentenceType {
            label: "M".to_string(),
            min_length: 11,
        },
        SentenceType {
            label: "L".to_string(),
            min_length: 26,
        },
    ];
    let mut sentence_counts = vec![0; sentence_types.len()];

    // Iterate over sentences in text to find their lengths
    for mut sentence in prepared_text.split(".").map(|s| s.trim().to_string()) {

        // Let's discount any citations in brackets
        let mut to_remove = Vec::new();
        let re = Regex::new(r"\(([^)]+)\)").unwrap();
        for capture in re.captures_iter(&sentence) {
            let text_in_brackets = &capture[1];

            // This cryptic expression denotes any correct MLA citation format
            // We will match on the following formats:
            // (2)
            // (5 Author)
            let citation_re = Regex::new(r"^\d+(\s[A-Z][a-zA-Z]*)?$").unwrap();
            if citation_re.is_match(text_in_brackets) {
                let index = sentence.find(text_in_brackets).unwrap();

                // Remove one additional character in each direction to account
                // for the brackets
                to_remove.push((index - 1, index + text_in_brackets.len() + 1))
            }
        }
        
        // Remove the citation we found to not count them against the word count
        // We'll iterate backwards since the ranges of later citations will change 
        // when removing earlier ones and we don't want that
        for &(start, end) in to_remove.iter().rev() {
            sentence.replace_range(start..end, "");
        }

        // Find the first sentence type that supports less than or equal to this sentence's count
        // Here we need to iterate backwards and subtract the index
        let word_count = sentence.split_whitespace().count();
        if let Some(matching_index) = sentence_types.iter().rev().enumerate().position(
            |t| t.1.min_length <= word_count
        ) {
            sentence_counts[sentence_types.len() - 1 - matching_index] += 1;
        }
    }

    // Print our each sentence type and its count
    for (i, sentence_type) in sentence_types.iter().enumerate() {
        println!("{} - {}", sentence_type.label, sentence_counts[i])
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}

struct SentenceType {
    label: String,
    min_length: usize,
}


    // TODO: Unit tests