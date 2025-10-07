use std::io::{Result, stdin};
struct LetterPrapabilitty {
    letter: char,
    propabilitty: f64,
}

impl LetterPrapabilitty {
    const fn new(letter: char, propabilitty: f64) -> LetterPrapabilitty {
        LetterPrapabilitty {
            letter: letter,
            propabilitty: propabilitty,
        }
    }
}

impl Clone for LetterPrapabilitty {
    fn clone(&self) -> Self {
        LetterPrapabilitty {
            letter: self.letter,
            propabilitty: self.propabilitty,
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
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    use crate::{ALPHABET, CZECH_PROPABILITTIES, read_line, transpose};

    pub fn cipher() -> std::io::Result<()> {
        println!("Zadejte text: ");
        let ot = read_line();
        println!("Zadejte klíč: ");
        let key = read_line();
        let mut shifted_alphabets: Vec<Vec<char>> = vec![];
        for item in key.chars() {
            let mut a = ALPHABET.to_vec();
            a.rotate_left(ALPHABET.iter().position(|c| *c == item).unwrap());
            shifted_alphabets.push(a);
        }
        let mut st = String::new();
        let mut i = 0;
        for item in ot.chars(){
            st.push(shifted_alphabets[i][ALPHABET.iter().position(|c| *c == item).unwrap()]);
            i += 1;
            if i == key.len() {
                i = 0;
            }
        }
        println!("Zašifrovaný text: {}", st);
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

        let mut char_frequencies: BTreeMap<char, i32> = BTreeMap::new();
        for n in &each_of(st, key_length) {
            n.iter().for_each(|item| {
                *char_frequencies.entry(*item).or_insert(0) += 1
            });
            let mut char_frequencies_sorted: Vec<(&char, &i32)> = char_frequencies.iter().collect();
            char_frequencies_sorted.sort_by(|a, b| b.1.cmp(a.1));
            let char_relative_frequencies_sorted: Vec<(char,f64)> = char_frequencies_sorted.iter().map(|item| (*item.0,*item.1 as f64/n.len() as f64)).collect();

            let mut posible_keys: Vec<(char, char)> = vec![];

            for item in &char_relative_frequencies_sorted {
                if let Some((idx, closest)) =
                    CZECH_PROPABILITTIES.to_vec()
                        .iter()
                        .enumerate()
                        .min_by(|(_, a_val), (_, b_val)| {
                            (a_val.propabilitty - item.1)
                                .abs()
                                .partial_cmp(&(b_val.propabilitty - item.1).abs())
                                .unwrap()
                        })
                {
                    posible_keys.push((item.0, closest.letter));
                    let second_closest = if idx+1 >= 26 {0} else {idx+1};
                    posible_keys.push((item.0, CZECH_PROPABILITTIES[second_closest].letter));
                    let third_closest = if idx+2 >= 26 {0} else {idx+2};
                    posible_keys.push((item.0, CZECH_PROPABILITTIES[third_closest].letter));
                }
            }

            let mut shifts = HashMap::new();
            for item in posible_keys{
                let first_position = ALPHABET.iter().position(|c| *c == item.0).unwrap();
                let second_position = ALPHABET.iter().position(|c| *c == item.1).unwrap();
                let mut shift_key = first_position.abs_diff(second_position);
                if first_position < second_position{
                    shift_key = 26 - shift_key;
                }
                *shifts.entry(shift_key).or_insert(0) += 1
            }

            let shift = shifts
                .iter()
                .max_by_key(|e| e.1)
                .map(|(&ch, _)| ch)
                .unwrap();

            let mut shifted_alphabet = ALPHABET;
            shifted_alphabet.rotate_left(shift);

            processed_characters.push(
                n.iter().map(|x| ALPHABET[shifted_alphabet.iter().position(|c| c == x).unwrap()]).collect(),
            );

            key.push(ALPHABET[shift]);

            char_frequencies = BTreeMap::new();
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
        for i in transpose(sectors) {
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