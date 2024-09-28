use std::io;

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
    let mut prepared_text = input.to_lowercase();
    for symbol in exclude_symbols {
        prepared_text = prepared_text.replace(symbol, "dummy");
    }

    // TODO: Let's also remove citations
    
    // TODO: Remove ellipses, and replace question marks and exclammation marks

    // TODO: Unit tests

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
    for sentence in prepared_text.split(".") {
        let word_count = sentence.split_whitespace().count();

        // Find the first sentence type that supports less than or equal to this sentence's count
        // Here we need to iterate backwards and subtract the index
        let matching_index = sentence_types.len() - 1 - sentence_types.iter().rev().enumerate().position(
            |t| t.1.min_length <= word_count
        ).unwrap_or_default();
        sentence_counts[matching_index] += 1;
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