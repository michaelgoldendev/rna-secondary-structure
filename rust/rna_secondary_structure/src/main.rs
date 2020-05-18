extern crate rna_secondary_structure;

use std::path::Path;
use std::str::FromStr;

use rna_secondary_structure::*;

fn main() {
    let ss = secondary_structure::SecondaryStructure::from_str("()").unwrap();
    println!("{}", ss);

    let ss: secondary_structure::SecondaryStructure = "(..)".parse().unwrap();
    println!("{}", ss);

    // let ss : secondary_structure::SecondaryStructure = "(".parse().unwrap();
    // println!("{}", ss);

    let ss: secondary_structure::SecondaryStructure = "((..)..)".parse().unwrap();
    secondary_structure_io::write_ct_file(Path::new("test.ct"), &"CGAACAAG".parse().unwrap(), &ss, None);
    println!("{}", ss);
}
