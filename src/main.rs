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
            //This pow function is the main limiter for decimal size
            .repeat((10_u32.pow((base - max_base) as u32) * digit) as usize)
    } else {
        if digit == 9 {
            format!("{}{}"
                    , if base == 3 { "Ī"} else { BASE10_NUMERALS[base] }
                    , BASE10_NUMERALS[base + 1]
            )
        } else if digit >= 5 {
            format!("{}{}", CENTRE_NUMERALS[
                base], BASE10_NUMERALS[base]
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
        writer.write(roman_numeral.as_bytes())
            .expect("Unable to write to stdout");
    }
}

#[cfg(test)]
mod tests {
    use crate::encode;

    #[test]
    fn basic_values() {
        assert_eq!(encode(('1', 0)), "I");
        assert_eq!(encode(('5', 0)), "V");
        assert_eq!(encode(('1', 1)), "X");
        assert_eq!(encode(('5', 1)), "L");
        assert_eq!(encode(('1', 2)), "C");
        assert_eq!(encode(('5', 2)), "D");
        assert_eq!(encode(('1', 3)), "M");
        assert_eq!(encode(('5', 3)), "V̄");
        assert_eq!(encode(('1', 4)), "X̄");
        assert_eq!(encode(('5', 4)), "L̄");
        assert_eq!(encode(('1', 5)), "C̄");
        assert_eq!(encode(('5', 5)), "D̄");
        assert_eq!(encode(('1', 6)), "M̄");
    }

    #[test]
    fn combinations() {
        assert_eq!(encode(('2', 0)), "II");
        assert_eq!(encode(('3', 0)), "III");
        assert_eq!(encode(('4', 0)), "IV");
        assert_eq!(encode(('6', 0)), "VI");
        assert_eq!(encode(('7', 0)), "VII");
        assert_eq!(encode(('8', 0)), "VIII");
        assert_eq!(encode(('9', 0)), "IX");

        assert_eq!(encode(('2', 1)), "XX");
        assert_eq!(encode(('3', 1)), "XXX");
        assert_eq!(encode(('4', 1)), "XL");
        assert_eq!(encode(('6', 1)), "LX");
        assert_eq!(encode(('7', 1)), "LXX");
        assert_eq!(encode(('8', 2)), "DCCC");
        assert_eq!(encode(('9', 2)), "CM");

        assert_eq!(encode(('9', 3)), "ĪX̄");
    }
}


