use caesars_cipher::*;
use std::io;
use std::io::Write;

mod caesars_cipher;

fn main() {

    let key :i32 = in_and_parse_num();
    print!("your key is {}. Enter a word to hide: ", key);
    io::stdout().flush().expect("an error..");

    let mut string = String :: new();
    io::stdin().read_line(&mut string).expect("error!");

    let chipher_str :String= caesar(key, string.trim().to_string());
    println!("зашифрованная строка: {:?}", chipher_str);

    let key :i32 = in_and_parse_num();
    print!("Ключ: {}. Какое слово расшифровать? ---> ", key);
    io::stdout().flush().expect("an error..");

    let mut string = String :: new();
    io::stdin().read_line(&mut string).expect("error!");

    let original_str :String= caesar_to_original(key, string.trim().to_string());
    println!("расшифрованная строка: {:?}", original_str);

    // print two alphabets to check:
    print_alphabets(key);
}
