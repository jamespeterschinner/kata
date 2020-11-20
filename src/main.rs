use std::env;
use std::io;
use std::io::BufWriter;
use std::io::Write;

static BASE10_NUMERALS: [&str; 7] = ["I", "X", "C", "M", "X̄", "C̄", "M̄"];

static CENTRE_NUMERALS: [&str; 6] = ["V", "L", "D", "V̄", "L̄", "D̄"];

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
    Bases { base: largest_base }
}


fn encode((decimal_number, base, ): (char, usize, )) -> String {
    let digit = decimal_number.to_digit(10).unwrap();
    let max_base = CENTRE_NUMERALS.len();

    if base >= max_base {
        BASE10_NUMERALS[BASE10_NUMERALS.len() - 1]
            .repeat((10_u32.pow((base - max_base) as u32) * digit) as usize)
    } else {
        if digit == 9 {
            format!("{}{}", BASE10_NUMERALS[base], BASE10_NUMERALS[base + 1])
        } else if digit >= 5 {
            format!("{}{}", CENTRE_NUMERALS[base], BASE10_NUMERALS[base]
                .repeat((digit - 5) as usize))
        } else if digit == 4 {
            format!("{}{}", BASE10_NUMERALS[base], CENTRE_NUMERALS[base])
        } else {
            // Less than 4
            BASE10_NUMERALS[base].repeat(digit as usize)
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut writer = BufWriter::new(io::stdout());
    let decimal_encoded_string = &args[1];
    for roman_numeral in decimal_encoded_string
        .chars()
        .zip(iter_bases(decimal_encoded_string.len()))
        .map(encode) {
        writer.write(roman_numeral.as_bytes());
    }
}

