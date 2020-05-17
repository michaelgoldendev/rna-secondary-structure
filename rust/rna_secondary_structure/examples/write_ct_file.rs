extern crate rna_secondary_structure;
use rna_secondary_structure::*;
use rna_secondary_structure::secondary_structure_io::*;

use std::path::Path;


fn main() {
    let ss : secondary_structure::SecondaryStructure = "((..)..)".parse().unwrap();
    let seq = "CGAACAAG".parse().unwrap();
    write_ct_file(Path::new("example.ct"), &seq, &ss, None);
    write_ct_file(Path::new("example_with_title.ct"), &seq, &ss, Some(&String::from("title")));
}
