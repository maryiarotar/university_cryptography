use std::io;
use std::io::Write;

static ALPHABET: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

pub(crate) fn caesar(key :i32, str :String) -> String {

    let mut chipher_str :String = String::new();
    for ch in str.to_uppercase().chars() {
        let index = ALPHABET.iter().position(|&x| x == ch);
        let index = match index {
            None => {continue},
            Some(i) => {i},
        };
        let ind_chipher :usize = ((26 + index + (key as usize)) % 26) as usize;
        chipher_str.push(ALPHABET[ind_chipher]);
    }
    chipher_str
}

pub(crate) fn caesar_to_original(key :i32, str :String) -> String {

    let key = key as usize;

    let mut chipher_str :String = String::new();
    for ch in str.to_uppercase().chars() {
        let index = ALPHABET.iter().position(|&x| x == ch);
        let index = match index {
            None => {continue},
            Some(i) => {i},
        };

        let ind_chipher :usize = (26 + index - key) % 26;
        chipher_str.push(ALPHABET[ind_chipher]);
    }
    chipher_str
}


//--------------------------------------------------------

pub(crate) fn print_alphabets(key :i32) {
    // print two alphabets to check:
    for i in 0..=25 {
        print!("_{}_", ALPHABET[i]);
    }
    println!();
    let mut i :i32 = key;
    while i < (26+key).into() {
        let ind :usize = ((26 + i) % 26) as usize;
        print!("_{}_", ALPHABET[ind]);
        i += 1;
    }
}



pub(crate) fn in_and_parse_num() -> i32 {

    let mut key = String::new();
loop {
    key.clear();
    print!("enter a key [0, 25] ----> ");
    io::stdout().flush().expect("an error..");

    io::stdin().read_line(&mut key).expect("error!");
    let key: i32 = match key.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    return key;
}

}