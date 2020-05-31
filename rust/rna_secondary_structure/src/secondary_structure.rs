//! A module for representing secondary structures.

use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str;

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum StructureParseError {
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

    #[error("All paired site(s) to the left have already been consumed.")]
    InputConsumed,

    #[error("Paired site(s) to the left have not been consumed.")]
    InputNotConsumed,

    #[error("{msg}")]
    ExpectedLine {
        msg: String
    },
}

/// A string of characters representing possible left bracket types
pub const LEFT_BRACKETS: &str = "(<{[ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// A string of characters representing corresponding right bracket types
pub const RIGHT_BRACKETS: &str = ")>}]abcdefghijklmnopqrstuvwxyz";

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
pub fn get_matching_bracket(brace: char) -> Result<char, StructureParseError> {
    let left_pos = LEFT_BRACKETS.find(brace).unwrap_or(1000);
    if left_pos != 1000 {
        return Ok(RIGHT_BRACKETS.chars().nth(left_pos).unwrap());
    }

    let right_pos = RIGHT_BRACKETS.find(brace).unwrap_or(1000);
    if right_pos != 1000 {
        return Ok(LEFT_BRACKETS.chars().nth(right_pos).unwrap());
    }

    Err(StructureParseError::BracketTypeNotRecognised { c: brace })
}

/// A struct containing the name, nucleotide sequence, and secondary structure conformation of
/// a secondary structure.
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
    /// structure conformation. With a default sequence of the same length consisting of all N's.
    pub fn new(paired: Vec<i64>) -> SecondaryStructureRecord {
        SecondaryStructureRecord {
            name: "".to_string(),
            sequence: "N".repeat(paired.len()),
            paired,
        }
    }

    /// Set the nucleotide sequence.
    pub fn set_sequence(&mut self, sequence: String) {
        self.sequence = sequence;
    }

    /// Set the secondary structure conformation from a paired sites representation.
    pub fn set_paired(&mut self, paired: Vec<i64>) {
        self.paired = paired;
    }

    /// Get a dot bracket string representation of the secondary structure conformation.
    pub fn get_dot_bracket_string(&self) -> Result<String, StructureParseError> {
        get_dot_bracket_string(self)
    }
}

/// A trait indicating that a struct can be converted to a vector representing a
/// list of base-paired and unpaired sites.
pub trait PairedSites {
    /// Returns a reference to a list of base-paired and unpaired sites representing the
    /// conformation of an arbitrarily pseudoknotted secondary structure.
    fn paired(&self) -> &Vec<i64>;
}

impl PartialEq<dyn PairedSites> for dyn PairedSites {
    fn eq(&self, other: &dyn PairedSites) -> bool {
        self.paired() == other.paired()
    }
}

impl PairedSites for SecondaryStructureRecord {
    fn paired(&self) -> &Vec<i64> {
        &self.paired
    }
}

impl Debug for SecondaryStructureRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.paired.fmt(f)
    }
}

impl PairedSites for Vec<i64> {
    fn paired(&self) -> &Vec<i64> {
        &self
    }
}

/// Returns a vector of paired sites from a dot bracket string representation.
/// For usage see [FromStr for SecondaryStructure](struct.SecondaryStructureRecord.html#impl-FromStr).
pub fn from_dotbracketstring(dbs: &str) -> Result<Vec<i64>, StructureParseError> {
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
            if !(stacks.get(index).unwrap().is_empty()) {
                let j = stacks.get_mut(index).unwrap().pop().unwrap();
                _paired[i] = j + 1;
                _paired[j as usize] = (i as i64) + 1;
            } else {
                return Err(
                    StructureParseError::MissingLeftParentheses {
                        left: get_matching_bracket(c)?,
                        right: c,
                        pos: i + 1,
                    });
            }
        }
    }

    for stack in stacks.iter() {
        if !stack.is_empty() {
            let j = *stack.last().unwrap() as usize;
            let c = dbs.chars().nth(j).unwrap();
            return Err(StructureParseError::MissingRightParentheses {
                left: c,
                right: get_matching_bracket(c)?,
                pos: j + 1,
            });
        }
    }
    Ok(_paired)
}

