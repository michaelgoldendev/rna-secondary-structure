use std::fmt;

pub struct SecondaryStructure {
    pub pairedsites: Vec<i64>
}

impl SecondaryStructure {
    /// Returns a dot bracket string representation of SecondaryStructure
    fn dotbracketstring(&self) -> String {
        // TODO: add mixed bracket types for ambiguous structures
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

pub fn from_dotbracketstring(s : &String) -> SecondaryStructure {
    let mut _pairedsites = vec![0; s.len()];
    let mut stack = Vec::<i64>::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i as i64);
        }
        else if c == ')' {
            let j = stack.pop();
            match j {
                None => {},
                Some(j) => {
                    _pairedsites[i] = j+1;
                    _pairedsites[j as usize] = (i as i64) + 1;
                }
            }
        }
    }

    let ss = SecondaryStructure {
        pairedsites: _pairedsites
    };
    ss
}

impl fmt::Display for SecondaryStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.dotbracketstring())
    }
}

#[cfg(test)]
#[path = "./secondary_structure_test.rs"]
mod secondary_structure_test;