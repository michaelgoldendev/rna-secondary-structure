//! A module for representing secondary structures.

use std::fmt;
use std::str;

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum SecondaryStructureParseError {
    #[error("Missing left parentheses '{left}' for '{right}' at position {pos}")]
    MissingLeftParentheses {
        left: char,
        right: char,
        pos: usize,
    },

    #[error("Missing right parentheses '{right}' for '{left}' at position {pos}")]
    MissingRightParentheses {
        left: char,
        right: char,
        pos: usize,
    },

    #[error("Bracket type not recognised: '{c}'")]
    BracketTypeNotRecognised {
        c: char
    },

    #[error("Insufficient bracket types are available for parsing structure unambigously.")]
    InsufficientBracketTypes,
}

/// A string of characters representing possible left bracket types
pub const LEFT_BRACKETS: &str = "(<{[abcdefghijklmnopqrstuvwxyz";
/// A string of characters representing corresponding right bracket types
pub const RIGHT_BRACKETS: &str = ")>}]ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn is_left_bracket(brace: char) -> bool {
    LEFT_BRACKETS.contains(brace)
}

fn is_right_bracket(brace: char) -> bool {
    RIGHT_BRACKETS.contains(brace)
}

/// ```rust
/// #[test]
/// assert_eq!(self::get_matching_bracket('<').unwrap(), '>');
/// #[test]
/// assert_eq!(self::get_matching_bracket('Z').unwrap(), 'z');
/// ```
pub fn get_matching_bracket(brace: char) -> Result<char, SecondaryStructureParseError> {
    let left_pos = LEFT_BRACKETS.find(brace).unwrap_or(1000);
    if left_pos != 1000 {
        return Ok(RIGHT_BRACKETS.chars().nth(left_pos).unwrap());
    }

    let right_pos = RIGHT_BRACKETS.find(brace).unwrap_or(1000);
    if right_pos != 1000 {
        return Ok(LEFT_BRACKETS.chars().nth(right_pos).unwrap());
    }

    return Err(SecondaryStructureParseError::BracketTypeNotRecognised {
        c: brace
    });
}

/// A struct represent a secondary structure and it's corresponding nucleotide sequence.
pub struct SecondaryStructureRecord {
    /// A name for this record.
    pub name: String,

    /// A String representing the secondary structures nucleotide sequence.
    pub sequence: String,

    /// A vector of paired sites.
    pub paired: Vec<i64>,
}

impl SecondaryStructureRecord {
    /// Constructs a SecondaryStructureRecord from a list of paired sites representing the secondary
    /// structure configuration. With a default sequence of the same length consisting of all N's.
    pub fn new(paired: Vec<i64>) -> SecondaryStructureRecord {
        SecondaryStructureRecord {
            name: "".to_string(),
            sequence: "N".repeat(paired.len()),
            paired,
        }
    }

    /// Set the nucleotide sequence
    pub fn set_sequence(&mut self, sequence: String) -> () {
        self.sequence = sequence;
    }

    /// Set the secondary structure configuration
    pub fn set_paired(&mut self, paired: Vec<i64>) -> () {
        self.paired = paired;
    }

    /// Returns a dot bracket string representation of the secondary structure configuration.
    pub fn get_dotbracketstring(&self) -> String {
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
pub fn from_dotbracketstring(dbs: &str) -> Result<Vec::<i64>, SecondaryStructureParseError> {
    let mut _paired = vec![0; dbs.len()];
    let mut stacks: Vec<Vec<i64>> = Vec::new();
    stacks.push(Vec::new());

    for (i, c) in dbs.chars().enumerate() {
        if is_left_bracket(c) {
            let index = LEFT_BRACKETS.find(c).unwrap();
            while stacks.len() <= index {
                stacks.push(Vec::new()); // add more stacks if additional bracket types are used.
            }
            stacks.get_mut(index).unwrap().push(i as i64);
        } else if is_right_bracket(c) {
            let index = RIGHT_BRACKETS.find(c).unwrap();
            if stacks.get(index).unwrap().len() == 0 {
                return Err(
                    SecondaryStructureParseError::MissingLeftParentheses {
                        left: get_matching_bracket(c)?,
                        right: c,
                        pos: i + 1,
                    });
            } else {
                let j = stacks.get_mut(index).unwrap().pop().unwrap();
                _paired[i] = j + 1;
                _paired[j as usize] = (i as i64) + 1;
            }
        }
    }

    /*if stack.len() > 0 {
        let j = stack.pop().unwrap() as usize;
        let c = dbs.chars().nth(j).unwrap();
        return Err(SecondaryStructureParseError::MissingRightParentheses {
            left: c,
            right: get_matching_bracket(c)?,
            pos: j + 1,
        });
    }*/

    Ok(_paired)
}

impl fmt::Display for SecondaryStructureRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.sequence, self.get_dotbracketstring())
    }
}

/// Returns a SecondaryStructureRecord from a dot bracket string representation with a sequence of
/// the same length consisting of all N's.
/// 
/// # Examples
/// 
/// ```rust
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