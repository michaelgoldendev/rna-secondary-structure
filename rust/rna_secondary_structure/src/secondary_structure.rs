use std::fmt;

pub struct SecondaryStructure {
    pub pairedsites: Vec<i32>
}

impl SecondaryStructure {
    /// Returns a dot bracket string representation of SecondaryStructure
    fn dotbracketstring(&self) -> String {
        // TODO: add mixed bracket types for ambiguous structures
        let mut dbs = String::with_capacity(self.pairedsites.len());
        for (i, j) in self.pairedsites.iter().enumerate() {
            if j == &(0 as i32) {
                dbs.push('.')
            } else if j > &(i as i32) {
                dbs.push('(')
            } else {
                dbs.push(')')
            }
        }
        dbs
    }
}

impl fmt::Display for SecondaryStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.dotbracketstring())
    }
}

#[cfg(test)]
#[path = "./secondary_structure_test.rs"]
mod secondary_structure_test;