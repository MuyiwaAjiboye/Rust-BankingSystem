use rand::prelude::*;
use std::fmt::Display;
use std::str::FromStr;

/// The account number uses random number generation
/// for the payload. This payload is then calculated
/// to produce the check digit.
#[derive(Debug)]
pub struct AccountNumber {
    /// Randomly generated number. Each digit is within 0..9
    pub(crate) payload: Vec<u8>,
    /// Check digit using the Luhn formula
    pub(crate) check_digit: u8,
}

//Defines a Default length for an account number 
impl Default for AccountNumber {
    fn default() -> Self {
        Self::new(10)
    }
}


//using the verify function so that any invalid account number is rejected
impl FromStr for AccountNumber {
    type Err = std::io::ErrorKind;

    fn from_str(s: &str) -> Result<Self, std::io::ErrorKind> {
        if verify(s) {
            let mut payload: Vec<u8> = s
                .chars()
                .map(|d| {
                    d.to_digit(10)
                        .expect("Not able to create u32 from character") as u8
                })
                .collect();
            let check_digit = payload.pop().expect("Cannot pop. Got an empty vector");
            Ok(Self {
                payload,
                check_digit,
            })
        } else {
            eprintln!("{s} is not a valid account number");
            Err(std::io::ErrorKind::InvalidData)
        }
    }
}