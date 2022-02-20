use crate::sign::Sign;
use crate::RealNumber;

impl RealNumber {
    fn parse_parts(val: Vec<char>) -> Option<(Vec<char>, Vec<char>)> {
        let is_valid_val = val
            .iter()
            .fold(true, |acc, chr| acc && (chr.is_digit(10) || *chr == '.'));

        if !is_valid_val {
            return None;
        }

        let parts = val.split(|chr| *chr == '.').collect::<Vec<_>>();

        match parts.len() {
            1 => Some((parts.first()?.to_vec(), Vec::new())),
            2 => Some((parts.first()?.to_vec(), parts.last()?.to_vec())),
            _ => None,
        }
    }

    fn parse_int_part(int_part: Vec<char>) -> Option<Vec<char>> {
        if int_part.len() == 0 {
            return None;
        }

        if int_part[0] == '0' && int_part.len() > 1 {
            return None;
        }

        Some(int_part)
    }

    fn parse_frac_part(frac_part: Vec<char>) -> Option<Vec<char>> {
        let mut vec = Vec::new();
        let mut is_insignificant_zeros = true;

        for &chr in frac_part.iter().rev() {
            if chr == '0' && is_insignificant_zeros {
                continue;
            }

            is_insignificant_zeros = false;
            vec.push(chr)
        }

        if vec.len() == 0 {
            vec.push('0');
        }

        vec.reverse();
        Some(vec)
    }

    pub fn from_unsigned_numeric_string(val: String) -> Option<Self> {
        let val = val.chars().collect::<Vec<_>>();
        let (int_part, frac_part) = Self::parse_parts(val)?;
        let int_part = Self::parse_int_part(int_part)?;
        let frac_part = Self::parse_frac_part(frac_part)?;

        Some(Self::new(int_part, frac_part, Sign::Positive))
    }
}
