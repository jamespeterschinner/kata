use std::env;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    io::stdout().write_all(roman(&args[1]).as_bytes())
}


fn encode((decimal_number, base, ): (char, usize, )) -> String {
    let digit = decimal_number.to_digit(10).unwrap();
    if base >= 6 {
        "M".repeat((10_u32.pow((base - 3) as u32) * digit) as usize)
    } else {
        let bases = ["I", "X", "C", "M", "X̄", "C̄", "M̄"];
        let center = ["V", "L", "D", "V̄", "L̄", "D̄"];


        if digit == 9 {
            format!("{}{}", bases[base], bases[base + 1])
        } else if digit >= 5 {
            format!("{}{}", center[base], bases[base].repeat((digit - 5) as usize))
        } else if digit == 4 {
            format!("{}{}", bases[base], center[base])
        } else {
            // Less than 4
            bases[base].repeat(digit as usize)
        }
    }
}

struct Bases {
    base: usize
}

impl Iterator for Bases {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.base = self.base - 1;
        Some(self.base)
    }
}

fn iter_bases(largest_base: usize) -> Bases {
    Bases {base: largest_base}
}

fn roman(decimal_encoded_string: &String) -> String {
    let result: Vec<String> = decimal_encoded_string
        .chars()
        .zip(iter_bases(decimal_encoded_string.len()))
        .map(encode)
        .collect();

    result.join("")
}