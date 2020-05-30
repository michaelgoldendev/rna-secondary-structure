extern crate rna_secondary_structure;

use std::path::Path;

use rna_secondary_structure::*;
use rna_secondary_structure::io::*;

fn main() {
    let ss: secondary_structure::SecondaryStructureRecord = "<((..)..).a>..A".parse().unwrap();
    write_ct_file(Path::new("example.ct"), &ss).unwrap();
    write_dbn_file(Path::new("example.dbn"), &ss).unwrap();
    let ss2: secondary_structure::SecondaryStructureRecord = "((((....))))".parse().unwrap();
    let ss3: secondary_structure::SecondaryStructureRecord = "((((..<<...)))).zz..>>...ZZ".parse().unwrap();

    let mut ls = Vec::new();
    ls.push(&ss);
    ls.push(&ss2);
    ls.push(&ss3);
    write_records_to_ct_file(Path::new("multiple.ct"), &ls).unwrap();
    write_records_to_dbn_file(Path::new("multiple.dbn"), &ls).unwrap();
    println!("{}", &ss3)
}
