use lazy_static::lazy_static;
use std::sync::Mutex;

//defining key, nonce and const salsa
lazy_static! {
    static ref KEY: Mutex<String> = Mutex::new("ThisStringHasACapacityOf32BytesX".to_string());
    static ref NONCE: Mutex<String> = Mutex::new("64bitNonc".to_string());
}
static SALSA20: [u32; 4] = [0x61707865,0x3320646e, 0x79622d32, 0x6b206574];//salsa20 const 128-bit


//setters for key & nonce:
pub fn set_key(s: &str) {
    if s.len() == 32 {
        *KEY.lock().unwrap() = s.to_string();
    }
}
pub fn set_nonce(s: &str) {
    if s.len() == 32 {
        *NONCE.lock().unwrap() = s.to_string();
    }
}


//to do: add size restrictions to text and expansion = (2^64)-1
pub fn encrypt(str: &str) -> Vec<u8> {

        let mut decripted_bytes: Vec<u8> = Vec::new();
        let mut increment: u64 = 0;
        let mut expansion = salsa20_expansion(0);
        for (i, byte) in str.as_bytes().iter().enumerate() {
            if i%64 == 0 {
                increment += 1;
                expansion = salsa20_expansion(increment);
            }
            let c = byte ^ expansion[(i+64)%64];
            decripted_bytes.push(c);
        }

    // let mut decripted_text = String::new();
    // for i in &mut decripted_bytes.iter() {
    //     decripted_text.push(char::from(*i));
    // }

    decripted_bytes
}
    


pub fn decrypt(stream: Vec<u8> ) -> String {


    let mut decripted_bytes: Vec<u8> = Vec::new();
        let mut increment: u64 = 0;
        let mut expansion = salsa20_expansion(0);

        for (i, c) in stream.iter().enumerate() {
            if i%64 == 0 {
                increment += 1;
                expansion = salsa20_expansion(increment);
            }
            let m = expansion[(i+64)%64] ^ c;
            decripted_bytes.push(m);
        }
    let decripted_string = String::from_utf8(decripted_bytes).unwrap();
    decripted_string
}





/* All 32-bit words are viewed as strings in little-endian form. */

fn salsa20_expansion(increment: u64) -> Vec<u8>{ 
    //it's better to override that function to something more beatuful

    //expansion = Const,Key1,Const,Nonce,Increment,Const,Key2,Const
    
    let mut key = KEY.lock().unwrap();
    let mut nonce = NONCE.lock().unwrap();

    //getting 4-byte words in little-endian order for key and nonce:
    let key_array: [u32; 8] = key.as_bytes().chunks_exact(4)
    .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
    .collect::<Vec<u32>>()
    .try_into()
    .unwrap();

    let mut nonce_array: Vec<u32> = nonce.as_bytes().chunks_exact(4)
    .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
    .collect::<Vec<u32>>();

    let increment_as_bytes: [u8; 8] = increment.to_le_bytes().to_vec().try_into().unwrap();
    let mut increment_as_words: Vec<u32> = increment_as_bytes.chunks_exact(4)
    .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
    .collect::<Vec<u32>>();


    //getting a key block 16 words == salsa20 expansion:
    let mut input_block_512: Vec<u32> = Vec::new();
    input_block_512.push(SALSA20[0]);
    input_block_512.append(&mut key_array[0..4].to_vec());
    input_block_512.push(SALSA20[1]);
    input_block_512.append(&mut nonce_array);
    input_block_512.append(&mut increment_as_words);
    input_block_512.push(SALSA20[2]);
    input_block_512.append(&mut key_array[4..8].to_vec());   
    input_block_512.push(SALSA20[3]);  

    //-----------------------------------------------------------------------

    //getting a 2-dimension array of 512-bits-block, that presents 16 words (1word=4byte)
    let mut four_words: Vec<Vec<u32>> = Vec::new();

    let mut temp :Vec<u32> = Vec::new();
    for (i, val) in input_block_512.iter().enumerate() {
        if i%4==0 && i!=0 {
            four_words.push(temp.clone());
            temp.clear();
        }
        temp.push(*val);
    }
    four_words.push(temp.clone());

    salsa20_hash(&mut four_words)
    

}


/*
The wrapping_add method performs the addition
 and wraps around to 0 if an overflow occurs. This is 
 equivalent to performing the sum of modulo 2^32 */

fn quaterround(four_words: &mut Vec<u32>) -> Vec<u32> {

    let mut vec: Vec<u32> = Vec::new();

    let z_1: u32 = four_words[1] ^ four_words[0].wrapping_add(four_words[3]).rotate_left(7);
    let z_2: u32 = four_words[2] ^ (z_1.wrapping_add(four_words[0])).rotate_left(9);
    let z_3: u32 = four_words[3] ^ (z_2.wrapping_add(z_1)).rotate_left(13);
    let z_0: u32 = four_words[0] ^ (z_3.wrapping_add(z_2)).rotate_left(18);
    vec.append(&mut [z_0, z_1, z_2, z_3].to_vec());

    vec
}



fn rowround(byte_array: &mut Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    let mut array_z: Vec<Vec<u32>> = Vec::new();

    for (shift, vect) in byte_array.iter().enumerate() {
        let mut tmp = vect.clone();
        tmp.rotate_left(shift);
        let mut z_i = quaterround(&mut tmp);
        z_i.rotate_right(shift);
        array_z.push(z_i);
    }
    array_z
}



fn columnround(byte_array: &mut Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    let mut vert_x: Vec<Vec<u32>> = Vec::new();
    for i in 0..4{
        let mut tmp: Vec<u32> = Vec::new();
        for j in 0..4 { tmp.push(byte_array[j][i]); }
        vert_x.push(tmp);
    }

    let vert_y: Vec<Vec<u32>> =  rowround(&mut vert_x);

    let mut array_y: Vec<Vec<u32>> = Vec::new();
    for i in 0..4{
        let mut tmp: Vec<u32> = Vec::new();
        for j in 0..4 { tmp.push(vert_y[j][i]); }
        array_y.push(tmp);
    }
    array_y
}



fn doubleround(byte_array: &mut Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    let mut doubleround_array: Vec<Vec<u32>> = Vec::new();
    let mut columnround_array: Vec<Vec<u32>> = columnround(byte_array);
    let mut rowround_array: Vec<Vec<u32>> = rowround(&mut columnround_array);

    doubleround_array.append(&mut rowround_array);

    doubleround_array
}



fn salsa20_hash(byte_array: &mut Vec<Vec<u32>>) -> Vec<u8> {

    let mut salsa20_hash: Vec<u8> = Vec::new();

    let mut doubleround_previous: Vec<Vec<u32>> = doubleround(byte_array); //iteration 0
    for i in 1..10 {
        let round_i: Vec<Vec<u32>> = doubleround(&mut doubleround_previous);
        doubleround_previous = round_i;
    }

    for i in 0_usize..4{
        for j in 0_usize..4{
            salsa20_hash.append(
                &mut (byte_array[i][j].wrapping_add(doubleround_previous[i][j])
                .to_le_bytes().to_vec()));
        }
    }
    salsa20_hash
}



