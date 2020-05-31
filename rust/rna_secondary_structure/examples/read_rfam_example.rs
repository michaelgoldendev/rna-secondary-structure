extern crate rna_secondary_structure;

use std::fs::File;

use rna_secondary_structure::read_rfam::parse_rfam_stockholm_gz_file;
use rna_secondary_structure::secondary_structure::is_pseudoknotted;

fn main() {
    let records = parse_rfam_stockholm_gz_file(File::open("../../data/Rfam_14.2.seed.gz").unwrap()).unwrap();
    for record in records {
        println!("{}", record);
        println!("length: {}", record.paired.len());
        println!("is pseudoknotted: {}", is_pseudoknotted(&record).unwrap());
        println!("");
    }
}