impl fmt::Display for SecondaryStructureRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ">{}\n{}\n{}", self.name, self.sequence, get_dot_bracket_string(self).unwrap())
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
    type Err = StructureParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SecondaryStructureRecord::new(from_dotbracketstring(s)?))
    }
}


/// Converts a paired sites list representing an arbitarily pseudoknotted secondary structure into
/// a dot bracket string representation.
///
/// # Examples
///
/// ```rust
/// use rna_secondary_structure::secondary_structure::get_dot_bracket_string;
/// let paired = vec![5, 7, 6, 9, 1, 3, 2, 10, 4, 8, 0, 0];
/// let dbs_observed = get_dot_bracket_string(&paired).unwrap();
/// let dbs_expected = "(<<{)>>(})..";
/// assert_eq!(dbs_observed, dbs_expected);
/// ```
pub fn get_dot_bracket_string(paired: &dyn PairedSites) -> Result<String, StructureParseError> {
    let paired = paired.paired();

    let mut stacks: Vec<Vec<i64>> = Vec::new();
    stacks.push(Vec::new());

    let mut dbn = "".to_string();
    for (i, j) in paired.iter().enumerate() {
        let i = i as i64;
        let j = *j;
        if j == 0 {
            dbn += ".";
        } else if i < j {
            let mut success = false;
            for (index, left) in LEFT_BRACKETS.chars().enumerate() {
                if index >= stacks.len() {
                    stacks.push(Vec::new()); // add a new stack for an additional bracket type
                }
                let stack = stacks.get(index).unwrap();
                if stack.is_empty() || j < *stack.last().unwrap() {
                    stacks.get_mut(index).unwrap().push(j);
                    dbn += &left.to_string();
                    success = true;
                    break;
                }
            }
            if !success {
                return Err(StructureParseError::InsufficientBracketTypes);
            }
        } else {
            let left = dbn.chars().nth((j - 1) as usize).unwrap();
            let index = LEFT_BRACKETS.find(left).unwrap();
            stacks.get_mut(index).unwrap().pop();

            let right = get_matching_bracket(left).unwrap();
            dbn.push_str(&right.to_string());
        }
    }
    Ok(dbn)
}

/// Returns true if the given secondary structure is pseudoknotted, false otherwise.
///
/// # Examples
/// ```rust
/// use rna_secondary_structure::secondary_structure::{from_dotbracketstring, is_pseudoknotted};
/// let non_pseudoknotted = from_dotbracketstring("<<<..<<<.<..>>.>..>..>...<<...>..>>.>").unwrap();
/// assert_eq!(is_pseudoknotted(&non_pseudoknotted).unwrap(), false);
/// let pseudoknotted = from_dotbracketstring("<<<..((.>>>....))").unwrap();
/// assert_eq!(is_pseudoknotted(&pseudoknotted).unwrap(), true);
/// let pseudoknotted2 = from_dotbracketstring("A..<<<..a...>>>....").unwrap();
/// assert_eq!(is_pseudoknotted(&pseudoknotted2).unwrap(), true);
/// ```
pub fn is_pseudoknotted(paired: &dyn PairedSites) -> Result<bool, StructureParseError> {
    let paired = paired.paired();

    let mut stack: Vec<i64> = Vec::new();

    for (i, j) in paired.iter().enumerate() {
        let i = i as i64;
        let j = *j;
        if j == 0 {} else if i < j {
            if !stack.is_empty() && j >= *stack.last().unwrap() {
                return Ok(true);
            } else {
                stack.push(j);
            }
        } else if !stack.is_empty() {
            stack.pop();
        } else {
            return Err(StructureParseError::InputConsumed);
        }
    }

    if !stack.is_empty() {
        return Err(StructureParseError::InputNotConsumed);
    }


    Ok(false)
}