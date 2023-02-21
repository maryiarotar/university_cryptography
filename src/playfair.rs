

pub(crate) fn playfair(key :String, str :String) -> String{

    let mut array :[[char; 5]; 5] = [['a';5];5];
    create_alphabet(&key, &mut array);
    let mut transform_str :Vec<char> = Vec::new();

    let mut ind :usize = 0;
    for ch in str.to_uppercase().chars() {
        if ch == ' ' {continue;}
        if ind>0 {
            if *transform_str.get(ind-1).unwrap() == ch && (ind%2==1) {
                transform_str.push('X');
                ind += 1;
            }
        }
        transform_str.push(ch);
        ind += 1;
    }
    if ind % 2 != 0 {
        transform_str.push('X');
    }
    let length = ind + 1;

    let mut secret_str = String::new();

    let mut bi :usize = 0;

    'first: while bi<length-1 {

        for i in 0..5 {

            //horizontal match
            if array[i].contains(&transform_str[bi])
                && array[i].contains(&transform_str[bi + 1]) {

                let pos_bi1 = array[i]
                    .iter().position(|&c| c == transform_str[bi]).expect("err!");
                let pos_bi2 = array[i]
                    .iter().position(|&c| c == transform_str[bi + 1]).expect("err!");
                secret_str.push(array[i][(pos_bi1 + 1) % 5]);
                secret_str.push(array[i][(pos_bi2 + 1) % 5]);
                bi += 2;
                continue 'first;
            }
        }
        for i in 0..5 {
            //vertical match
            for j in 0..5 {
                if array[j][i] == transform_str[bi] {
                    for k in 0..5 {
                        if array[k][i] == transform_str[bi + 1] {
                            secret_str.push(array[(j + 1) % 5][i]);
                            secret_str.push(array[(k + 1) % 5][i]);
                            bi += 2;
                            continue 'first;
                        }
                    }
                }
            }
        }
        //square cipher
        for i in 0..5 {
            if array[i].contains(&transform_str[bi]) {
                let ind_bi1: (usize, usize) = (i, array[i].iter().position(|&c| c == transform_str[bi]).expect("err!"));
                for j in 0..5 {
                    if array[j].contains(&transform_str[bi + 1]) {
                        let ind_bi2: (usize, usize) = (j, array[j].iter().position(|&c| c == transform_str[bi + 1]).expect("err!"));
                        secret_str.push(array[ind_bi1.0][ind_bi2.1]);
                        secret_str.push(array[ind_bi2.0][ind_bi1.1]);
                        bi += 2;
                        continue 'first;
                    }
                }
            }
        }

        bi += 2;
    }

    check_this(&transform_str.iter().collect());
    check_this(&secret_str);
    for i in 0..5 {
        println!("{:?}", array[i]);
    }

    return secret_str;
}

//decryption
pub(crate) fn please_help_i_wanna_sleep(key :String, str :String) -> String{

    let mut array :[[char; 5]; 5] = [['a';5];5];
    create_alphabet(&key, &mut array);
    let mut transform_str :Vec<char> = str.chars().collect();

    let mut orig_str = String::new();

    let mut bi :usize = 0;

    'first: while bi< str.len()-1 {

        for i in 0..5 {

            //horizontal match
            if array[i].contains(&transform_str[bi])
                && array[i].contains(&transform_str[bi + 1]) {

                let pos_bi1 = array[i]
                    .iter().position(|&c| c == transform_str[bi]).expect("err!");
                let pos_bi2 = array[i]
                    .iter().position(|&c| c == transform_str[bi + 1]).expect("err!");
                orig_str.push(array[i][(pos_bi1+5 - 1) % 5]);
                orig_str.push(array[i][(pos_bi2+5 - 1) % 5]);
                bi += 2;
                continue 'first;
            }
        }
        for i in 0..5 {
            //vertical match
            for j in 0..5 {
                if array[j][i] == transform_str[bi] {
                    for k in 0..5 {
                        if array[k][i] == transform_str[bi + 1] {
                            orig_str.push(array[(j+5 - 1) % 5][i]);
                            orig_str.push(array[(k+5 - 1) % 5][i]);
                            bi += 2;
                            continue 'first;
                        }
                    }
                }
            }
        }
        //square chipher
        for i in 0..5 {
            if array[i].contains(&transform_str[bi]) {
                let ind_bi1: (usize, usize) = (i, array[i].iter().position(|&c| c == transform_str[bi]).expect("err!"));
                for j in 0..5 {
                    if array[j].contains(&transform_str[bi + 1]) {
                        let ind_bi2: (usize, usize) = (j, array[j].iter().position(|&c| c == transform_str[bi + 1]).expect("err!"));
                        orig_str.push(array[ind_bi1.0][ind_bi2.1]);
                        orig_str.push(array[ind_bi2.0][ind_bi1.1]);
                        bi += 2;
                        continue 'first;
                    }
                }
            }
        }
        bi += 2;
    }
    check_this(&orig_str);

    return orig_str;
}


fn check_this(str :&String) {
        let mut ii :usize = 1;
    for i in str.chars() {
        print!("{}",i);
        if ii%2==0 {
            print!(" ");
        }
        ii += 1;
    }
    println!();
}

fn create_alphabet(str :&String, array :&mut [[char; 5]; 5]) {
    let mut ALPHABET :Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];
    let length = str.len();
    let mut iter  = 0;
    let mut k :usize = 0;

        for i in 0..5 {
        for j in 0..5 {
            if iter<length {
                let ch :char = str.to_uppercase().chars().nth(iter).unwrap();
                iter += 1;
                array[i][j] = ch;
                ALPHABET.retain(|x| *x != ch);
            }
            else {
                array[i][j] = ALPHABET[k];
                k += 1;
            }
        }
    }
}
