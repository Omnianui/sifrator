use std::io::{Result, stdin};
use near_enough::*;
use std::{collections::HashMap, hash::Hash};

struct LetterPrapabilitty {
    letter: char,
    propabilitty: f32,
}

impl LetterPrapabilitty {
    const fn new(letter: char, propabilitty: f32) -> LetterPrapabilitty {
        LetterPrapabilitty {
            letter: letter,
            propabilitty: propabilitty,
        }
    }
}

impl<T: Diff + Hash + Eq, V> Closest<T> for HashMap<T, V> {
    type Value = V;

    fn closest(&self, to: &T) -> Option<&Self::Value> {
        let mut keys: Vec<&T> = self.values().into_iter().collect();
        keys.sort_by(|a, b| a.diff(to).cmp(&b.diff(to)));

        match keys.first() {
            Some(key) => self.get(key),
            None => None,
        }
    }
}

const CZECH_PROPABILITTIES: [LetterPrapabilitty; 26] = [
    LetterPrapabilitty::new('e', 0.109041),
    LetterPrapabilitty::new('a', 0.095890),
    LetterPrapabilitty::new('o', 0.080300),
    LetterPrapabilitty::new('i', 0.066861),
    LetterPrapabilitty::new('n', 0.059172),
    LetterPrapabilitty::new('l', 0.057208),
    LetterPrapabilitty::new('s', 0.055860),
    LetterPrapabilitty::new('t', 0.053853),
    LetterPrapabilitty::new('r', 0.043968),
    LetterPrapabilitty::new('v', 0.039525),
    LetterPrapabilitty::new('d', 0.037748),
    LetterPrapabilitty::new('m', 0.036055),
    LetterPrapabilitty::new('u', 0.035790),
    LetterPrapabilitty::new('k', 0.035281),
    LetterPrapabilitty::new('z', 0.033026),
    LetterPrapabilitty::new('c', 0.029991),
    LetterPrapabilitty::new('y', 0.028575),
    LetterPrapabilitty::new('h', 0.024975),
    LetterPrapabilitty::new('j', 0.023058),
    LetterPrapabilitty::new('p', 0.031150),
    LetterPrapabilitty::new('b', 0.017761),
    LetterPrapabilitty::new('g', 0.002198),
    LetterPrapabilitty::new('f', 0.001751),
    LetterPrapabilitty::new('w', 0.000543),
    LetterPrapabilitty::new('x', 0.000359),
    LetterPrapabilitty::new('q', 0.000059),
];

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

mod vigener_cipher {
    use std::collections::HashMap;

    use crate::{read_line, transpose, ALPHABET, CZECH_PROPABILITTIES};

    pub fn cipher() -> std::io::Result<()> {
        println!("Zadejte text: ");

        println!("Zadejte klíč: ");


        Ok(())
    }

    pub fn decipher() -> std::io::Result<()> {
        println!("Zadejte zašifrovaný text: ");
        let st = read_line();
        println!("Zadejte délku klíče: ");
        let key_length = read_line()
            .parse::<usize>()
            .expect("nespravny format klice");

        let mut processed_characters: Vec<Vec<char>> = vec![];

        let mut key = String::new();

        let mut char_frequencies: HashMap<char, usize> = HashMap::new();
        for n in &each_of(st, key_length) {
            for item in n {
                *char_frequencies.entry(*item).or_insert(0) += 1;
            }

            let max_frequency = char_frequencies.iter().max_by_key(|z| z.1).unwrap();
            let best_match  = CZECH_PROPABILITTIES.iter().min_by_key(|p| 
                (p.propabilitty * (n.len() as f32) - char_frequencies.closest(p.propabilitty * (n.len() as f32)) as f32).round().abs() as i32).unwrap();
            
            println!("{}", best_match.letter);
            let shift = ALPHABET.iter().position(|c| c == max_frequency.0).unwrap()
                - ALPHABET
                    .iter()
                    .position(|c| *c == best_match.letter)
                    .unwrap();
            let mut shifted_alphabet = ALPHABET;
            shifted_alphabet.rotate_left(shift);

            processed_characters.push(
                n.iter()
                    .map(|x| {
                        ALPHABET
                            [shifted_alphabet.iter().position(|c| c == x).unwrap()]
                    })
                    .collect(),
            );

            key.push(ALPHABET[shift]);

            char_frequencies = HashMap::new();
        }
        println!("Klíčem je: {}", key);
        println!("Rozšifrovaný text: {}", combine_to_ot(processed_characters));
        Ok(())
    }

    fn each_of(text: String, number_of_segments: usize) -> Vec<Vec<char>> {
        let mut i = 0;
        let mut divided: Vec<Vec<char>> = vec![vec![]; number_of_segments];
        for n in text.chars() {
            divided[i].push(n);
            i += 1;
            if i == number_of_segments {
                i = 0;
            }
        }
        divided
    }

    fn combine_to_ot(sectors: Vec<Vec<char>>) -> String {
        let mut buffer = String::new();
        for i in transpose(sectors){
            buffer.push_str(&i.iter().collect::<String>());
        }
        buffer
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
    
}

fn main() -> Result<()> {
    loop {
        println!("Zašifrovat nebo rozšifrovat (z/r): ");
        match read_line().as_str() {
            "r" => vigener_cipher::decipher()?,
            "z" => vigener_cipher::cipher()?,
            "c" => break,
            _ => {}
        }
    }
    Ok(())
}
