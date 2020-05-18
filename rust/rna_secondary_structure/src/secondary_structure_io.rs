use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::secondary_structure;
use crate::secondary_structure::SecondaryStructure;

/// Get a connect (CT) format string representation of a sequence and SecondaryStructure.
/// 
/// # Examples
/// 
/// ```
/// use crate::rna_secondary_structure::secondary_structure;
/// use crate::rna_secondary_structure::secondary_structure_io;
/// 
/// let ss : secondary_structure::SecondaryStructure = "((..)..)".parse().unwrap();
/// let seq = "CGAACAAG".parse().unwrap();
/// let ct_string_observed = secondary_structure_io::get_ct_string(&seq, &ss, &String::from("example"));
/// 
/// let ct_string_expected =
/// ">example
/// 1	C	0	2	8	1
/// 2	G	1	3	5	2
/// 3	A	2	4	0	3
/// 4	A	3	5	0	4
/// 5	C	4	6	2	5
/// 6	A	5	7	0	6
/// 7	A	6	8	0	7
/// 8	G	7	9	1	8
/// ";
///
/// assert_eq!(ct_string_observed, ct_string_expected);
/// ```
pub fn get_ct_string(seq: &String, ss: &secondary_structure::SecondaryStructure, title: &String) -> String {
    let it = seq.chars().zip(ss.pairedsites.iter());

    let mut data = format!(">{}\n", title);
    for (i, (c, j)) in it.enumerate() {
        data.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", i + 1, c, i, i + 2, j, i + 1));
    }
    data
}

pub fn parse_ct_string(ct_string: &String) -> Vec<i64> {
    let mut ls : Vec<Vec<i64>>  = Vec::new();
    let mut pairedsites = Vec::new();
    for line in ct_string.lines() {
        let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
        if spl.len() > 0 && spl[0].starts_with(">") {
            if pairedsites.len() > 0 {
                ls.push(pairedsites.clone());
                pairedsites.clear();
            }
        } else if spl.len() >= 6 && spl[0].parse::<i64>().is_ok() && spl[5].parse::<i64>().is_ok() {
            pairedsites.push(spl[4].parse::<i64>().unwrap());
        }
    }
    if pairedsites.len() > 0 {
        ls.push(pairedsites.clone());
        pairedsites.clear();
    }
    pairedsites
}

pub fn write_ct_file(path: &Path, seq: &String, ss: &secondary_structure::SecondaryStructure, title: Option<&String>) -> () {
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
    } else {
        let data = get_ct_string(seq, ss, &format!("{}", seq.len()));
        file.write_all(data.as_bytes()).expect("Write failed.");
    }
}