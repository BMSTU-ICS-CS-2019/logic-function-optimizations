use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Term(pub u128);

impl Term {
    pub fn from_string(string: &String, size: usize) -> Result<Self, MintermParseError> {
        if size > u128::BITS as usize {
            return Err(MintermParseError::TooBig);
        }

        let string_length = string.len();
        if string_length != size {
            return Err(MintermParseError::InvalidStringLength(string_length));
        }

        let mut characters = string.chars();
        if let Some(character) = characters.next() {
            let mut value;
            match character {
                '0' => value = 0u128,
                '1' => value = 1u128,
                character => return Err(MintermParseError::InvalidCharacter(character)),
            }

            for character in characters {
                value <<= 1;
                match character {
                    '0' => {}
                    '1' => value |= 1,
                    character => return Err(MintermParseError::InvalidCharacter(character)),
                }
            }

            Ok(Self(value))
        } else {
            Ok(Self(0))
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:b}", self.0)
    }
}

#[derive(Debug)]
pub enum MintermParseError {
    TooBig,
    InvalidStringLength(usize),
    InvalidCharacter(char),
}
