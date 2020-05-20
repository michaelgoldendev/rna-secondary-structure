use std::fmt;
use std::str;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecondaryStructureParseError {
    #[error("No matching closing bracket.")]
    MissingRightBracket,

    #[error("No matching opening bracket.")]
    MissingLeftBracket,
}

pub struct SecondaryStructureRecord {
    pub sequence: String,
    pub pairedsites: Vec<i64>,
}

impl SecondaryStructureRecord {
    pub fn new(pairedsites: Vec<i64>) -> SecondaryStructureRecord {
        SecondaryStructureRecord {
            sequence: "N".repeat(pairedsites.len()),
            pairedsites: pairedsites,
        }
    }

    /// Returns a dot bracket string representation of the secondary structure
    pub fn dotbracketstring(&self) -> String {
        // TODO: add mixed bracket types for ambiguous/pseudoknotted structures.
        let mut dbs = String::with_capacity(self.pairedsites.len());
        for (i, j) in self.pairedsites.iter().enumerate() {
            if j == &(0 as i64) {
                dbs.push('.')
            } else if j > &(i as i64) {
                dbs.push('(')
            } else {
                dbs.push(')')
            }
        }
        dbs
    }
}

/// Returns a SecondaryStructure from a dot bracket string representation.
/// For usage see [FromStr for SecondaryStructure](struct.SecondaryStructure.html#impl-FromStr).
pub fn from_dotbracketstring(s: &str) -> Result<Vec::<i64>, SecondaryStructureParseError> {
    let mut _pairedsites = vec![0; s.len()];
    let mut stack = Vec::<i64>::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i as i64);
        } else if c == ')' {
            let j = stack.pop();
            match j {
                None => return Err(SecondaryStructureParseError::MissingLeftBracket),
                Some(j) => {
                    _pairedsites[i] = j + 1;
                    _pairedsites[j as usize] = (i as i64) + 1;
                }
            }
        }
    }

    if stack.len() > 0 {
        return Err(SecondaryStructureParseError::MissingRightBracket);
    }

    Ok(_pairedsites)
}

impl fmt::Display for SecondaryStructureRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.sequence, self.dotbracketstring())
    }
}

/// Returns a SecondaryStructure from a dot bracket string representation.
/// 
/// # Examples
/// 
/// ```
/// use crate::rna_secondary_structure::secondary_structure::SecondaryStructureRecord;
/// let pairedsites = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
/// let ss : SecondaryStructureRecord = "(((..))..)..".parse().unwrap();
/// assert_eq!(ss.pairedsites, pairedsites);
/// ```
impl str::FromStr for SecondaryStructureRecord {
    type Err = SecondaryStructureParseError;
    // TODO: I don't think this error is ever raised because of panics.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = from_dotbracketstring(s);
        if res.is_ok() {
            return Ok(SecondaryStructureRecord::new(res.unwrap()));
        }
        Err(res.unwrap_err())
    }
}

#[cfg(test)]
#[path = "./secondary_structure_test.rs"]
mod secondary_structure_test;