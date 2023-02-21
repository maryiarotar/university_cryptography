

pub(crate) fn playfair(key :String, str :String) {

    let mut array :[[char; 5]; 5] = [['a';5];5];
    create_alphabet(&key, &mut array);
    let mut transform_str :Vec<char> = Vec::new();

    let mut ind :usize = 0;
    for ch in str.to_uppercase().chars() {
        if ch == ' ' {continue;}
        if ind>0 {
            if *transform_str.get(ind-1).unwrap() == ch && (ind%2==1) {
                transform_str.push('J');
                ind += 1;
            }
        }
        transform_str.push(ch);
        ind += 1;
    }
    if ind % 2 != 0 {
        transform_str.push('J');
    }
    let length = ind + 1;

    let mut secret_str = String::new();

    'first: for mut bi in 0..length-2 {

        'second: for i in 0..5 {
            //horizontal match
            if array[i].contains(&transform_str[bi])
                && array[i].contains(&transform_str[bi+1]){
                    let pos_bi1 = array[i].
                        iter().position(|&c| c == transform_str[bi]).expect("err!");
                    let pos_bi2 = array[i].
                        iter().position(|&c| c == transform_str[bi+1]).expect("err!");
                    secret_str.push(array[i][(pos_bi1+1)%5]);
                    secret_str.push(array[i][(pos_bi2+1)%5]);
                    bi += 2;
                    continue 'first;
            }
            else {
                //vertical match
                for j in 0..5 {
                    if array[j][i] == transform_str[bi] {
                        let mut k0 = j + 1;
                        for k in k0..j {
                            if array[k][i] == transform_str[bi + 1] {
                                secret_str.push(array[(j + 1) % 5][i]);
                                secret_str.push(array[(k + 1) % 5][i]);
                                bi += 2;
                                continue 'first;
                            }
                        }
                    }
                }

         //square chipher
                if array[i].contains(&transform_str[bi]) {
                    let ind_bi1 :(usize, usize) = (i, array[i].iter().position(|&c| c == transform_str[bi]).expect("err!"));
                    for j in 0..5 {
                        if array[j].contains(&transform_str[bi+1]) {
                            let ind_bi2 :(usize, usize) = (j, array[j].iter().position(|&c| c == transform_str[bi + 1]).expect("err!"));
                                secret_str.push(array[ind_bi1.0][ind_bi2.1]);
                                secret_str.push(array[ind_bi2.0][ind_bi1.1]);
                                bi += 2;
                                continue 'first;
                        }
                    }
                }
            }
        }

        bi += 2;
    }


println!(" {:?}", transform_str);
    println!(" {:?}", secret_str);
    for i in 0..5 {
        println!("{:?}", array[i]);
    }

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
