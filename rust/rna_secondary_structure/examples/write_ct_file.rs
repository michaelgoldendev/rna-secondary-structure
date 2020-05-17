extern crate rna_secondary_structure;
use rna_secondary_structure::*;

use std::path::Path;


fn main() {
    let ss : secondary_structure::SecondaryStructure = "((..)..)".parse().unwrap();
    secondary_structure_io::write_ct_file(Path::new("example.ct"), &"CGAACAAG".parse().unwrap(), &ss, None);
}
