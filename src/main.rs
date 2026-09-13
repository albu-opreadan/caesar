use clap::Parser;
use std::io::{self, Read};
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input message to encrypt/decrypt (reads from stdin if omitted)
    #[arg(short, long)]
    message: Option<String>,

    /// Key type for the Caesar cypher (1 = simple shift, 2 = shift + alphabet permutation)
    #[arg(short, long, default_value_t = 1)]
    keytype: u8,

    /// First key for the Caesar cypher (the shift), must be 1-25
    #[arg(short = 'n', long, default_value_t = 0)]
    keyone: u8,

    /// Second key for the Caesar cypher (used when keytype = 2), min length 7
    #[arg(short = 't', long, default_value = "")]
    keytwo: String,

    /// "Language" or rather the alphabet where the letters are gotten from
    #[arg(short, long, default_value = "en")]
    lang: String,

    /// Decrypt instead of encrypt
    #[arg(short, long, default_value_t = false)]
    decrypt: bool
}

fn main() {
        let alphabets = [
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        String::from("AĂÂBCDEFGHIÎJKLMNOPQRSȘTȚUVWXYZ"),
    ];
    
    let args = Args::parse();

    let alphabet: String = match args.lang.as_str() {
        "en" => alphabets[0].clone(),
        "ro" => alphabets[1].clone(),
        _ => process::exit(1),
    };

    match args.keytype {
        1 | 2 => {}
        _ => process::exit(2),
    }

    let alength = alphabet.chars().count();

    if args.keyone < 1 || usize::from(args.keyone) > alength - 1 {
        eprintln!("Error: Key needs to be between 1 or {} ", alength);
        process::exit(3);
    }

    if args.keytype == 2 {
        if args.keytwo.chars().count() < 7 {
            eprintln!("Error: 2nd key needs to be at least 7 letters long");
            process::exit(4);
        }
        if !args.keytwo.chars().all(|c| c.is_alphabetic()) {
            eprintln!("Error: 2nd key needs to be composed from characters from the alphabets");
            process::exit(5);
        }
    }
    let raw_message = match args.message {
        Some(m) => m,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
                eprintln!("Error: couldn't read from stdin: {}", e);
                process::exit(7);
            });
            buf
        }
    };

        let message = normalize_text(raw_message);

    if !message.chars().all(|c| alphabet.contains(c)) {
        eprintln!(
            "Error: Invalid character detected, the only valid characters are: {}",
            alphabet
        );
        process::exit(6);
    }

    let result = match args.keytype {
        1 => caesar_one_key(&message, &alphabet, alength, args.keyone, args.decrypt),
        2 => {
            let permuted = build_permuted_alphabet(&alphabet, &normalize_text(args.keytwo));
            caesar_one_key(&message, &permuted, alength, args.keyone, args.decrypt)
        }
        _ => unreachable!(),
    };

    println!("{}", result);
    
}

fn normalize_text(text: String) -> String {
    text.to_uppercase().chars().filter(|c| !c.is_whitespace()).collect()
}

fn modulo(x: i32, y: i32) -> i32 {
    let mut r: i32 = x % y;
    if r < 0 {
        r += y;
    }
    r
}

fn position_in_alphabet(alphabet: &str, ch: char) -> Option<usize> {
    alphabet.chars().position(|c| c == ch)
}

fn build_permuted_alphabet(alphabet: &str, keyword: &str) -> String {
    let mut result = String::with_capacity(alphabet.chars().count());

    for ch in keyword.chars() {
        if alphabet.contains(ch) && !result.contains(ch) {
            result.push(ch);
        }
    }
    for ch in alphabet.chars() {
        if !result.contains(ch) {
            result.push(ch);
        }
    }
    result
}

fn caesar_one_key(message: &str, alphabet: &str, alength: usize, key: u8, decrypt: bool) -> String {
    let mut r = String::with_capacity(message.len());
    let key = key as i32;

    for ch in message.chars() {
        let pos = position_in_alphabet(alphabet, ch).unwrap();
        let shifted = if decrypt {
            modulo(pos as i32 - key, alength as i32)
        } else {
            modulo(pos as i32 + key, alength as i32)
        };
        r.push(alphabet.chars().nth(shifted as usize).unwrap());
    }
    r
}