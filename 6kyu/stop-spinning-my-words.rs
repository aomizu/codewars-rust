// Write a function that takes in a string of one or more words, and returns the same string, but with all five or more letter words reversed (Just like the name of this Kata). Strings passed in will consist of only letters and spaces. Spaces will be included only when more than one word is present.

// Examples:

// spinWords( "Hey fellow warriors" ) => returns "Hey wollef sroirraw" 
// spinWords( "This is a test") => returns "This is a test" 
// spinWords( "This is another test" )=> returns "This is rehtona test"

// https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust/643d012167178f000f1e3165

fn spin_words(words: &str) -> String {
    let mut flipped = String::new();
    let sentence = String::from(words);
    for word in sentence.split_whitespace() {
        
        if sentence.split_whitespace().count() == 1 {
            if word.len() > 4 {
                let mut reversed = String::new();
                for character in word.chars() {
                    reversed.insert(0, character);
                }
                flipped = [flipped, reversed].join("");
                return flipped.trim().to_string();
            } else {
                flipped = String::from(word);
                return flipped.trim().to_string();
            }
        }
        if word.len() > 4 {
            let mut reversed = String::new();
            for character in word.chars() {
                reversed.insert(0, character);
            }
            flipped = flipped + &" ".to_string() + &reversed;
        } else {
            flipped = flipped + &" ".to_string() + &String::from(word);
        }
    }
    println!("{}", flipped);
    return flipped.trim().to_string()
    }