use caesar_cipher::*;
use std::io;
use std::io::Write;
use playfair::*;

mod caesar_cipher;
mod playfair;
mod salsa20;

fn main() {


    //SALSA20 with 10 rounds of transformation
    
    let my_str = "Roses are lovely flowers with a 
    sweet scent that has captivated people for centuries.
    И ещё немного кириллицы для проверки!";
    

    println!("_______ENCRYPTING________");
    
    let secret_byte_stream = salsa20::encrypt(my_str);
    println!("{:?}", secret_byte_stream);

    println!("_______DECRYPTING________");

    let decripted_text = salsa20::decrypt(secret_byte_stream);
    println!("{}", decripted_text);

//-------------------------------------------
    salsa20::set_key("newkeyforsalsa20nobodycandecrypt");
    salsa20::set_nonce("ABCDEFGH");

    let mut text = String::new();
    println!("enter your text -> ");
    io::stdin().read_line(&mut text).expect("stdin error!");

    println!("_______ENCRYPTING________");
    
    let secret_byte_stream = salsa20::encrypt(&text);
    println!("{:?}", secret_byte_stream);

    println!("_______DECRYPTING________");

    let decripted_text = salsa20::decrypt(secret_byte_stream);
    println!("{}", decripted_text);



    /* //CAESAR
    let mut string = String :: new();

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
/* //PLAYFAIR
    println!("\n--------------------------------------------------------------");
    string.clear();
    println!("Enter a key word for Playfair: ");
    io::stdin().read_line(&mut string).expect("error!");

    let mut str :String = String::new();
    println!("Phrase to encrypt: ");
    io::stdin().read_line(&mut str).expect("error!");

//clocks will run more quickly during free time
    let secret : String = playfair(string.trim().to_string(), str.trim().to_string());

    string.clear();
    println!("secret: {secret}\nEnter a key word for decryption: ");
    io::stdin().read_line(&mut string).expect("error!");

    str.clear();
    println!("Phrase to deccrypt: ");
    io::stdin().read_line(&mut str).expect("error!");

    please_help_i_wanna_sleep(string.trim().to_string(), str.trim().to_string());
*/

}
