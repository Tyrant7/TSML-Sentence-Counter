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

    // Now we'll count up each sentence type's length

    println!("You entered: ");
    println!("{}", prepared_text);
}
