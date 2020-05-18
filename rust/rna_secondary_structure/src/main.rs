extern crate rna_secondary_structure;

use std::path::Path;
use std::str::FromStr;

use rna_secondary_structure::*;

fn main() {
    let ct_string_expected = ">example
    1	C	0	2	8	1
    2	G	1	3	5	2
    3	A	2	4	0	3
    4	A	3	5	0	4
    5	C	4	6	2	5
    6	A	5	7	0	6
    7	A	6	8	0	7
    8	G	7	9	1	8";
    secondary_structure_io::parse_ct_string(&ct_string_expected.parse().unwrap());

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
