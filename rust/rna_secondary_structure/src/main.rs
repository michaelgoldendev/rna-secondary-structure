mod secondary_structure;

use std::str::FromStr;   

fn main() {
    let ss = secondary_structure::SecondaryStructure::from_str("()").unwrap();
    println!("{}", ss);

    let ss : secondary_structure::SecondaryStructure = "(..)".parse().unwrap();
    println!("{}", ss);
}
