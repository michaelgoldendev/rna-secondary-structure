use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::secondary_structure;
use crate::secondary_structure::SecondaryStructureRecord;

/// Get a connect (CT) format string representation of a sequence and SecondaryStructure.
/// 
/// # Examples
/// 
/// ```
/// use crate::rna_secondary_structure::secondary_structure;
/// use crate::rna_secondary_structure::io;
/// 
/// let ss : secondary_structure::SecondaryStructureRecord = "((..)..)".parse().unwrap();
/// let seq = "CGAACAAG".to_string();
/// let title = "example".to_string();
/// let ct_string_observed = io::get_ct_string(&seq, &ss.paired, &title);
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
pub fn get_ct_string(seq: &String, paired: &Vec<i64>, title: &String) -> String {
    let it = seq.chars().zip(paired.iter());

    let mut data = format!(">{}\n", title);
    for (i, (c, j)) in it.enumerate() {
        data.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", i + 1, c, i, i + 2, j, i + 1));
    }
    data
}

/// Reads a connect (CT) format string and returns a vector of Secondary Structures
///
/// # Examples
///
/// ```
/// use crate::rna_secondary_structure::io;
///
/// let ct_string =
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
/// let ls = io::parse_ct_string(&ct_string.to_string());
/// let observed_ss = &ls[0];
/// let seq = "CGAACAAG";
/// let paired = vec![8, 5, 0, 0, 2, 0, 0, 1];
/// assert_eq!(observed_ss.sequence, seq);
/// assert_eq!(observed_ss.paired, paired);
/// ```
pub fn parse_ct_string(ct_string: &String) -> Vec<SecondaryStructureRecord> {
    let mut ls: Vec<SecondaryStructureRecord> = Vec::new();
    let mut sequence = "".to_string();
    let mut paired = Vec::new();
    for line in ct_string.lines() {
        let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
        if spl.len() > 0 && spl[0].starts_with(">") {
            if paired.len() > 0 {
                ls.push(SecondaryStructureRecord {
                    sequence: sequence.to_string(),
                    paired: paired.clone(),
                });
                sequence = "".to_string();
                paired.clear();
            }
        } else if spl.len() >= 6 && spl[0].parse::<i64>().is_ok() && spl[5].parse::<i64>().is_ok() {
            sequence.push_str(spl[1]);
            paired.push(spl[4].parse::<i64>().unwrap());
        }
    }
    if paired.len() > 0 {
        ls.push(SecondaryStructureRecord {
            sequence: sequence.to_string(),
            paired: paired.clone(),
        });
        paired.clear();
    }
    ls
}

pub fn write_ct_file(path: &Path, ss: &secondary_structure::SecondaryStructureRecord, title: Option<&String>) -> () {
    let append = false;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&path)
        .map_err(|err| println!("{:?}", err)).unwrap();

    if let Some(x) = title {
        let data = get_ct_string(&ss.sequence, &ss.paired, x);
        file.write_all(data.as_bytes()).map_err(|err| println!("{:?}", err)).ok();
    } else {
        let data = get_ct_string(&ss.sequence, &ss.paired, &format!("{}", &ss.sequence.len()));
        file.write_all(data.as_bytes()).map_err(|err| println!("{:?}", err)).ok();
    }
}