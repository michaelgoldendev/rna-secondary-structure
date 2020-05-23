use std::fmt;
use std::str;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecondaryStructureParseError {
    #[error("No matching closing bracket.")]
    MissingClosingBracket,

    #[error("No matching opening bracket.")]
    MissingOpeningBracket,
}

pub struct SecondaryStructureRecord {
    pub sequence: String,
    pub paired: Vec<i64>,
}

impl SecondaryStructureRecord {
    pub fn new(paired: Vec<i64>) -> SecondaryStructureRecord {
        SecondaryStructureRecord {
            sequence: "N".repeat(paired.len()),
            paired,
        }
    }

    /// Returns a dot bracket string representation of the secondary structure
    pub fn dotbracketstring(&self) -> String {
        // TODO: add mixed bracket types for ambiguous/pseudoknotted structures.
        let mut dbs = String::with_capacity(self.paired.len());
        for (i, j) in self.paired.iter().enumerate() {
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

/// Returns a vector of paired sites from a dot bracket string representation.
/// For usage see [FromStr for SecondaryStructure](struct.SecondaryStructureRecord.html#impl-FromStr).
pub fn from_dotbracketstring(s: &str) -> Result<Vec::<i64>, SecondaryStructureParseError> {
    let mut _paired = vec![0; s.len()];
    let mut stack = Vec::<i64>::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i as i64);
        } else if c == ')' {
            let j = stack.pop();
            match j {
                None => return Err(SecondaryStructureParseError::MissingOpeningBracket),
                Some(j) => {
                    _paired[i] = j + 1;
                    _paired[j as usize] = (i as i64) + 1;
                }
            }
        }
    }

    if stack.len() > 0 {
        return Err(SecondaryStructureParseError::MissingClosingBracket);
    }

    Ok(_paired)
}

impl fmt::Display for SecondaryStructureRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.sequence, self.dotbracketstring())
    }
}

/// Returns a SecondaryStructureRecord from a dot bracket string representation with a sequence of
/// the same length consisting of all N's.
/// 
/// # Examples
/// 
/// ```
/// use crate::rna_secondary_structure::secondary_structure::SecondaryStructureRecord;
/// let paired = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
/// let ss : SecondaryStructureRecord = "(((..))..)..".parse().unwrap();
/// assert_eq!(ss.paired, paired);
/// ```
impl str::FromStr for SecondaryStructureRecord {
    type Err = SecondaryStructureParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SecondaryStructureRecord::new(from_dotbracketstring(s)?))
    }
}

#[cfg(test)]
#[path = "./secondary_structure_test.rs"]
mod secondary_structure_test;