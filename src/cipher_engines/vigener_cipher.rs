
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

        let mut char_frequencies: HashMap<char, f64> = HashMap::new();
        for n in &each_of(st, key_length) {
            n.iter().for_each(|item| {
                *char_frequencies.entry(*item).or_insert(0.0) += 1.0 / n.len() as f64
            });

            let mut posible_keys: Vec<(char, char)> = vec![];

            for item in &char_frequencies {
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
                    posible_keys.push((*item.0, closest.letter));
                    let second_closest = if idx+1 >= 26 {0} else {idx+1};
                    posible_keys.push((*item.0, CZECH_PROPABILITTIES[second_closest].letter));
                    let third_closest = if idx+2 >= 26 {0} else {idx+2};
                    posible_keys.push((*item.0, CZECH_PROPABILITTIES[third_closest].letter));
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
        for i in transpose(sectors) {
            buffer.push_str(&i.iter().collect::<String>());
        }
        buffer
    }