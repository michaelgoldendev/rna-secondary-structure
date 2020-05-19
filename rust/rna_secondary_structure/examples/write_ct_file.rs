extern crate rna_secondary_structure;

use std::path::Path;

use rna_secondary_structure::*;
use rna_secondary_structure::secondary_structure_io::*;

fn main() {
    let ss: secondary_structure::SecondaryStructure = "((..)..)".parse().unwrap();
    let seq = "CGAACAAG".to_string();
    write_ct_file(Path::new("example.ct"), &seq, &ss, None);
    let title = "title".to_string();
    write_ct_file(Path::new("example_with_title.ct"), &seq, &ss, Some(&title));
}
