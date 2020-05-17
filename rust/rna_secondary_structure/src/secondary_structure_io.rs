use crate::secondary_structure;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn get_ct_string(seq : &String, ss : &secondary_structure::SecondaryStructure, title : &String) -> String {
    let it = seq.chars().zip(ss.pairedsites.iter());

    let mut data = format!(">{}\n", title);
    for (i, (c, j)) in it.enumerate() {
        data.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", i+1, c, i, i+2, j, i+1));
    }
    data
}

pub fn write_ct_file(path : &Path, seq : &String, ss : &secondary_structure::SecondaryStructure, title : Option<&String>) -> () {
    let append = false;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&path)
        .expect("Cannot open file");

    if let Some(x) = title {
        let data = get_ct_string(seq, ss, x);
        file.write_all(data.as_bytes()).expect("Write failed.");
    }
    else {
        let data = get_ct_string(seq, ss, &format!("{}", seq.len()));
        file.write_all(data.as_bytes()).expect("Write failed.");
    }
}