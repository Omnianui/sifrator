use std::io::{Result, stdin};
mod cipher_engines;

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
            "r" => cipher_engines::vigener_cipher::decipher()?,
            "z" => cipher_engines::vigener_cipher::cipher()?,
            "c" => break,
            _ => {}
        }
    }
    Ok(())
}