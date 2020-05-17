use std::string::ParseError;
use std::fmt;
use std::str;

pub struct SecondaryStructure {
    pub pairedsites: Vec<i64>
}

impl SecondaryStructure {
    /// Returns a dot bracket string representation.
    fn dotbracketstring(&self) -> String {
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
pub fn from_dotbracketstring(s : &str) -> Result<SecondaryStructure, ParseError> {
    let mut _pairedsites = vec![0; s.len()];
    let mut stack = Vec::<i64>::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i as i64);
        }
        else if c == ')' {
            let j = stack.pop();
            match j {
                None => panic!("No matching bracket for ')'."),
                Some(j) => {
                    _pairedsites[i] = j+1;
                    _pairedsites[j as usize] = (i as i64) + 1;
                }
            }
        }
    }

    if stack.len() > 0 {
        panic!("No matching bracket for '('.")
    }

    let ss = SecondaryStructure {
        pairedsites: _pairedsites
    };
    Ok(ss)
}

impl fmt::Display for SecondaryStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.dotbracketstring())
    }
}

/// Returns a SecondaryStructure from a dot bracket string representation.
/// 
/// # Examples
/// 
/// ```
/// use crate::rna_secondary_structure::secondary_structure::SecondaryStructure;
/// let pairedsites = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
/// let ss : SecondaryStructure = "(((..))..)..".parse().unwrap();
/// assert_eq!(ss.pairedsites, pairedsites);
/// ```
impl str::FromStr for SecondaryStructure {
    type Err = ParseError; // TODO: I don't think this error is ever raised because of panics.
    fn  from_str(s: &str) -> Result<Self, Self::Err> {
        from_dotbracketstring(s)
    }
}

#[cfg(test)]
#[path = "./secondary_structure_test.rs"]
mod secondary_structure_test;