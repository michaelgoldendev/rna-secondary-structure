extern crate rna_secondary_structure;

use std::path::Path;

use rna_secondary_structure::*;
use rna_secondary_structure::io::*;

fn main() {
    let ss: secondary_structure::SecondaryStructureRecord = "((..)..)".parse().unwrap();
    write_ct_file(Path::new("example.ct"), &ss).unwrap();
}
