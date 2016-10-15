use std::str::FromStr;

pub struct Roman(u16);

const SYMBOLS: [(u16, &'static str); 13] = [(1, "I"),
                                            (4, "IV"),
                                            (5, "V"),
                                            (9, "IX"),
                                            (10, "X"),
                                            (40, "XL"),
                                            (50, "L"),
                                            (90, "XC"),
                                            (100, "C"),
                                            (400, "CD"),
                                            (500, "D"),
                                            (900, "CM"),
                                            (1000, "M")];

impl Roman {
    pub fn from(n: u16) -> Roman {
        Roman(n)
    }

    pub fn to_string(&self) -> String {
        let mut roman = String::new();
        let decomposed = self.decompose(self.0);
        for digit in decomposed {
            roman.push_str(&self.convert(digit));
        }

        roman
    }

    fn convert(&self, n: u16) -> String {
        let mut roman = String::new();

        return match n {
            0 => roman,
            _ => {
                let main_symbol = self.find_main_symbol(n);
                roman.push_str(main_symbol.1);
                roman.push_str(&self.convert(n - main_symbol.0));
                roman
            }
        };
    }

    fn decompose(&self, n: u16) -> Vec<u16> {
        let mut decomposition: Vec<u16> = Vec::new();
        let n_string = n.to_string();

        // If n is too big panic!
        for (i, c) in n_string.chars().enumerate() {
            decomposition.push((u16::from_str(&c.to_string()).unwrap() *
                                10u16.pow((n_string.len() - (i + 1)) as u32)));
        }

        decomposition
    }

    fn find_main_symbol(&self, n: u16) -> (u16, &'static str) {
        ensure_is_convertible(n);

        let mut symbol = SYMBOLS[0];

        for i in SYMBOLS.iter() {
            if (n / i.0) == 0 {
                break;
            }

            symbol = *i;
        }

        symbol
    }
}

fn ensure_is_convertible(n: u16) -> bool {
    if n > 3000 {
        panic!("only number lower than `3000` are convertible");
    }

    true
}

#[cfg(test)]
mod tests {
    use super::Roman;

    #[test]
    fn test_decompose() {
        let fake_roman = &Roman(110);
        assert_eq!(vec![90, 3], Roman::decompose(fake_roman, 93));
        assert_eq!(vec![1000, 0, 20, 4], Roman::decompose(fake_roman, 1024));
    }

    #[test]
    fn test_find_main_symbol() {
        let fake_roman = &Roman(110);
        assert_eq!("C", Roman::find_main_symbol(fake_roman, 110).1);
        assert_eq!("L", Roman::find_main_symbol(fake_roman, 80).1);
        assert_eq!("X", Roman::find_main_symbol(fake_roman, 10).1);
        assert_eq!("X", Roman::find_main_symbol(fake_roman, 11).1);
        assert_eq!("XC", Roman::find_main_symbol(fake_roman, 90).1);
        assert_eq!("CD", Roman::find_main_symbol(fake_roman, 400).1);
    }
}
