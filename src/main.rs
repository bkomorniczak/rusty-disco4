use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn lfsr(initial_state: &[u8], feedback_taps: &[usize], output_length: usize) -> Vec<u8> {
    let mut state = initial_state.to_vec();
    let mut output = Vec::with_capacity(output_length);

    for _ in 0..output_length {
        output.push(state[state.len() - 1]);
        let new_bit = feedback_taps.iter().fold(0, |acc, &tap| acc ^ state[tap]);
        state.rotate_right(1);
        state[0] = new_bit;
    }

    output
}

fn read_file(file_path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn write_file(file_path: &Path, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
}

fn xor_with_key_stream(data: &[u8], key_stream: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_stream[i % key_stream.len()])
        .collect()
}

fn berlekamp_massey(s: &[u8]) -> (usize, Vec<u8>) {
    let n = s.len();
    let mut c = vec![0; n]; // connection
    let mut b = vec![0; n]; //temp
    let mut t = vec![0; n]; //temp
    c[0] = 1;
    b[0] = 1;
    let mut l = 0; //zlozonosc
    let mut m = 0; // pozycja ostatniej zmiany
    let mut n_iter = 0; //pozycja w sekwencji

    while n_iter < s.len() {
        let mut d = s[n_iter];//rozbieznosc
        for i in 1..=l {
            d ^= c[i] * s[n_iter - i];  //rozbieznosc miedzy obecnym bitem a bitem z wielomiany polaczenia
        }
        if d == 1 {  //update wielomian polaczenia
            t.copy_from_slice(&c);
            for i in 0..(n_iter - m) {
                if b[i] == 1 {
                    c[i + n_iter - m] ^= 1; //update c xorujac z przezsunieta wersja b
                }
            }
            if l <= n_iter / 2 { //te updaty z algo
                l = n_iter + 1 - l;
                m = n_iter;
                b.copy_from_slice(&t);
            }
        }
        n_iter += 1;
    }

    (l, c[..=l].to_vec())
}

fn main() -> io::Result<()> {
    let initial_states = vec![
        vec![0, 0, 0, 1],       //  dla 1 + x + x^3
        vec![1, 0, 0, 1, 0, 0], //  dla 1 + x^2 + x^5
        vec![1, 0, 0, 0, 1, 0], //  dla 1 + x + x^3 + x^5
        vec![1, 0, 0, 1, 0]     // dla zadania 2
    ];

    let feedback_taps = vec![
        vec![0, 1, 3],          // 1 + x + x^3
        vec![0, 2, 5],          // 1 + x^2 + x^5
        vec![0, 1, 3, 5],       // 1 + x + x^3 + x^5
    ];


    //Ustawianie wybranego indeksu
    let state_index = 1;
    let taps_index = 1;

    let output_length = 25;

    let key_stream = lfsr(&initial_states[state_index], &feedback_taps[taps_index], output_length);

    let input_file_path = Path::new("src/resources/plain.txt");
    let plaintext = read_file(input_file_path)?;

    let encrypted_file_path = Path::new("src/resources/ciphertext.txt");
    let encrypted_data = xor_with_key_stream(&plaintext, &key_stream);
    write_file(encrypted_file_path, &encrypted_data)?;

    let decrypted_file_path = Path::new("src/resources/decrypted.txt");
    let decrypted_data = xor_with_key_stream(&encrypted_data, &key_stream);
    write_file(decrypted_file_path, &decrypted_data)?;

    let (l, c) = berlekamp_massey(&key_stream);
    println!("Linear complexity: {}", l);
    println!("Connection polynomial: {:?}", c);
    println!("Keystream: {:?}", key_stream);

    Ok(())
}