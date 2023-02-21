use caesar_cipher::*;
use std::io;
use std::io::Write;
use playfair::*;

mod caesar_cipher;
mod playfair;

fn main() {
    let mut string = String :: new();
/*
    let key :i32 = in_and_parse_num();
    print!("your key is {}. Enter a word to hide: ", key);
    io::stdout().flush().expect("an error..");
    io::stdin().read_line(&mut string).expect("error!");

    let chipher_str :String= caesar(key, string.trim().to_string());
    println!("зашифрованная строка: {:?}", chipher_str);
    let key :i32 = in_and_parse_num();

    print!("Ключ: {}. Какое слово расшифровать? ---> ", key);
    io::stdout().flush().expect("an error..");
    string.clear();
    io::stdin().read_line(&mut string).expect("error!");

    let original_str :String= caesar_to_original(key, string.trim().to_string());
    println!("расшифрованная строка: {:?}", original_str);

    // print two alphabets to check:
    print_alphabets(key);
*/
    println!("\n--------------------------------------------------------------");
    string.clear();
    println!("Enter a key word for Playfair: ");
    io::stdin().read_line(&mut string).expect("error!");

    let mut str :String = String::new();
    println!("Phrase to encrypt: ");
    io::stdin().read_line(&mut str).expect("error!");

//clocks will run more quickly during free time
    playfair(string.trim().to_string(), str.trim().to_string());


}
